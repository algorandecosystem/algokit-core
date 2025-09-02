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
