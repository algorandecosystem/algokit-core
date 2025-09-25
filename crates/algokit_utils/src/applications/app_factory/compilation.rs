use super::{AppFactory, AppFactoryError};
use crate::applications::app_client::CompilationParams;
use crate::clients::app_manager::{CompiledPrograms, DeploymentMetadata};

impl AppFactory {
    pub(crate) async fn compile(
        &self,
        compilation_params: Option<CompilationParams>,
    ) -> Result<CompiledPrograms, AppFactoryError> {
        let compilation_params = self.resolve_compilation_params(compilation_params);

        let (approval_teal, clear_teal) =
            self.app_spec()
                .decoded_teal()
                .map_err(|e| AppFactoryError::CompilationError {
                    message: e.to_string(),
                })?;

        let metadata = DeploymentMetadata {
            updatable: compilation_params.updatable,
            deletable: compilation_params.deletable,
        };

        let approval = self
            .algorand()
            .app()
            .compile_teal_template(
                &approval_teal,
                compilation_params.deploy_time_params.as_ref(),
                Some(&metadata),
            )
            .await
            .map_err(|e| AppFactoryError::CompilationError {
                message: e.to_string(),
            })?;

        let clear = self
            .algorand()
            .app()
            .compile_teal_template(
                &clear_teal,
                compilation_params.deploy_time_params.as_ref(),
                None,
            )
            .await
            .map_err(|e| AppFactoryError::CompilationError {
                message: e.to_string(),
            })?;

        self.update_source_maps(approval.source_map.clone(), clear.source_map.clone());

        Ok(CompiledPrograms { approval, clear })
    }
}
