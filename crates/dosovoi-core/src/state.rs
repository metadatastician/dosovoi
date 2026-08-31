// SPDX-License-Identifier: MPL-2.0

use crate::{Config, Mode, ParseValueError, RiskLevel, RiskSignal, SignalSource};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppState {
    pub enabled: bool,
    pub mode: Mode,
    pub care: bool,
    pub risk: RiskSignal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    SetEnabled(bool),
    SetMode(Mode),
    SetCare(bool),
    SetRisk(RiskSignal),
}

#[derive(Debug)]
pub enum StateError {
    InvalidLine(usize),
    UnknownKey { line: usize, key: String },
    InvalidValue { line: usize, message: String },
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLine(line) => write!(f, "state line {line}: expected key = value"),
            Self::UnknownKey { line, key } => {
                write!(f, "state line {line}: unknown key {key:?}")
            }
            Self::InvalidValue { line, message } => {
                write!(f, "state line {line}: {message}")
            }
        }
    }
}

impl std::error::Error for StateError {}

impl AppState {
    #[must_use]
    pub fn from_config(config: &Config) -> Self {
        Self {
            enabled: config.enabled,
            mode: config.default_mode,
            care: false,
            risk: RiskSignal::clear(),
        }
    }

    pub fn apply(&mut self, event: Event) {
        match event {
            Event::SetEnabled(enabled) => self.enabled = enabled,
            Event::SetMode(mode) => self.mode = mode,
            Event::SetCare(care) => self.care = care,
            Event::SetRisk(risk) => self.risk = risk,
        }
    }

    #[must_use]
    pub fn serialize(&self) -> String {
        let reason = self.risk.reason.as_deref().map(escape).unwrap_or_default();
        let source = self.risk.source.map_or("", SignalSource::as_str);
        format!(
            "enabled = {}\nmode = {}\ncare = {}\nrisk = {}\nreason = {}\nsource = {}\n",
            self.enabled, self.mode, self.care, self.risk.level, reason, source
        )
    }

    /// Parses persisted runtime state, using configuration for defaults.
    ///
    /// # Errors
    ///
    /// Returns [`StateError`] for malformed lines, unknown keys, invalid
    /// values, or caution/high risk without a non-empty reason.
    pub fn parse(input: &str, config: &Config) -> Result<Self, StateError> {
        let mut state = Self::from_config(config);
        let mut reason = None;
        let mut source = None;
        for (index, raw_line) in input.lines().enumerate() {
            let line_number = index + 1;
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (key, value) = line
                .split_once('=')
                .ok_or(StateError::InvalidLine(line_number))?;
            let key = key.trim();
            let value = value.trim();
            match key {
                "enabled" => state.enabled = parse_bool(value, line_number)?,
                "mode" => state.mode = parse_value(value, line_number)?,
                "care" => state.care = parse_bool(value, line_number)?,
                "risk" => state.risk.level = parse_value(value, line_number)?,
                "reason" => reason = Some(unescape(value, line_number)?),
                "source" if value.is_empty() => source = None,
                "source" => source = Some(parse_value(value, line_number)?),
                _ => {
                    return Err(StateError::UnknownKey {
                        line: line_number,
                        key: key.to_owned(),
                    });
                }
            }
        }
        state.risk = RiskSignal::new(state.risk.level, reason).map_err(|error| {
            StateError::InvalidValue {
                line: 0,
                message: error.to_string(),
            }
        })?;
        if state.risk.level != RiskLevel::None {
            state.risk.source = source.or(Some(SignalSource::ExplicitUserInput));
        }
        Ok(state)
    }
}

fn parse_bool(value: &str, line: usize) -> Result<bool, StateError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(StateError::InvalidValue {
            line,
            message: format!("invalid boolean {value:?}"),
        }),
    }
}

fn parse_value<T>(value: &str, line: usize) -> Result<T, StateError>
where
    T: FromStr<Err = ParseValueError>,
{
    value
        .parse()
        .map_err(|error: ParseValueError| StateError::InvalidValue {
            line,
            message: error.to_string(),
        })
}

fn escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn unescape(value: &str, line: usize) -> Result<String, StateError> {
    let mut result = String::new();
    let mut chars = value.chars();
    while let Some(character) = chars.next() {
        if character != '\\' {
            result.push(character);
            continue;
        }
        match chars.next() {
            Some('n') => result.push('\n'),
            Some('r') => result.push('\r'),
            Some('\\') => result.push('\\'),
            Some(other) => {
                return Err(StateError::InvalidValue {
                    line,
                    message: format!("invalid escape sequence \\{other}"),
                });
            }
            None => {
                return Err(StateError::InvalidValue {
                    line,
                    message: "trailing escape character".to_owned(),
                });
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RiskLevel;

    #[test]
    fn state_round_trip_preserves_explicit_reason() {
        let config = Config::default();
        let state = AppState {
            enabled: true,
            mode: Mode::Vigilance,
            care: true,
            risk: RiskSignal::new(RiskLevel::High, Some("line 1\nline \\ 2".into()))
                .expect("valid signal"),
        };
        assert_eq!(AppState::parse(&state.serialize(), &config).unwrap(), state);
    }

    #[test]
    fn pre_provenance_state_defaults_to_explicit_cli_input() {
        let config = Config::default();
        let legacy = "enabled = true\nmode = vigilance\ncare = false\n\
                      risk = caution\nreason = Review this\n";
        let state = AppState::parse(legacy, &config).unwrap();
        assert_eq!(state.risk.source, Some(SignalSource::ExplicitUserInput));
        assert!(state.serialize().contains("source = explicit-user-input\n"));
    }
}
