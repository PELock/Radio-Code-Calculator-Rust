/******************************************************************************
 *
 * Radio Code Calculator API - WebApi interface usage example
 *
 * In this example, we will demonstrate how to generate a code for a specific
 * type of car radio.
 *
 * Version      : v1.0.0
 * Rust         : 2021
 * Dependencies : radio-code-calculator
 * Author       : Bartosz Wójcik (support@pelock.com)
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
use radio_code_calculator::{
    RadioCodeCalculator, RadioCodeCalculatorError, RadioErrors, RadioModels,
};

#[tokio::main]
async fn main() {
    //
    // create Radio Code Calculator API class instance (we are using our activation key)
    //
    let my_radio_code_calculator =
        RadioCodeCalculator::new(Some("ABCD-ABCD-ABCD-ABCD".to_string()));

    //
    // generate radio code (using Web API)
    //
    match my_radio_code_calculator
        .calc(&*RadioModels::FORD_M_SERIES, "123456", "")
        .await
    {
        Ok(result) => {
            println!("Radio code is {}", result["code"]);
        }
        Err(e) => match e {
            RadioCodeCalculatorError::ApiError(ref v) => match v["error"].as_i64().unwrap_or(-1) as i32 {
                RadioErrors::INVALID_RADIO_MODEL => {
                    println!("Invalid radio model (not supported)");
                }
                RadioErrors::INVALID_SERIAL_LENGTH => println!(
                    "Invalid serial number length (expected {} characters)",
                    v["serialMaxLen"]
                ),
                RadioErrors::INVALID_SERIAL_PATTERN => println!(
                    "Invalid serial number regular expression pattern (expected {} regex pattern)",
                    v["serialRegexPattern"]["php"]
                ),
                RadioErrors::INVALID_SERIAL_NOT_SUPPORTED => {
                    println!("This serial number is not supported");
                }
                RadioErrors::INVALID_EXTRA_LENGTH => println!(
                    "Invalid extra data length (expected {} characters)",
                    v["extraMaxLen"]
                ),
                RadioErrors::INVALID_EXTRA_PATTERN => println!(
                    "Invalid extra data regular expression pattern (expected {} regex pattern",
                    v["extraRegexPattern"]["php"]
                ),
                RadioErrors::INVALID_INPUT => println!("Invalid input data"),
                RadioErrors::INVALID_COMMAND => {
                    println!("Invalid command sent to the Web API interface");
                }
                RadioErrors::INVALID_LICENSE => println!("Invalid license key!"),
                _ => println!("Something unexpected happen while trying to login to the service (error code {v:?})."),
            },
            RadioCodeCalculatorError::InvalidLicense => println!("Invalid license key!"),
            RadioCodeCalculatorError::Transport(msg) => {
                println!("Connection error: {msg}");
            }
        },
    }
}
