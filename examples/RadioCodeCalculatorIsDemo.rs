/******************************************************************************
 *
 * Radio Code Calculator API - WebApi interface usage example
 *
 * In this example we will verify our activation key status.
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
    // login to the service
    //
    match my_radio_code_calculator.login().await {
        Ok(result) => {
            //
            // result[] array holds the information about the license
            //
            // result["license"]["activationStatus"] - True if license is active, False on invalid/expired keys
            // result["license"]["userName"] - user name/company name of the license owner
            // result["license"]["type"] - license type (0 - Personal License, 1 - Company License)
            // result["license"]["expirationDate"] - license expiration date (in YYYY-MM-DD format)
            //
            let license = &result["license"];
            println!(
                "License activation status - {}<br>",
                if license["activationStatus"].as_bool().unwrap_or(false) {
                    "True"
                } else {
                    "False"
                }
            );
            println!("License owner - {}", license["userName"]);
            println!(
                "License type - {}<br>",
                if license["type"].as_i64() == Some(0) {
                    "Personal"
                } else {
                    "Company"
                }
            );
            println!("Expiration date - {}<br>", license["expirationDate"]);
        }
        Err(e) => match e {
            RadioCodeCalculatorError::ApiError(ref v) => {
                if v["error"].as_i64() == Some(RadioErrors::INVALID_LICENSE as i64) {
                    println!("Invalid license key!");
                } else {
                    println!("Something unexpected happen while trying to login to the service (error code {v:?}).");
                }
            }
            RadioCodeCalculatorError::InvalidLicense => println!("Invalid license key!"),
            RadioCodeCalculatorError::Transport(msg) => println!("Connection error: {msg}"),
        },
    }
}
