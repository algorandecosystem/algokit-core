use super::{AppFactory, AppFactoryError};
use crate::applications::app_client::CompilationParams;
use crate::clients::app_manager::CompiledPrograms;

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
            resolved.updatable = self.updatable.or_else(|| {
                self.detect_deploy_time_control_flag(
                    crate::clients::app_manager::UPDATABLE_TEMPLATE_NAME,
                    algokit_abi::arc56_contract::CallOnApplicationComplete::UpdateApplication,
                )
            });
        }
        if resolved.deletable.is_none() {
            resolved.deletable = self.deletable.or_else(|| {
                self.detect_deploy_time_control_flag(
                    crate::clients::app_manager::DELETABLE_TEMPLATE_NAME,
                    algokit_abi::arc56_contract::CallOnApplicationComplete::DeleteApplication,
                )
            });
        }
        resolved
    }

    // Removed unused compile_programs in favor of compile_programs_with

    pub(crate) async fn compile_programs_with(
        &self,
        override_cp: Option<CompilationParams>,
    ) -> Result<CompiledPrograms, AppFactoryError> {
        let cp = self.resolve_compilation_params(override_cp);
        let source =
            self.app_spec()
                .source
                .as_ref()
                .ok_or_else(|| AppFactoryError::CompilationError {
                    message: "Missing source in app spec".to_string(),
                })?;

        let approval_teal =
            source
                .get_decoded_approval()
                .map_err(|e| AppFactoryError::CompilationError {
                    message: e.to_string(),
                })?;
        let clear_teal =
            source
                .get_decoded_clear()
                .map_err(|e| AppFactoryError::CompilationError {
                    message: e.to_string(),
                })?;

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
            .map_err(|e| AppFactoryError::CompilationError {
                message: e.to_string(),
            })?;

        let clear = self
            .algorand()
            .app()
            .compile_teal_template(&clear_teal, cp.deploy_time_params.as_ref(), None)
            .await
            .map_err(|e| AppFactoryError::CompilationError {
                message: e.to_string(),
            })?;

        self.update_source_maps(approval.source_map.clone(), clear.source_map.clone());

        Ok(CompiledPrograms { approval, clear })
    }
}
