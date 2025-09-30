use algokit_transact::*;
use base64::Engine;

fn tx_id_from_indexer_tx_json(json: &str) -> String {
    // Minimal extraction of fields necessary to build a Transaction for ID comparison
    let v: serde_json::Value = serde_json::from_str(json).unwrap();
    let t = &v["transaction"];

    // Map common header fields
    let mut header_b = TransactionHeaderBuilder::default();
    if let Some(fv) = t["first-valid"].as_u64() {
        header_b.first_valid(fv);
    }
    if let Some(lv) = t["last-valid"].as_u64() {
        header_b.last_valid(lv);
    }
    if let Some(fee) = t["fee"].as_u64() {
        header_b.fee(fee);
    }
    if let Some(sender) = t["sender"].as_str() {
        header_b.sender(sender.parse().unwrap());
    }
    if let Some(gh) = t["genesis-hash"].as_str() {
        let gh_bytes: Byte32 = base64::engine::general_purpose::STANDARD
            .decode(gh)
            .unwrap()
            .try_into()
            .unwrap();
        header_b.genesis_hash(gh_bytes);
    }
    if let Some(gid) = t["genesis-id"].as_str() {
        header_b.genesis_id(gid.to_string());
    }
    if let Some(note_b64) = t["note"].as_str() {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(note_b64)
            .unwrap();
        header_b.note(bytes);
    }
    let header = header_b.build().unwrap();

    match t["tx-type"].as_str().unwrap() {
        "appl" => {
            let appl = &t["application-transaction"];
            let mut b = AppCallTransactionBuilder::default();
            b.header(header);

            if let Some(id) = appl["application-id"].as_u64() {
                b.app_id(id);
            }

            // on-completion
            if let Some(onc) = appl["on-completion"].as_str() {
                let oc = match onc {
                    "noop" => OnApplicationComplete::NoOp,
                    "optin" => OnApplicationComplete::OptIn,
                    "closeout" => OnApplicationComplete::CloseOut,
                    "clear" => OnApplicationComplete::ClearState,
                    "update" => OnApplicationComplete::UpdateApplication,
                    "delete" => OnApplicationComplete::DeleteApplication,
                    _ => OnApplicationComplete::NoOp,
                };
                b.on_complete(oc);
            }

            // programs (optional in indexer JSON)
            if let Some(apap) = appl["approval-program"].as_str() {
                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(apap)
                    .unwrap();
                b.approval_program(bytes);
            }
            if let Some(apsu) = appl["clear-state-program"].as_str() {
                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(apsu)
                    .unwrap();
                b.clear_state_program(bytes);
            }

            // do not set state schemas unless this is a create (app_id == 0)
            if appl["application-id"].as_u64().unwrap_or(0) == 0 {
                if let Some(gs) = appl.get("global-state-schema") {
                    let nu = gs["num-uint"].as_u64().unwrap_or(0) as u32;
                    let nb = gs["num-byte-slice"].as_u64().unwrap_or(0) as u32;
                    b.global_state_schema(StateSchema {
                        num_uints: nu,
                        num_byte_slices: nb,
                    });
                }
                if let Some(ls) = appl.get("local-state-schema") {
                    let nu = ls["num-uint"].as_u64().unwrap_or(0) as u32;
                    let nb = ls["num-byte-slice"].as_u64().unwrap_or(0) as u32;
                    b.local_state_schema(StateSchema {
                        num_uints: nu,
                        num_byte_slices: nb,
                    });
                }
            }

            // args (common)
            if let Some(args) = appl["application-args"].as_array() {
                let decoded: Vec<Vec<u8>> = args
                    .iter()
                    .filter_map(|a| a.as_str())
                    .map(|s| base64::engine::general_purpose::STANDARD.decode(s).unwrap())
                    .collect();
                if !decoded.is_empty() {
                    b.args(decoded);
                }
            }

            let tx = b.build().unwrap();
            tx.id().unwrap()
        }
        other => panic!("Unsupported tx-type for test: {}", other),
    }
}

#[test]
fn tx_id_matches_app_create_with_global_state_delta() {
    let json = algokit_test_artifacts::msgpack::TESTNET_GLOBAL_STATE_DELTA_TX;
    let v: serde_json::Value = serde_json::from_str(json).unwrap();
    let expected = v["transaction"]["id"].as_str().unwrap();

    let recalculated = tx_id_from_indexer_tx_json(json);
    assert_eq!(recalculated, expected);
}

#[test]
fn tx_id_matches_app_call_with_local_state_delta() {
    let json = algokit_test_artifacts::msgpack::TESTNET_LOCAL_STATE_DELTA_TX;
    let v: serde_json::Value = serde_json::from_str(json).unwrap();
    let expected = v["transaction"]["id"].as_str().unwrap();

    let recalculated = tx_id_from_indexer_tx_json(json);
    assert_eq!(recalculated, expected);
}
