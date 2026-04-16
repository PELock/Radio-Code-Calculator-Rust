//! Radio Code Calculator API module
//!
//! Usage:
//!
//! ```ignore
//! my_radio_code_calculator = RadioCodeCalculator::new(Some("YOUR-WEB-API-KEY".into()));
//!
//! // generate radio code (using Web API)
//! my_radio_code_calculator.calc(&RadioModels::FORD_M_SERIES, "123456").await?;
//!
//!     println!("Radio code is {}", result["code"]);
//!
//! }
//! ```

use std::collections::HashMap;

use reqwest::Client;
use serde_json::{json, Value};

use crate::error::{RadioCodeCalculatorError, RadioErrors};
use crate::model::RadioModel;

/// Return type for [`RadioCodeCalculator::info`].
#[derive(Debug, Clone)]
pub struct InfoResult {
    pub value: Value,
    pub radio_model: RadioModel,
}

/// Return type for [`RadioCodeCalculator::list`].
#[derive(Debug, Clone)]
pub struct ListResult {
    pub value: Value,
    pub radio_models: Vec<RadioModel>,
}

/// Trait for values accepted as a radio model identifier ([`RadioModel`] or model name string).
pub trait AsRadioModelName {
    fn radio_model_name(&self) -> &str;
}

impl AsRadioModelName for RadioModel {
    fn radio_model_name(&self) -> &str {
        &self.name
    }
}

impl AsRadioModelName for &'_ RadioModel {
    fn radio_model_name(&self) -> &str {
        &self.name
    }
}

impl AsRadioModelName for str {
    fn radio_model_name(&self) -> &str {
        self
    }
}

impl AsRadioModelName for String {
    fn radio_model_name(&self) -> &str {
        self.as_str()
    }
}

/// Radio Code Calculator API module
pub struct RadioCodeCalculator {
    /// @var string default Radio Code Calculator API WebApi endpoint
    pub api_url: String,

    /// @var string|null WebApi key for the service
    _api_key: Option<String>,

    client: Client,
}

impl RadioCodeCalculator {
    /// Default Radio Code Calculator API WebApi endpoint
    pub const DEFAULT_API_URL: &'static str = "https://www.pelock.com/api/radio-code-calculator/v1";

    /// Initialize Radio Code Calculator API class
    ///
    /// @param string|null api_key Activation key for the service (it cannot be empty!)
    pub fn new(api_key: Option<String>) -> Self {
        Self::with_client(api_key, Client::new())
    }

    /// Construct with a custom [`reqwest::Client`] (timeouts, proxies, etc.).
    pub fn with_client(api_key: Option<String>, client: Client) -> Self {
        Self {
            api_url: Self::DEFAULT_API_URL.to_string(),
            _api_key: api_key,
            client,
        }
    }

    /// Login to the service and get the information about the current license limits
    ///
    /// @return RadioCodeCalculator A list with an error code, and an optional dictionary with the raw results (or null on error)
    pub async fn login(&self) -> Result<Value, RadioCodeCalculatorError> {
        let mut params = HashMap::new();
        params.insert("command".to_string(), "login".to_string());
        self.post_request(params).await
    }

    /// Calculate the radio code for the selected radio model
    ///
    /// @param RadioModel|string radio_model Radio model either as a RadioModel class or a string
    /// @param string radio_serial_number Radio serial number / pre code
    /// @param string radio_extra_data Optional extra data (for example - a supplier code) to generate the radio code
    /// @return array A list with an error code, and an optional dictionary with the raw results (or null)
    pub async fn calc<R: AsRadioModelName + ?Sized>(
        &self,
        radio_model: &R,
        radio_serial_number: &str,
        radio_extra_data: &str,
    ) -> Result<Value, RadioCodeCalculatorError> {
        let mut params = HashMap::new();
        params.insert("command".to_string(), "calc".to_string());
        params.insert(
            "radio_model".to_string(),
            radio_model.radio_model_name().to_string(),
        );
        params.insert("serial".to_string(), radio_serial_number.to_string());
        params.insert("extra".to_string(), radio_extra_data.to_string());
        self.post_request(params).await
    }

