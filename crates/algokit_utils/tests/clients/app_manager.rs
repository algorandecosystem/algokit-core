use algokit_test_artifacts::template_variables;
use algokit_utils::{clients::app_manager::*, testing::algorand_fixture};
use base64::prelude::*;
use rstest::*;
use std::{collections::HashMap, sync::Arc};

/// Test template variable replacement behavior
#[rstest]
#[case("pushint TMPL_NUMBER\npushbytes TMPL_STRING",
       &[("NUMBER", TealTemplateValue::Int(42)), ("STRING", TealTemplateValue::String("hello".to_string()))],
       "pushint 42\npushbytes 0x68656c6c6f")]
#[case("pushint TMPL_UPDATABLE\npushint TMPL_DELETABLE",
       &[("UPDATABLE", TealTemplateValue::Int(1)), ("DELETABLE", TealTemplateValue::Int(0))],
       "pushint 1\npushint 0")]
#[case("pushbytes \"TMPL_NUMBER\"\npushint TMPL_NUMBER",
       &[("NUMBER", TealTemplateValue::Int(42))],
       "pushbytes \"TMPL_NUMBER\"\npushint 42")]
#[case("TMPL_X TMPL_X TMPL_X",
       &[("X", TealTemplateValue::String("test".to_string()))],
       "0x74657374 0x74657374 0x74657374")]
fn test_template_variable_replacement_behavior(
    #[case] teal_code: &str,
    #[case] template_vars: &[(&str, TealTemplateValue)],
    #[case] expected: &str,
) {
    let template_map = template_vars
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect();

    let result = AppManager::replace_template_variables(teal_code, &template_map).unwrap();
    assert_eq!(result.trim(), expected.trim());
}

