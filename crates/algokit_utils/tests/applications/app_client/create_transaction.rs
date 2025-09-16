use crate::applications::app_client::common::testing_app_fixture;
use crate::common::TestResult;
use algokit_abi::ABIValue;
use algokit_transact::BoxReference;
use algokit_utils::applications::app_client::AppClientMethodCallParams;
use algokit_utils::AppMethodCallArg;
use rstest::*;

#[rstest]
#[tokio::test]
async fn create_txn_with_box_references(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = testing_app_fixture.await?;
    let sender = f.sender_address;
    let client = f.client;

    let tx = client
        .create_transaction()
        .call(
            AppClientMethodCallParams {
                method: "call_abi".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from("test"))],
                sender: Some(sender.to_string()),
                box_references: Some(vec![BoxReference {
                    app_id: 0,
                    name: b"1".to_vec(),
                }]),
                ..Default::default()
            },
            None,
        )
        .await?;

    if let algokit_transact::Transaction::AppCall(fields) = tx {
        let boxes = fields.box_references.expect("boxes");
        assert_eq!(boxes.len(), 1);
        assert_eq!(boxes[0].app_id, 0);
        assert_eq!(boxes[0].name, b"1".to_vec());
    } else {
        return Err("expected app call txn".into());
    }

    Ok(())
}