    /// Get the information about the given radio calculator and its parameters (name, max. len & regex pattern)
    ///
    /// @param RadioModel|string radio_model Radio model either as a RadioModel class or a string
    /// @return array A list with an error code, and an optional RadioModel create from the return values (or null)
    pub async fn info<R: AsRadioModelName + ?Sized>(
        &self,
        radio_model: &R,
    ) -> Result<InfoResult, RadioCodeCalculatorError> {
        let mut params = HashMap::new();
        params.insert("command".to_string(), "info".to_string());
        let name = radio_model.radio_model_name().to_string();
        params.insert("radio_model".to_string(), name.clone());

        let mut value = self.post_request(params).await?;
        let model = radio_model_from_response(&name, &value).ok_or_else(|| {
            RadioCodeCalculatorError::Transport(
                "missing radio model fields in info response".into(),
            )
        })?;
        value["radioModel"] = json_model_summary(&model);
        Ok(InfoResult {
            value,
            radio_model: model,
        })
    }

    /// List all the supported radio calculators and their parameters (name, max. len & regex pattern)
    ///
    /// @return array A list with an error code, and an optional list of supported RadioModels (or null)
    pub async fn list(&self) -> Result<ListResult, RadioCodeCalculatorError> {
        let mut params = HashMap::new();
        params.insert("command".to_string(), "list".to_string());

        let mut value = self.post_request(params).await?;
        let supported = value
            .get("supportedRadioModels")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        let mut radio_models = Vec::new();
        for (radio_model_name, obj) in supported {
            if let Some(model) = radio_model_from_response(&radio_model_name, &obj) {
                radio_models.push(model);
            }
        }

        value["radioModels"] = json!(radio_models
            .iter()
            .map(json_model_summary)
            .collect::<Vec<_>>());

        Ok(ListResult {
            value,
            radio_models,
        })
    }

    /// Send a POST request to the server & returns a Promise
    ///
    /// @param {Array} params_array params_array An array with the parameters
    /// @param {decodedCallback} callback_ Funkcja callback wywolywana po zdekodowaniu danych
    /// @returns {Promise} An array with the POST request results (or default error)
    pub async fn post_request(
        &self,
        params_array: HashMap<String, String>,
    ) -> Result<Value, RadioCodeCalculatorError> {
        // default error -> only returned by the SDK
        // return error if the activation key is not set (no demo version)
        let Some(key) = &self._api_key else {
            return Err(RadioCodeCalculatorError::InvalidLicense);
        };

        let mut form = reqwest::multipart::Form::new();
        form = form.text("key", key.clone());

        for (param, val) in params_array {
            form = form.text(param, val);
        }

        let response = self
            .client
            .post(&self.api_url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| RadioCodeCalculatorError::Transport(e.to_string()))?;

        let value: Value = response
            .json()
            .await
            .map_err(|e| RadioCodeCalculatorError::Transport(e.to_string()))?;

        let err = value
            .get("error")
            .and_then(|e| e.as_i64())
            .unwrap_or(RadioErrors::ERROR_CONNECTION as i64);

        if err == RadioErrors::SUCCESS as i64 {
            Ok(value)
        } else {
            Err(RadioCodeCalculatorError::ApiError(value))
        }
    }
}

fn radio_model_from_response(name: &str, v: &Value) -> Option<RadioModel> {
    let serial_max_len = v.get("serialMaxLen")?.as_u64()? as usize;
    let serial_regex = v.get("serialRegexPattern")?.clone();
    let extra_max_len = v.get("extraMaxLen").and_then(|x| x.as_u64()).unwrap_or(0) as usize;
    let extra_regex = if extra_max_len == 0 {
        None
    } else {
        v.get("extraRegexPattern").cloned()
    };
    Some(RadioModel::new(
        name,
        serial_max_len,
        serial_regex,
        extra_max_len,
        extra_regex,
    ))
}

fn json_model_summary(m: &RadioModel) -> Value {
    json!({
        "name": m.name,
        "serialMaxLen": m.serial_max_len,
        "extraMaxLen": m.extra_max_len,
        "serialRegexPattern": m.serial_regex_pattern(),
        "extraRegexPattern": m.extra_regex_pattern(),
    })
}
