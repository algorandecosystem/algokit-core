use super::{AppFactory, AppFactoryError};
use crate::applications::app_client::CompilationParams;

pub(crate) struct CompiledPrograms {
    pub approval: crate::clients::app_manager::CompiledTeal,
    pub clear: crate::clients::app_manager::CompiledTeal,
}

impl AppFactory {
    pub(crate) fn resolve_compilation_params(
        &self,
        override_cp: Option<CompilationParams>,
    ) -> CompilationParams {
        let mut resolved = override_cp.unwrap_or_default();
        if resolved.deploy_time_params.is_none() {
            resolved.deploy_time_params = self.deploy_time_params.clone();
        }
        if resolved.updatable.is_none() {
            resolved.updatable = self.updatable;
        }
        if resolved.deletable.is_none() {
            resolved.deletable = self.deletable;
        }
        resolved
    }

    #[allow(dead_code)]
    pub(crate) async fn compile_programs(&self) -> Result<CompiledPrograms, AppFactoryError> {
        let source = self.app_spec().source.as_ref().ok_or_else(|| {
            AppFactoryError::CompilationError("Missing source in app spec".to_string())
        })?;

        let approval_teal = source
            .get_decoded_approval()
            .map_err(|e| AppFactoryError::CompilationError(e.to_string()))?;
        let clear_teal = source
            .get_decoded_clear()
            .map_err(|e| AppFactoryError::CompilationError(e.to_string()))?;

        let metadata = crate::clients::app_manager::DeploymentMetadata {
            updatable: self.updatable,
            deletable: self.deletable,
        };
        let metadata_opt = if metadata.updatable.is_some() || metadata.deletable.is_some() {
            Some(&metadata)
        } else {
            None
        };
        let approval = self
            .algorand()
            .app()
            .compile_teal_template(
                &approval_teal,
                self.deploy_time_params.as_ref(),
                metadata_opt,
            )
            .await
            .map_err(|e| AppFactoryError::CompilationError(e.to_string()))?;

        let clear = self
            .algorand()
            .app()
            .compile_teal_template(&clear_teal, self.deploy_time_params.as_ref(), None)
            .await
            .map_err(|e| AppFactoryError::CompilationError(e.to_string()))?;

        // Capture source maps for export
        if crate::config::Config::debug() {
            if let Some(map) = &approval.source_map {
                // best-effort capture; avoid failing compile on map issues
                let _ = map.clone();
            }
            if let Some(map) = &clear.source_map {
                let _ = map.clone();
            }
        }

        Ok(CompiledPrograms { approval, clear })
    }

    pub(crate) async fn compile_programs_with(
        &self,
        override_cp: Option<CompilationParams>,
    ) -> Result<CompiledPrograms, AppFactoryError> {
        let cp = self.resolve_compilation_params(override_cp);
        let source = self.app_spec().source.as_ref().ok_or_else(|| {
            AppFactoryError::CompilationError("Missing source in app spec".to_string())
        })?;

        let approval_teal = source
            .get_decoded_approval()
            .map_err(|e| AppFactoryError::CompilationError(e.to_string()))?;
        let clear_teal = source
            .get_decoded_clear()
            .map_err(|e| AppFactoryError::CompilationError(e.to_string()))?;

        let metadata = crate::clients::app_manager::DeploymentMetadata {
            updatable: cp.updatable,
            deletable: cp.deletable,
        };
        let metadata_opt = if metadata.updatable.is_some() || metadata.deletable.is_some() {
            Some(&metadata)
        } else {
            None
        };
        let approval = self
            .algorand()
            .app()
            .compile_teal_template(&approval_teal, cp.deploy_time_params.as_ref(), metadata_opt)
            .await
            .map_err(|e| AppFactoryError::CompilationError(e.to_string()))?;

        let clear = self
            .algorand()
            .app()
            .compile_teal_template(&clear_teal, cp.deploy_time_params.as_ref(), None)
            .await
            .map_err(|e| AppFactoryError::CompilationError(e.to_string()))?;

        Ok(CompiledPrograms { approval, clear })
    }
}
