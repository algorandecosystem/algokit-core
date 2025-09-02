use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

/// Minimal lifecycle event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    /// Emitted when an app is compiled (for source map capture)
    AppCompiled,
    /// Emitted when a transaction group is simulated (for AVM traces)
    TxnGroupSimulated,
}

/// Minimal event payloads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppCompiledEventData {
    pub app_name: Option<String>,
    pub approval_source_map: Option<serde_json::Value>,
    pub clear_source_map: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxnGroupSimulatedEventData {
    pub simulate_response: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum EventData {
    AppCompiled(AppCompiledEventData),
    TxnGroupSimulated(TxnGroupSimulatedEventData),
}

/// Async event emitter using Tokio broadcast
#[derive(Clone)]
pub struct AsyncEventEmitter {
    sender: broadcast::Sender<(EventType, EventData)>,
}

impl AsyncEventEmitter {
    pub fn new(buffer: usize) -> Self {
        let (sender, _receiver) = broadcast::channel(buffer);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<(EventType, EventData)> {
        self.sender.subscribe()
    }

    pub async fn emit(&self, event_type: EventType, data: EventData) {
        // Ignore error if there are no subscribers
        let _ = self.sender.send((event_type, data));
    }
}

/// Configuration with minimal flags compatible with TS Config
#[derive(Clone)]
pub struct ConfigInner {
    pub debug: bool,
    pub trace_all: bool,
    pub events: AsyncEventEmitter,
}

impl Default for ConfigInner {
    fn default() -> Self {
        Self {
            debug: false,
            trace_all: false,
            events: AsyncEventEmitter::new(32),
        }
    }
}

/// Global runtime config singleton
pub struct Config;

impl Config {
    pub fn get() -> &'static Lazy<std::sync::Mutex<ConfigInner>> {
        static INSTANCE: Lazy<std::sync::Mutex<ConfigInner>> =
            Lazy::new(|| std::sync::Mutex::new(ConfigInner::default()));
        &INSTANCE
    }

    pub fn debug() -> bool {
        Self::get().lock().unwrap().debug
    }

    pub fn trace_all() -> bool {
        Self::get().lock().unwrap().trace_all
    }

    pub fn events() -> AsyncEventEmitter {
        Self::get().lock().unwrap().events.clone()
    }

    pub fn configure(new_debug: Option<bool>, new_trace_all: Option<bool>) {
        let mut cfg = Self::get().lock().unwrap();
        if let Some(d) = new_debug {
            cfg.debug = d;
        }
        if let Some(t) = new_trace_all {
            cfg.trace_all = t;
        }
    }
}
