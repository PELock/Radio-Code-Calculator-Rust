//! RadioModel class used to calculate the radio code for specified car radio/navigation
//!
//! Usage:
//!
//! ```ignore
//! // create Radio Code Calculator API class instance (we are using our activation key)
//! let my_radio_code_calculator = RadioCodeCalculator::new(Some("ABCD-ABCD-ABCD-ABCD".into()));
//!
//! // validate the serial number (offline) before sending the Web API request
//! let error = radio_model.validate(serial, extra);
//!
//! ...
//!
//! // generate radio code (using Web API)
//! my_radio_code_calculator.calc(&RadioModels::FORD_M_SERIES, "123456").await?;
//!
//!     println!("Radio code is {}", result["code"]);
//! ```

use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;
use serde_json::Value;

use crate::error::RadioErrors;

/// RadioModel class used to calculate the radio code for specified car radio/navigation
#[derive(Debug, Clone)]
pub struct RadioModel {
    /// @var string A single radio model with its parameters
    pub name: String,

    /// @var int Required, valid length of the radio serial/seed number
    pub serial_max_len: usize,

    /// @var array PCRE compatible regex patterns for the radio serial/seed number
    _serial_regex_patterns: HashMap<String, String>,

    /// @var int Length of the optional param for radio code generation
    pub extra_max_len: usize,

    /// @var array|null PCRE compatible regex patterns for the optional radio serial/seed number
    _extra_regex_patterns: Option<HashMap<String, String>>,

    /// @var string Default programming language used to determine the format of regular expression formats
    pub default_programming_language: String,
}

impl RadioModel {
    /// In JS you cannot pass the extra parameters for the RegExp in a single string
    /// This function splits the provided rule into Reg Exp rule & extra params (like case insensitive flag)
    ///
    /// @return RegExp regular expression
    pub fn regex_string_to_rule(regex_string: &str) -> Result<Regex, regex::Error> {
        let s = regex_string.trim();
        if !s.starts_with('/') {
            return Regex::new(s);
        }
        let inner = &s[1..];
        let Some(last_slash) = inner.rfind('/') else {
            return Regex::new(s);
        };
        let (pattern_src, flags) = inner.split_at(last_slash);
        let flags = &flags[1..];
        let mut b = regex::RegexBuilder::new(pattern_src);
        if flags.contains('i') {
            b.case_insensitive(true);
        }
        b.build()
    }

    /// Return the regex pattern for the current programming language only
    ///
    /// @return RegExp|string PCRE compatible regular expression or an empty string ""
    pub fn serial_regex_pattern(&self) -> String {
        self._serial_regex_patterns
            .get(&self.default_programming_language)
            .cloned()
            .unwrap_or_default()
    }

    /// Extra field (if defined) regex pattern for the current programming language only or null
    ///
    /// @return RegExp|null PCRE compatible regular expression or null if not required
    pub fn extra_regex_pattern(&self) -> Option<String> {
        let map = self._extra_regex_patterns.as_ref()?;
        map.get(&self.default_programming_language).cloned()
    }

    /// Initialize RadioModel class with the radio model name, serial & extra fields max. length and regex pattern
    ///
    /// @param string name Radio model name
    /// @param int serial_max_len Max. serial length
    /// @param string|array serial_regex_pattern Serial number single regex pattern or a dictionary
    /// @param int extra_max_len Max. extra field length
    /// @param string|array|null extra_regex_pattern: Extra field single regex pattern or a dictionary
    pub fn new(
        name: impl Into<String>,
        serial_max_len: usize,
        serial_regex_pattern: Value,
        extra_max_len: usize,
        extra_regex_pattern: Option<Value>,
    ) -> Self {
        let name = name.into();
        // create an empty dict to prevent Python re-using previous dict from previous object (!)
        let mut _serial_regex_patterns: HashMap<String, String> = HashMap::new();

        let default_programming_language = "js".to_string();

        // store the regex pattern under the key for the default programming language (compatibility)
        match &serial_regex_pattern {
            Value::String(s) => {
                _serial_regex_patterns.insert(default_programming_language.clone(), s.clone());
            }
            Value::Object(map) => {
                for (k, v) in map {
                    if let Some(s) = v.as_str() {
                        _serial_regex_patterns.insert(k.clone(), s.to_string());
                    }
                }
            }
            _ => {}
        }

        // initialize extra field
        let mut _extra_regex_patterns: Option<HashMap<String, String>> = None;

        if extra_max_len != 0 {
            let mut map = HashMap::new();
            match extra_regex_pattern {
                Some(Value::String(s)) => {
                    map.insert(default_programming_language.clone(), s);
                }
                Some(Value::Object(o)) => {
                    for (k, v) in o {
                        if let Some(s) = v.as_str() {
                            map.insert(k.clone(), s.to_string());
                        }
                    }
                }
                _ => {}
            }
            _extra_regex_patterns = Some(map);
        }

        Self {
            name,
            serial_max_len,
            _serial_regex_patterns,
            extra_max_len,
            _extra_regex_patterns,
            default_programming_language,
        }
    }

