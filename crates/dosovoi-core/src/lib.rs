// SPDX-License-Identifier: MPL-2.0

//! Deterministic state, configuration, mascot definitions, and view models.
//! This crate performs no model inference, networking, command execution, or
//! terminal inspection.

mod config;
mod mascot;
mod state;
mod types;
mod view;

pub use config::{CarePreferences, Config, ConfigError};
pub use mascot::{mascot_frame, MascotFrame};
pub use state::{AppState, Event, StateError};
pub use types::{Mascot, Mode, ParseValueError, RenderStyle, RiskLevel, RiskSignal, SignalSource};
pub use view::{build_view, render_text, PresentationOptions, ViewModel, WarningView};
