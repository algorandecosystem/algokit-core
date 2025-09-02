use super::{AppClient, AppClientError};
use crate::clients::app_manager::DeploymentMetadata;

impl AppClient {
    pub async fn compile_with_params(
        &self,
        compilation_params: &super::types::CompilationParams,
    ) -> Result<(Vec<u8>, Vec<u8>), AppClientError> {
        let approval = self
            .compile_approval_with_params(compilation_params)
            .await?;
        let clear = self.compile_clear_with_params(compilation_params).await?;

        // Emit AppCompiled event when debug flag is enabled
        if crate::config::Config::debug() {
            let app_name = self.app_name.clone();
            let approval_map = self
                .algorand()
                .app()
                .get_compilation_result(&String::from_utf8_lossy(&approval))
                .and_then(|c| c.source_map);
            let clear_map = self
                .algorand()
                .app()
                .get_compilation_result(&String::from_utf8_lossy(&clear))
                .and_then(|c| c.source_map);

            let event = crate::config::AppCompiledEventData {
                app_name,
                approval_source_map: approval_map,
                clear_source_map: clear_map,
            };
            crate::config::Config::events()
                .emit(
                    crate::config::EventType::AppCompiled,
                    crate::config::EventData::AppCompiled(event),
                )
                .await;
        }

        Ok((approval, clear))
    }

    pub async fn compile_approval_with_params(
        &self,
        compilation_params: &super::types::CompilationParams,
    ) -> Result<Vec<u8>, AppClientError> {
        let source = self.app_spec.source.as_ref().ok_or_else(|| {
            AppClientError::CompilationError("Missing source in app spec".to_string())
        })?;

        // 1) Decode TEAL from ARC-56 source
        let mut teal = source
            .get_decoded_approval()
            .map_err(|e| AppClientError::CompilationError(e.to_string()))?;

        // 2) Apply template variables if provided
        if let Some(params) = &compilation_params.deploy_time_params {
            teal =
                crate::clients::app_manager::AppManager::replace_template_variables(&teal, params)
                    .map_err(|e| AppClientError::CompilationError(e.to_string()))?;
        }

        // 3) Apply deploy-time controls
        if compilation_params.updatable.is_some() || compilation_params.deletable.is_some() {
            let metadata = DeploymentMetadata {
                updatable: compilation_params.updatable,
                deletable: compilation_params.deletable,
            };
            teal = crate::clients::app_manager::AppManager::replace_teal_template_deploy_time_control_params(&teal, &metadata)
                .map_err(|e| AppClientError::CompilationError(e.to_string()))?;
        }

        // 4) Compile to populate AppManager cache and source maps
        let _compiled = self
            .algorand()
            .app()
            .compile_teal(&teal)
            .await
            .map_err(|e| AppClientError::AppManagerError(e.to_string()))?;

        // Return TEAL source bytes (TransactionSender will pull compiled bytes from cache)
        Ok(teal.into_bytes())
    }

    pub async fn compile_clear_with_params(
        &self,
        compilation_params: &super::types::CompilationParams,
    ) -> Result<Vec<u8>, AppClientError> {
        let source = self.app_spec.source.as_ref().ok_or_else(|| {
            AppClientError::CompilationError("Missing source in app spec".to_string())
        })?;

        // 1) Decode TEAL from ARC-56 source
        let mut teal = source
            .get_decoded_clear()
            .map_err(|e| AppClientError::CompilationError(e.to_string()))?;

        // 2) Apply template variables if provided
        if let Some(params) = &compilation_params.deploy_time_params {
            teal =
                crate::clients::app_manager::AppManager::replace_template_variables(&teal, params)
                    .map_err(|e| AppClientError::CompilationError(e.to_string()))?;
        }

        // 3) NOTE: Deploy-time controls don't apply to clear program; skip

        // 4) Compile to populate AppManager cache and source maps
        let _compiled = self
            .algorand()
            .app()
            .compile_teal(&teal)
            .await
            .map_err(|e| AppClientError::AppManagerError(e.to_string()))?;

        Ok(teal.into_bytes())
    }
}
