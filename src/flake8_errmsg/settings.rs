//! Settings for the `flake8-errmsg` plugin.

use ruff_macros::ConfigurationOptions;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, PartialEq, Eq, Serialize, Deserialize, Default, ConfigurationOptions, JsonSchema,
)]
#[serde(
    deny_unknown_fields,
    rename_all = "kebab-case",
    rename = "Flake8ErrMsgOptions"
)]
pub struct Options {
    #[option(
        default = "0",
        value_type = "usize",
        example = "max-string-length = 20"
    )]
    /// Maximum string length for string literals in exception messages.
    pub max_string_length: Option<usize>,
}

#[derive(Debug, Default, Hash)]
pub struct Settings {
    pub max_string_length: usize,
}

impl Settings {
    #[allow(clippy::needless_pass_by_value)]
    pub fn from_options(options: Options) -> Self {
        Self {
            max_string_length: options.max_string_length.unwrap_or_default(),
        }
    }
}
