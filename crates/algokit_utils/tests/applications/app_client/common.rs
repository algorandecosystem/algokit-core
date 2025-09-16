use crate::common::{
    AlgorandFixtureResult, AppFixtureOptions, AppFixtureResult, algorand_fixture,
    build_app_fixture, default_teal_params,
};
use algokit_abi::Arc56Contract;
use algokit_utils::ResourcePopulation;
use algokit_utils::transactions::TransactionComposerConfig;
use rstest::fixture;

pub fn testing_app_spec() -> Arc56Contract {
    Arc56Contract::from_json(algokit_test_artifacts::testing_app::APPLICATION_ARC56).unwrap()
}

pub fn sandbox_spec() -> Arc56Contract {
    Arc56Contract::from_json(algokit_test_artifacts::sandbox::APPLICATION_ARC56).unwrap()
}

pub fn hello_world_spec() -> Arc56Contract {
    Arc56Contract::from_json(algokit_test_artifacts::hello_world::APPLICATION_ARC56).unwrap()
}

pub fn boxmap_spec() -> Arc56Contract {
    Arc56Contract::from_json(algokit_test_artifacts::box_map_test::APPLICATION_ARC56).unwrap()
}

pub fn testing_app_puya_spec() -> Arc56Contract {
    Arc56Contract::from_json(algokit_test_artifacts::testing_app_puya::APPLICATION_ARC56).unwrap()
}

#[fixture]
pub async fn testing_app_fixture(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> AppFixtureResult {
    let f = algorand_fixture.await?;
    let spec = testing_app_spec();
    build_app_fixture(
        f,
        spec,
        AppFixtureOptions {
            template_params: Some(default_teal_params(0, false, false)),
            ..Default::default()
        },
    )
    .await
}

#[fixture]
pub async fn sandbox_app_fixture(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> AppFixtureResult {
    let f = algorand_fixture.await?;
    let spec = sandbox_spec();
    build_app_fixture(
        f,
        spec,
        AppFixtureOptions {
            template_params: Some(default_teal_params(0, false, false)),
            ..Default::default()
        },
    )
    .await
}

#[fixture]
pub async fn hello_world_app_fixture(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> AppFixtureResult {
    let f = algorand_fixture.await?;
    let spec = hello_world_spec();
    build_app_fixture(f, spec, AppFixtureOptions::default()).await
}

#[fixture]
pub async fn boxmap_app_fixture(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> AppFixtureResult {
    let f = algorand_fixture.await?;
    let spec = boxmap_spec();
    build_app_fixture(
        f,
        spec,
        AppFixtureOptions {
            args: Some(vec![vec![184u8, 68u8, 123u8, 54u8]]),
            transaction_composer_config: Some(TransactionComposerConfig {
                populate_app_call_resources: ResourcePopulation::Enabled {
                    use_access_list: false,
                },
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .await
}

#[fixture]
pub async fn testing_app_puya_fixture(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> AppFixtureResult {
    let f = algorand_fixture.await?;
    let spec = testing_app_puya_spec();
    build_app_fixture(f, spec, AppFixtureOptions::default()).await
}
