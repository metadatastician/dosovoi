// SPDX-License-Identifier: MPL-2.0

use crate::{Mascot, Mode, RenderStyle};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CarePreferences {
    pub plain_text: bool,
    pub disable_color: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub enabled: bool,
    pub mascot: Mascot,
    pub render: RenderStyle,
    pub color: bool,
    pub motion: bool,
    pub default_mode: Mode,
    pub care: CarePreferences,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            mascot: Mascot::Bunny,
            render: RenderStyle::Ascii,
            color: false,
            motion: false,
            default_mode: Mode::Serenity,
            care: CarePreferences {
                plain_text: false,
                disable_color: true,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    line: usize,
    message: String,
}

impl ConfigError {
    fn at(line: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "configuration line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    /// Parses a deliberately small `key = value` format. Unknown keys fail
    /// visibly instead of being silently ignored.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] when a non-comment line is malformed, a key is
    /// unknown, or a value is invalid for its field.
    pub fn parse(input: &str) -> Result<Self, ConfigError> {
        let mut config = Self::default();
        for (index, raw_line) in input.lines().enumerate() {
            let line_number = index + 1;
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| ConfigError::at(line_number, "expected key = value"))?;
            let key = key.trim();
            let value = value.trim();
            match key {
                "enabled" => config.enabled = parse_bool(value, line_number)?,
                "mascot" => config.mascot = parse_value(value, line_number)?,
                "render" => config.render = parse_value(value, line_number)?,
                "color" => config.color = parse_bool(value, line_number)?,
                "motion" => config.motion = parse_bool(value, line_number)?,
                "default_mode" => config.default_mode = parse_value(value, line_number)?,
                "care_plain_text" => config.care.plain_text = parse_bool(value, line_number)?,
                "care_disable_color" => {
                    config.care.disable_color = parse_bool(value, line_number)?;
                }
                _ => {
                    return Err(ConfigError::at(line_number, format!("unknown key {key:?}")));
                }
            }
        }
        Ok(config)
    }

    #[must_use]
    pub fn example() -> &'static str {
        "# DOSovoi configuration\n\
enabled = true\n\
mascot = bunny\n\
render = ascii\n\
color = false\n\
motion = false\n\
default_mode = serenity\n\
care_plain_text = false\n\
care_disable_color = true\n"
    }
}

fn parse_bool(value: &str, line: usize) -> Result<bool, ConfigError> {
    match value {
        "true" | "on" => Ok(true),
        "false" | "off" => Ok(false),
        _ => Err(ConfigError::at(
            line,
            format!("expected true/false or on/off, got {value:?}"),
        )),
    }
}

fn parse_value<T>(value: &str, line: usize) -> Result<T, ConfigError>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    value
        .parse()
        .map_err(|error: T::Err| ConfigError::at(line, error.to_string()))
}
