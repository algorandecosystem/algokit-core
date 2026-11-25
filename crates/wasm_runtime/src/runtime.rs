//! The sandbox itself (feature `runtime`).

use alloc::string::ToString;
use zeroize::Zeroize;

use crate::error::{Error, Result};
use crate::ProgramId;
use wasmi::core::TrapCode;
use wasmi::{
    CompilationMode, Config, Engine, Linker, Module, StackLimits, Store, StoreLimits,
    StoreLimitsBuilder,
};

/// Engine-wide settings, fixed for every program of a [`WasmRuntime`].
#[derive(Debug, Clone, Copy)]
pub struct RuntimeConfig {
    /// Maximum value-stack height, in 8-byte slots.
    pub max_value_stack_height: usize,
    /// Maximum call recursion depth.
    pub max_recursion_depth: usize,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        RuntimeConfig {
            max_value_stack_height: 1 << 16,
            max_recursion_depth: 1024,
        }
    }
}

/// Per-instance limits.
#[derive(Debug, Clone, Copy)]
pub struct Limits {
    /// Fuel for one [`Instance::call`] (roughly one unit per instruction).
    pub fuel: u64,
    /// Cap on the instance's linear memory, in bytes.
    pub memory_limit_bytes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            fuel: 10_000_000,
            memory_limit_bytes: 1 << 20,
        }
    }
}

/// What a [`wasmi::Store`] holds: the embedder's per-call data `T` plus the
/// memory limiter. Host functions reach `T` via
/// `caller.data()`/`caller.data_mut()` and [`HostState::data`].
pub struct HostState<T> {
    data: T,
    limits: StoreLimits,
}

impl<T> HostState<T> {
    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

/// A validated module, ready to instantiate.
pub struct CompiledProgram {
    id: ProgramId,
    module: Module,
}

impl CompiledProgram {
    pub fn id(&self) -> ProgramId {
        self.id
    }

    /// Whether the module exports a function `name` of type `() -> i32`,
    /// the only kind [`Instance::call`] can call.
    pub fn exports_call(&self, name: &str) -> bool {
        match self.module.get_export(name) {
            Some(wasmi::ExternType::Func(ty)) => {
                ty.params().is_empty() && ty.results() == [wasmi::core::ValType::I32]
            }
            _ => false,
        }
    }

    /// Whether the module imports `module.name` (of any kind).
    pub fn imports(&self, module: &str, name: &str) -> bool {
        self.module
            .imports()
            .any(|import| import.module() == module && import.name() == name)
    }

    /// Whether the module exports a linear memory named `memory`.
    pub fn exports_memory(&self) -> bool {
        matches!(
            self.module.get_export("memory"),
            Some(wasmi::ExternType::Memory(_))
        )
    }
}

/// The interpreter plus the host functions linked into every program.
pub struct WasmRuntime<T> {
    engine: Engine,
    linker: Linker<HostState<T>>,
}

impl<T> WasmRuntime<T> {
    /// Builds a runtime. `define_host_functions` registers the imports
    /// programs may use (e.g. with [`Linker::func_wrap`]); a program that
    /// imports anything else fails to instantiate.
    pub fn new(
        config: &RuntimeConfig,
        define_host_functions: impl FnOnce(
            &mut Linker<HostState<T>>,
        ) -> core::result::Result<(), wasmi::Error>,
    ) -> Result<Self> {
        let stack_limits = StackLimits::new(
            config.max_value_stack_height.min(1024),
            config.max_value_stack_height,
            config.max_recursion_depth,
        )
        .map_err(|e| Error::Config(e.to_string()))?;
        let mut wasmi_config = Config::default();
        wasmi_config
            .set_stack_limits(stack_limits)
            .consume_fuel(true)
            // Translate the whole module up front, so a malformed module
            // fails at `compile` rather than midway through a later call.
            .compilation_mode(CompilationMode::Eager);
        let engine = Engine::new(&wasmi_config);
        let mut linker = Linker::new(&engine);
        define_host_functions(&mut linker).map_err(|e| Error::HostFunction(e.to_string()))?;
        Ok(WasmRuntime { engine, linker })
    }

    /// Validates and compiles `module_bytes`, keyed by their [`ProgramId`].
    pub fn compile(&self, module_bytes: &[u8]) -> Result<CompiledProgram> {
        let id = ProgramId::of(module_bytes);
        let module = Module::new(&self.engine, module_bytes)
            .map_err(|e| Error::InvalidModule(e.to_string()))?;
        Ok(CompiledProgram { id, module })
    }

    /// Creates a fresh instance of `program` (new linear memory, globals
    /// reset) holding `data`, and runs its start function if it has one.
    pub fn instantiate(
        &self,
        program: &CompiledProgram,
        data: T,
        limits: &Limits,
    ) -> Result<Instance<T>> {
        let mut store = Store::new(
            &self.engine,
            HostState {
                data,
                limits: StoreLimitsBuilder::new()
                    .memory_size(limits.memory_limit_bytes)
                    .build(),
            },
        );
        store.limiter(|state| &mut state.limits);
        store
            .set_fuel(limits.fuel)
            .map_err(|e| Error::Config(e.to_string()))?;
        let instance = self
            .linker
            .instantiate(&mut store, &program.module)
            .and_then(|pre| pre.start(&mut store))
            .map_err(classify_call_error)?;
        Ok(Instance {
            store,
            instance,
            fuel: limits.fuel,
        })
    }
}

/// One live instance of a [`CompiledProgram`].
pub struct Instance<T> {
    store: Store<HostState<T>>,
    instance: wasmi::Instance,
    fuel: u64,
}

impl<T> Instance<T> {
    /// Calls the `() -> i32` export `name`, with a full fuel budget.
    pub fn call(&mut self, name: &str) -> Result<i32> {
        let func = self
            .instance
            .get_typed_func::<(), i32>(&self.store, name)
            .map_err(|_| Error::MissingExport(name.to_string()))?;
        self.store
            .set_fuel(self.fuel)
            .map_err(|e| Error::Config(e.to_string()))?;
        func.call(&mut self.store, ()).map_err(classify_call_error)
    }

    pub fn data(&self) -> &T {
        &self.store.data().data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.store.data_mut().data
    }

    /// Overwrites the guest's linear memory with zeros. Call before dropping
    /// an instance that handled secrets: wasmi frees memory without wiping it.
    pub fn wipe_memory(&mut self) {
        if let Some(memory) = self.instance.get_memory(&self.store, "memory") {
            memory.data_mut(&mut self.store).zeroize();
        }
    }
}

fn classify_call_error(error: wasmi::Error) -> Error {
    if error.as_trap_code() == Some(TrapCode::OutOfFuel) {
        Error::OutOfFuel
    } else {
        Error::Trap(error.to_string())
    }
}
