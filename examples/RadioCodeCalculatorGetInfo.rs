/******************************************************************************
 *
 * Radio Code Calculator API - WebApi interface usage example
 *
 * In this example, we will demonstrate how to get information about the
 * specific radio calculator and its parameters (max. length & regex pattern).
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
use radio_code_calculator::{RadioCodeCalculator, RadioCodeCalculatorError, RadioErrors};

#[tokio::main]
async fn main() {
    //
    // create Radio Code Calculator API class instance (we are using our activation key)
    //
    let my_radio_code_calculator =
        RadioCodeCalculator::new(Some("ABCD-ABCD-ABCD-ABCD".to_string()));

    //
    // query information about the radio model
    //
    match my_radio_code_calculator.info("ford-m-series").await {
        Ok(info) => {
            let radio_model = &info.radio_model;

            println!("Radio model name - {}<br>", radio_model.name);

            println!(
                "Max. length of the radio serial number - {}<br>",
                radio_model.serial_max_len
            );
            println!(
                "Regex pattern for the radio serial number - {}<br>",
                radio_model.serial_regex_pattern()
            );

            // is extra field specified?
            if radio_model.extra_max_len > 0 {
                println!(
                    "Max. length of the radio extra data - {}<br>",
                    radio_model.extra_max_len
                );
                println!(
                    "Regex pattern for the radio extra data - {:?}<br>",
                    radio_model.extra_regex_pattern()
                );
                println!("<br>");
            }
        }
        Err(e) => match e {
            RadioCodeCalculatorError::ApiError(v) => {
                if v["error"].as_i64() == Some(RadioErrors::INVALID_LICENSE as i64) {
                    println!("Invalid license key!");
                } else {
                    println!(
                        "Something unexpected happen while trying to login to the service (error code {:?}).",
                        v["error"]
                    );
                }
            }
            RadioCodeCalculatorError::InvalidLicense => println!("Invalid license key!"),
            RadioCodeCalculatorError::Transport(msg) => println!("Connection error: {msg}"),
        },
    }
}
