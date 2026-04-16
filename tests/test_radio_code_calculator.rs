/******************************************************************************
 *
 * Radio Code Calculator API - WebApi interface unit test
 *
 * Validate Radio Code Calculator Web API responses
 *
 * Run with cargo test
 *
 * Tests that need a live, active license are marked #[ignore]. Run them with:
 *
 *   set RADIO_CODE_CALCULATOR_API_KEY=your-key-here
 *   cargo test -- --ignored
 *
 * Version      : v1.0.0
 * Rust         : 2021
 * Dependencies : radio-code-calculator, tokio
 * Author       : Bartosz Wójcik (support@pelock.com);
 * Project      : https://www.pelock.com/products/radio-code-calculator
 * Homepage     : https://www.pelock.com
 *
 * @link https://www.pelock.com/products/radio-code-calculator
 * @copyright Copyright (c) 2021-2026 PELock LLC
 * @license Apache-2.0
 *
 *****************************************************************************/

//
// include Radio Code Calculator API module
//
use std::collections::HashMap;

use radio_code_calculator::{
    RadioCodeCalculator, RadioCodeCalculatorError, RadioErrors, RadioModel, RadioModels,
};

/// Prefer `RADIO_CODE_CALCULATOR_API_KEY`; fallback matches the original JS tests' placeholder.
fn api_key() -> String {
    std::env::var("RADIO_CODE_CALCULATOR_API_KEY")
        .unwrap_or_else(|_| "ABCD-ABCD-ABCD-ABCD".to_string())
}

/// @var RadioCodeCalculator global instance of RadioCodeCalculator
fn calculator() -> RadioCodeCalculator {
    RadioCodeCalculator::new(Some(api_key()))
}

#[tokio::test]
#[ignore = "requires active RADIO_CODE_CALCULATOR_API_KEY; run: cargo test -- --ignored"]
async fn test_login() {
    let my_radio_code_calculator = calculator();
    match my_radio_code_calculator.login().await {
        Ok(result) => {
            assert!(result.get("error").is_some());
            assert_eq!(result["error"].as_i64(), Some(RadioErrors::SUCCESS as i64));
            assert!(result.get("license").is_some());
            let license = &result["license"];
            assert!(license.get("userName").is_some());
            assert!(license.get("type").is_some());
            assert!(license.get("expirationDate").is_some());
        }
        Err(e) => panic!("login failed: {e:?}"),
    }
}

#[tokio::test]
async fn test_login_invalid() {
    // provide invalid license key
    let radio_code_api = RadioCodeCalculator::new(Some("AAAA-BBBB-CCCC-DDDD".to_string()));

    match radio_code_api.login().await {
        Ok(_) => panic!("expected API error"),
        Err(RadioCodeCalculatorError::ApiError(v)) => {
            assert_eq!(
                v["error"].as_i64(),
                Some(RadioErrors::INVALID_LICENSE as i64)
            );
        }
        Err(e) => panic!("unexpected error: {e:?}"),
    }
}

#[tokio::test]
#[ignore = "requires active RADIO_CODE_CALCULATOR_API_KEY; run: cargo test -- --ignored"]
async fn test_invalid_radio_model() {
    let my_radio_code_calculator = calculator();

    match my_radio_code_calculator
        .calc("INVALID RADIO MODEL", "1234", "")
        .await
    {
        Ok(_) => panic!("expected API error"),
        Err(RadioCodeCalculatorError::ApiError(v)) => {
            assert_eq!(
                v["error"].as_i64(),
                Some(RadioErrors::INVALID_RADIO_MODEL as i64)
            );
        }
        Err(e) => panic!("unexpected error: {e:?}"),
    }
}

