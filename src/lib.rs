/******************************************************************************
 *
 * Radio Code Calculator API - WebApi interface
 *
 * Generate radio unlocking codes for various radio players.
 *
 * Version      : v1.0.0
 * Rust         : 2021
 * Dependencies : reqwest, serde_json, regex
 * Author       : Bartosz Wójcik (support@pelock.com)
 * Project      : https://www.pelock.com/products/radio-code-calculator
 * Homepage     : https://www.pelock.com
 *
 * @link https://www.pelock.com/products/radio-code-calculator
 * @copyright Copyright (c) 2021-2026 PELock LLC
 * @license Apache-2.0
 *
 *****************************************************************************/

mod client;
mod error;
mod model;

pub use client::{AsRadioModelName, InfoResult, ListResult, RadioCodeCalculator};
pub use error::{RadioCodeCalculatorError, RadioErrors};
pub use model::{RadioModel, RadioModels};
