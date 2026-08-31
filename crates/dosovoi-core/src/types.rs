// SPDX-License-Identifier: MPL-2.0

use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseValueError {
    kind: &'static str,
    value: String,
}

impl ParseValueError {
    #[must_use]
    pub fn new(kind: &'static str, value: impl Into<String>) -> Self {
        Self {
            kind,
            value: value.into(),
        }
    }
}

impl fmt::Display for ParseValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}: {:?}", self.kind, self.value)
    }
}

impl std::error::Error for ParseValueError {}

macro_rules! string_enum {
    ($name:ident, $kind:literal, {$($variant:ident => $text:literal),+ $(,)?}) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name { $($variant),+ }

        impl $name {
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];

            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self { $(Self::$variant => $text),+ }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = ParseValueError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value.to_ascii_lowercase().as_str() {
                    $($text => Ok(Self::$variant),)+
                    _ => Err(ParseValueError::new($kind, value)),
                }
            }
        }
    };
}

string_enum!(Mode, "mode", {
    Serenity => "serenity",
    Flow => "flow",
    Vigilance => "vigilance",
    Detachment => "detachment",
    Care => "care",
});

string_enum!(Mascot, "mascot", {
    Bunny => "bunny",
    Puff => "puff",
    Lens => "lens",
});

string_enum!(RenderStyle, "render style", {
    Ascii => "ascii",
    Unicode => "unicode",
    Plain => "plain",
});

string_enum!(RiskLevel, "risk level", {
    None => "clear",
    Caution => "caution",
    High => "high",
});

string_enum!(SignalSource, "signal source", {
    ExplicitUserInput => "explicit-user-input",
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiskSignal {
    pub level: RiskLevel,
    pub reason: Option<String>,
    pub source: Option<SignalSource>,
}

impl RiskSignal {
    /// Constructs a signal. Caution and high risk require an explicit reason;
    /// the unsignalled state never retains one.
    ///
    /// # Errors
    ///
    /// Returns [`ParseValueError`] when caution or high risk has no non-empty
    /// reason.
    pub fn new(level: RiskLevel, reason: Option<String>) -> Result<Self, ParseValueError> {
        if level == RiskLevel::None {
            return Ok(Self {
                level,
                reason: None,
                source: None,
            });
        }

        let reason = reason
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| ParseValueError::new("risk reason", "missing or empty"))?;
        Ok(Self {
            level,
            reason: Some(reason),
            source: Some(SignalSource::ExplicitUserInput),
        })
    }

    #[must_use]
    pub const fn clear() -> Self {
        Self {
            level: RiskLevel::None,
            reason: None,
            source: None,
        }
    }
}