    /// Convenience for static presets using a single JS-style slash regex string.
    pub fn with_slash_patterns(
        name: impl Into<String>,
        serial_max_len: usize,
        serial_regex: &str,
        extra_max_len: usize,
        extra_regex: Option<&str>,
    ) -> Self {
        Self::new(
            name,
            serial_max_len,
            Value::String(serial_regex.to_string()),
            extra_max_len,
            extra_regex.map(|s| Value::String(s.to_string())),
        )
    }

    /// Validate radio serial number and extra data (if provided), check their lenghts and regex patterns
    ///
    /// @param string serial Radio serial number
    /// @param string|null extra: Extra data (optional)
    /// @return int one of the RadioErrors values
    pub fn validate(&self, serial: &str, extra: Option<&str>) -> i32 {
        if serial.len() != self.serial_max_len {
            return RadioErrors::INVALID_SERIAL_LENGTH;
        }

        let serial_pat = self.serial_regex_pattern();
        if serial_pat.is_empty() {
            return RadioErrors::INVALID_SERIAL_PATTERN;
        }
        let Ok(re) = Self::regex_string_to_rule(&serial_pat) else {
            return RadioErrors::INVALID_SERIAL_PATTERN;
        };
        if !re.is_match(serial) {
            return RadioErrors::INVALID_SERIAL_PATTERN;
        }

        if let Some(ex) = extra {
            if !ex.is_empty() {
                if ex.len() != self.extra_max_len {
                    return RadioErrors::INVALID_EXTRA_LENGTH;
                }
                let Some(extra_pat) = self.extra_regex_pattern() else {
                    return RadioErrors::INVALID_EXTRA_PATTERN;
                };
                let Ok(re_extra) = Self::regex_string_to_rule(&extra_pat) else {
                    return RadioErrors::INVALID_EXTRA_PATTERN;
                };
                if !re_extra.is_match(ex) {
                    return RadioErrors::INVALID_EXTRA_PATTERN;
                }
            }
        }

        RadioErrors::SUCCESS
    }
}

/// Supported radio models with the validation parameters (max. lengths & regex patterns)
///
/// This helper class can be used to quickly perform offline validation of the radio
/// serial/seed codes before its send to the WebApi.
///
/// Usage:
///
/// let radioModel = RadioModels::FORD_M_SERIES
#[allow(non_snake_case)]
pub mod RadioModels {
    use super::{LazyLock, RadioModel};

    macro_rules! lazy_model {
        ($name:ident, $slug:literal, $len:expr, $pat:literal) => {
            /// Supported radio model preset (lazy-initialized).
            pub static $name: LazyLock<RadioModel> =
                LazyLock::new(|| RadioModel::with_slash_patterns($slug, $len, $pat, 0, None));
        };
    }

    lazy_model!(RENAULT_DACIA, "renault-dacia", 4, "/^([A-Z]{1}[0-9]{3})$/");
    lazy_model!(
        CHRYSLER_PANASONIC_TM9,
        "chrysler-panasonic-tm9",
        4,
        "/^([0-9]{4})$/"
    );
    lazy_model!(
        CHRYSLER_DODGE_VP,
        "chrysler-dodge-vp",
        4,
        "/^([a-zA-Z0-9]{4})$/"
    );
    lazy_model!(FORD_M_SERIES, "ford-m-series", 6, "/^([0-9]{6})$/");
    lazy_model!(FORD_V_SERIES, "ford-v-series", 6, "/^([0-9]{6})$/");
    lazy_model!(FORD_TRAVELPILOT, "ford-travelpilot", 7, "/^([0-9]{7})$/");
    lazy_model!(
        FIAT_STILO_BRAVO_VISTEON,
        "fiat-stilo-bravo-visteon",
        6,
        "/^([a-zA-Z0-9]{6})$/"
    );
    lazy_model!(FIAT_DAIICHI, "fiat-daiichi", 4, "/^([0-9]{4})$/");
    lazy_model!(FIAT_VP, "fiat-vp", 4, "/^([0-9]{4})$/");
    lazy_model!(TOYOTA_ERC, "toyota-erc", 16, "/^([a-zA-Z0-9]{16})$/");
    lazy_model!(
        JEEP_CHEROKEE,
        "jeep-cherokee",
        14,
        "/^([a-zA-Z0-9]{10}[0-9]{4})$/"
    );
    lazy_model!(
        NISSAN_GLOVE_BOX,
        "nissan-glove-box",
        12,
        "/^([a-zA-Z0-9]{12})$/"
    );
    lazy_model!(ECLIPSE_ESN, "eclipse-esn", 6, "/^([a-zA-Z0-9]{6})$/");
    lazy_model!(JAGUAR_ALPINE, "jaguar-alpine", 5, "/^([0-9]{5})$/");
}