/// Test comprehensive comment stripping behavior with all edge cases
#[test]
fn test_comprehensive_comment_stripping() {
    let input = r#"//comment
op arg //comment
op "arg" //comment
op "//" //comment
op "  //comment  " //comment
op "\" //" //comment
op "// \" //" //comment
op "" //comment
//
op 123
op 123 // something
op "" // more comments
op "//" //op "//"
op "//"
pushbytes base64(//8=)
pushbytes b64(//8=)

pushbytes base64(//8=)  // pushbytes base64(//8=)
pushbytes b64(//8=)     // pushbytes b64(//8=)
pushbytes "base64(//8=)"  // pushbytes "base64(//8=)"
pushbytes "b64(//8=)"     // pushbytes "b64(//8=)"

pushbytes base64 //8=
pushbytes b64 //8=

pushbytes base64 //8=  // pushbytes base64 //8=
pushbytes b64 //8=     // pushbytes b64 //8=
pushbytes "base64 //8="  // pushbytes "base64 //8="
pushbytes "b64 //8="     // pushbytes "b64 //8=""#;

    let expected = r#"
op arg
op "arg"
op "//"
op "  //comment  "
op "\" //"
op "// \" //"
op ""

op 123
op 123
op ""
op "//"
op "//"
pushbytes base64(//8=)
pushbytes b64(//8=)

pushbytes base64(//8=)
pushbytes b64(//8=)
pushbytes "base64(//8=)"
pushbytes "b64(//8=)"

pushbytes base64 //8=
pushbytes b64 //8=

pushbytes base64 //8=
pushbytes b64 //8=
pushbytes "base64 //8="
pushbytes "b64 //8=""#;

    let result = AppManager::strip_teal_comments(input);
    assert_eq!(result.trim(), expected.trim());
}

/// Test TEAL compilation
#[tokio::test]
async fn test_teal_compilation() {
    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();
    let app_manager = AppManager::new(Arc::new(fixture.context().unwrap().algod.clone()));

    let teal = "#pragma version 3\npushint 1\nreturn";
    let result = app_manager.compile_teal(teal).await.unwrap();

    assert_eq!(result.teal, teal);
    assert!(!result.compiled_base64_to_bytes.is_empty());
    assert!(!result.compiled_hash.is_empty());

    // Test caching
    let cached = app_manager.compile_teal(teal).await.unwrap();
    assert_eq!(result.compiled_hash, cached.compiled_hash);
}

/// Test template compilation
#[tokio::test]
async fn test_template_compilation() {
    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();
    let app_manager = AppManager::new(Arc::new(fixture.context().unwrap().algod.clone()));

    let template_params = HashMap::from([("VALUE".to_string(), TealTemplateValue::Int(42))]);
    let result = app_manager
        .compile_teal_template(
            "#pragma version 3\npushint TMPL_VALUE\nreturn",
            Some(&template_params),
            None,
        )
        .await
        .unwrap();

    assert!(result.teal.contains("pushint 42"));
    assert!(!result.teal.contains("TMPL_"));
    assert!(!result.compiled_base64_to_bytes.is_empty());
}

/// Test compilation caching
#[tokio::test]
async fn test_compilation_caching() {
    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();
    let app_manager = AppManager::new(Arc::new(fixture.context().unwrap().algod.clone()));

    let teal = "#pragma version 3\npushint 100\nreturn";
    app_manager.compile_teal(teal).await.unwrap();

    let cached = app_manager.get_compilation_result(teal).unwrap();
    assert_eq!(cached.teal, teal);
    assert!(!cached.compiled_base64_to_bytes.is_empty());
}

/// Test deploy-time control
#[tokio::test]
async fn test_deploy_time_control() {
    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();
    let app_manager = AppManager::new(Arc::new(fixture.context().unwrap().algod.clone()));

    let template = format!(
        "#pragma version 3\npushint {}\npushint {}\nreturn",
        UPDATABLE_TEMPLATE_NAME, DELETABLE_TEMPLATE_NAME
    );
    let metadata = DeploymentMetadata {
        updatable: Some(true),
        deletable: Some(false),
    };

    let result = app_manager
        .compile_teal_template(&template, None, Some(&metadata))
        .await
        .unwrap();

    assert!(result.teal.contains("pushint 1"));
    assert!(result.teal.contains("pushint 0"));
    assert!(!result.teal.contains("TMPL_"));
}

/// Test real contract compilation
#[tokio::test]
async fn test_real_contract_compilation() {
    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();
    let app_manager = AppManager::new(Arc::new(fixture.context().unwrap().algod.clone()));

    let contract: serde_json::Value =
        serde_json::from_str(template_variables::APPLICATION_ARC56).unwrap();
    let approval_teal = contract["source"]["approval"].as_str().unwrap();
    let approval_code = String::from_utf8(BASE64_STANDARD.decode(approval_teal).unwrap()).unwrap();

    let template_params = HashMap::from([
        ("uint64TmplVar".to_string(), TealTemplateValue::Int(42)),
        ("bytesTmplVar".to_string(), TealTemplateValue::String("hello".to_string())),
        ("bytes32TmplVar".to_string(), TealTemplateValue::String("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string())),
        ("bytes64TmplVar".to_string(), TealTemplateValue::String("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string())),
    ]);

    let result = app_manager
        .compile_teal_template(&approval_code, Some(&template_params), None)
        .await
        .unwrap();

    assert!(!result.compiled_base64_to_bytes.is_empty());
    assert!(!result.compiled_hash.is_empty());
}

/// Test template substitution
#[test]
fn test_template_substitution() {
    let program = r#"test TMPL_INT // TMPL_INT
test TMPL_INT
no change
test TMPL_STR // TMPL_STR
TMPL_STR
TMPL_STR // TMPL_INT
TMPL_STR // foo //
TMPL_STR // bar
test "TMPL_STR" // not replaced
test "TMPL_STRING" // not replaced
test TMPL_STRING // not replaced
test TMPL_STRI // not replaced
test TMPL_STR TMPL_INT TMPL_INT TMPL_STR // TMPL_STR TMPL_INT TMPL_INT TMPL_STR
test TMPL_INT TMPL_STR TMPL_STRING "TMPL_INT TMPL_STR TMPL_STRING" //TMPL_INT TMPL_STR TMPL_STRING
test TMPL_INT TMPL_INT TMPL_STRING TMPL_STRING TMPL_STRING TMPL_INT TMPL_STRING //keep
TMPL_STR TMPL_STR TMPL_STR
TMPL_STRING
test NOTTMPL_STR // not replaced
NOTTMPL_STR // not replaced
TMPL_STR // replaced"#;

    let mut template_values = HashMap::new();
    template_values.insert("INT".to_string(), TealTemplateValue::Int(123));
    template_values.insert(
        "STR".to_string(),
        TealTemplateValue::String("ABC".to_string()),
    );

    let result = AppManager::replace_template_variables(program, &template_values)
        .expect("Template replacement should succeed");

    let expected = r#"test 123 // TMPL_INT
test 123
no change
test 0x414243 // TMPL_STR
0x414243
0x414243 // TMPL_INT
0x414243 // foo //
0x414243 // bar
test "TMPL_STR" // not replaced
test "TMPL_STRING" // not replaced
test TMPL_STRING // not replaced
test TMPL_STRI // not replaced
test 0x414243 123 123 0x414243 // TMPL_STR TMPL_INT TMPL_INT TMPL_STR
test 123 0x414243 TMPL_STRING "TMPL_INT TMPL_STR TMPL_STRING" //TMPL_INT TMPL_STR TMPL_STRING
test 123 123 TMPL_STRING TMPL_STRING TMPL_STRING 123 TMPL_STRING //keep
0x414243 0x414243 0x414243
TMPL_STRING
test NOTTMPL_STR // not replaced
NOTTMPL_STR // not replaced
0x414243 // replaced"#;

    // Verify the output matches exactly
    assert_eq!(result.trim(), expected.trim());
}

/// Test compilation error handling
#[tokio::test]
async fn test_compilation_errors() {
    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();
    let app_manager = AppManager::new(Arc::new(fixture.context().unwrap().algod.clone()));

    // Invalid TEAL should fail
    let result = app_manager
        .compile_teal("#pragma version 3\ninvalid_opcode_xyz")
        .await;
    assert!(result.is_err());

    // Missing template variables should either preserve or fail
    let result = app_manager
        .compile_teal_template(
            "#pragma version 3\npushint TMPL_MISSING\nreturn",
            None,
            None,
        )
        .await;

    match result {
        Ok(compiled) => assert!(compiled.teal.contains("TMPL_MISSING")),
        Err(_) => {} // Both outcomes are acceptable
    }
}