#[tokio::test]
#[ignore = "requires active RADIO_CODE_CALCULATOR_API_KEY; run: cargo test -- --ignored"]
async fn test_radio_command() {
    // send invalid command to the service
    let my_radio_code_calculator = calculator();
    let mut params = HashMap::new();
    params.insert("command".to_string(), "INVALID COMMAND".to_string());

    match my_radio_code_calculator.post_request(params).await {
        Ok(_) => panic!("expected API error"),
        Err(RadioCodeCalculatorError::ApiError(v)) => {
            assert_eq!(
                v["error"].as_i64(),
                Some(RadioErrors::INVALID_COMMAND as i64)
            );
        }
        Err(e) => panic!("unexpected error: {e:?}"),
    }
}

#[tokio::test]
#[ignore = "requires active RADIO_CODE_CALCULATOR_API_KEY; run: cargo test -- --ignored"]
async fn test_radio_codes() {
    let my_radio_code_calculator = calculator();

    // valid pair of radio codes to test the calculator
    let codes: &[(&RadioModel, &str, &str)] = &[
        (&*RadioModels::RENAULT_DACIA, "Z999", "0060"),
        (&*RadioModels::CHRYSLER_PANASONIC_TM9, "1234", "8865"),
        (&*RadioModels::CHRYSLER_DODGE_VP, "E3NE", "5635"),
        (&*RadioModels::FORD_M_SERIES, "123456", "2487"),
        (&*RadioModels::FORD_V_SERIES, "123456", "3067"),
        (&*RadioModels::FORD_TRAVELPILOT, "1234567", "3982"),
        (&*RadioModels::FIAT_STILO_BRAVO_VISTEON, "999999", "4968"),
        (&*RadioModels::FIAT_DAIICHI, "6461", "8354"),
        (&*RadioModels::FIAT_VP, "2063", "1341"),
        (&*RadioModels::TOYOTA_ERC, "10211376ab8e0d25", "A6905892"),
        (&*RadioModels::JEEP_CHEROKEE, "TQ1AA1500E2884", "1315"),
        (
            &*RadioModels::NISSAN_GLOVE_BOX,
            "D4CDDC568498",
            "55B7AB0BAB6F",
        ),
        (&*RadioModels::ECLIPSE_ESN, "7D4046", "15E0ED"),
        (&*RadioModels::JAGUAR_ALPINE, "99999", "6125"),
    ];

    for (model, seed, key) in codes {
        // offline validate input first
        assert_eq!(model.validate(seed, None), RadioErrors::SUCCESS);

        match my_radio_code_calculator.calc(model, seed, "").await {
            Ok(result) => {
                assert_eq!(result["error"].as_i64(), Some(RadioErrors::SUCCESS as i64));
                assert_eq!(result["code"].as_str(), Some(*key));
            }
            Err(e) => panic!("calc failed: {e:?}"),
        }
    }
}

#[tokio::test]
#[ignore = "requires active RADIO_CODE_CALCULATOR_API_KEY; run: cargo test -- --ignored"]
async fn test_radio_code_len() {
    let my_radio_code_calculator = calculator();

    // invalid radio serial length
    match my_radio_code_calculator
        .calc(&*RadioModels::FORD_M_SERIES, "1", "")
        .await
    {
        Ok(_) => panic!("expected API error"),
        Err(RadioCodeCalculatorError::ApiError(v)) => {
            assert_eq!(
                v["error"].as_i64(),
                Some(RadioErrors::INVALID_SERIAL_LENGTH as i64)
            );
        }
        Err(e) => panic!("unexpected error: {e:?}"),
    }
}

#[tokio::test]
#[ignore = "requires active RADIO_CODE_CALCULATOR_API_KEY; run: cargo test -- --ignored"]
async fn test_radio_code_pattern() {
    let my_radio_code_calculator = calculator();

    // calculate the code with invalid regex pattern
    match my_radio_code_calculator
        .calc(&*RadioModels::FORD_M_SERIES, "12345A", "")
        .await
    {
        Ok(_) => panic!("expected API error"),
        Err(RadioCodeCalculatorError::ApiError(v)) => {
            assert_eq!(
                v["error"].as_i64(),
                Some(RadioErrors::INVALID_SERIAL_PATTERN as i64)
            );
        }
        Err(e) => panic!("unexpected error: {e:?}"),
    }
}
