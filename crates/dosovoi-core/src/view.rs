// SPDX-License-Identifier: MPL-2.0

use crate::{
    mascot_frame, AppState, Config, Mascot, MascotFrame, Mode, RenderStyle, RiskLevel, SignalSource,
};
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PresentationOptions {
    pub mascot: Mascot,
    pub style: RenderStyle,
    pub color: bool,
}

impl PresentationOptions {
    #[must_use]
    pub const fn from_config(config: &Config) -> Self {
        Self {
            mascot: config.mascot,
            style: config.render,
            color: config.color,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarningView {
    pub label: &'static str,
    pub reason: String,
    pub source: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewModel {
    pub warning: Option<WarningView>,
    pub mascot: Option<MascotFrame>,
    pub mode: Mode,
    pub care: bool,
    pub color: bool,
}

#[must_use]
pub fn build_view(
    config: &Config,
    state: &AppState,
    mut options: PresentationOptions,
) -> ViewModel {
    let enabled = config.enabled && state.enabled;
    let effective_mode = if state.care { Mode::Care } else { state.mode };
    if state.care {
        if config.care.plain_text {
            options.style = RenderStyle::Plain;
        }
        if config.care.disable_color {
            options.color = false;
        }
    }

    // High-risk information takes precedence over decoration. Descriptive
    // status remains available, but mascot art is suppressed.
    if state.risk.level == RiskLevel::High {
        options.style = RenderStyle::Plain;
    }

    let warning = match state.risk.level {
        RiskLevel::None => None,
        RiskLevel::Caution => Some(WarningView {
            label: "CAUTION",
            reason: state.risk.reason.clone().unwrap_or_default(),
            source: signal_source_label(state.risk.source),
        }),
        RiskLevel::High => Some(WarningView {
            label: "HIGH RISK",
            reason: state.risk.reason.clone().unwrap_or_default(),
            source: signal_source_label(state.risk.source),
        }),
    };

    ViewModel {
        warning,
        mascot: enabled.then(|| mascot_frame(options.mascot, effective_mode, options.style)),
        mode: effective_mode,
        care: state.care,
        color: options.color,
    }
}

const fn signal_source_label(source: Option<SignalSource>) -> &'static str {
    match source {
        Some(SignalSource::ExplicitUserInput) => "explicit user input",
        None => "unspecified",
    }
}

#[must_use]
pub fn render_text(view: &ViewModel) -> String {
    let mut output = String::new();

    if let Some(warning) = &view.warning {
        if view.color {
            let code = if warning.label == "HIGH RISK" {
                "31"
            } else {
                "33"
            };
            let _ = writeln!(
                output,
                "\u{1b}[1;{code}mWARNING [{}]\u{1b}[0m: {}",
                warning.label, warning.reason
            );
        } else {
            let _ = writeln!(output, "WARNING [{}]: {}", warning.label, warning.reason);
        }
        let _ = writeln!(output, "Signal source: {}", warning.source);
    } else {
        output.push_str("Risk signal: none supplied; no safety assessment performed.\n");
    }

    output.push('\n');

    if let Some(mascot) = &view.mascot {
        if !mascot.art.is_empty() {
            let _ = writeln!(output, "{}", mascot.art);
        }
        let _ = writeln!(output, "Mascot status: {}", mascot.alt_text);
    } else {
        let _ = writeln!(output, "Mascot status: disabled.");
    }

    let _ = writeln!(output, "Mode: {}", view.mode);
    let _ = writeln!(output, "Care: {}", if view.care { "on" } else { "off" });
    // Motion is intentionally unimplemented, including when configured on.
    output.push_str("Motion: off\n");
    if view.mode == Mode::Serenity {
        output.push_str("Note: Serenity is a quiet presentation, not a safety assessment.\n");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Event, RiskSignal};

    #[test]
    fn warning_survives_every_mode_and_care_transition() {
        let config = Config::default();
        let mut state = AppState::from_config(&config);
        state.apply(Event::SetRisk(
            RiskSignal::new(RiskLevel::High, Some("Named external signal".into())).unwrap(),
        ));
        for &mode in Mode::ALL {
            state.apply(Event::SetMode(mode));
            for care in [false, true] {
                state.apply(Event::SetCare(care));
                let view = build_view(&config, &state, PresentationOptions::from_config(&config));
                assert_eq!(view.warning.as_ref().unwrap().label, "HIGH RISK");
                assert!(render_text(&view).starts_with("WARNING [HIGH RISK]"));
            }
        }
    }

    #[test]
    fn warning_is_independent_of_mascot_selection() {
        let config = Config::default();
        let mut state = AppState::from_config(&config);
        state.apply(Event::SetRisk(
            RiskSignal::new(RiskLevel::Caution, Some("Review this input".into())).unwrap(),
        ));
        for &mascot in Mascot::ALL {
            let options = PresentationOptions {
                mascot,
                style: RenderStyle::Ascii,
                color: false,
            };
            assert!(build_view(&config, &state, options).warning.is_some());
        }
    }

    #[test]
    fn no_color_rendering_has_no_terminal_escapes() {
        let config = Config::default();
        let mut state = AppState::from_config(&config);
        state.apply(Event::SetRisk(
            RiskSignal::new(RiskLevel::Caution, Some("Review".into())).unwrap(),
        ));
        let output = render_text(&build_view(
            &config,
            &state,
            PresentationOptions::from_config(&config),
        ));
        assert!(!output.contains('\u{1b}'));
        assert!(output.contains("WARNING [CAUTION]: Review"));
    }

    #[test]
    fn detachment_contains_no_reengagement_prompt() {
        let config = Config::default();
        let mut state = AppState::from_config(&config);
        state.apply(Event::SetMode(Mode::Detachment));
        let output = render_text(&build_view(
            &config,
            &state,
            PresentationOptions::from_config(&config),
        ));
        for forbidden in ["come back", "return soon", "missed you", "miss you"] {
            assert!(!output.to_ascii_lowercase().contains(forbidden));
        }
    }

    #[test]
    fn every_unsignalled_mode_names_the_evidence_boundary() {
        let config = Config::default();
        let mut state = AppState::from_config(&config);
        for &mode in Mode::ALL {
            state.apply(Event::SetMode(mode));
            let output = render_text(&build_view(
                &config,
                &state,
                PresentationOptions::from_config(&config),
            ));
            assert!(output
                .starts_with("Risk signal: none supplied; no safety assessment performed.\n\n"));
        }
    }

    #[test]
    fn high_risk_names_source_and_suppresses_decorative_art() {
        let config = Config::default();
        let mut state = AppState::from_config(&config);
        state.apply(Event::SetMode(Mode::Vigilance));
        state.apply(Event::SetRisk(
            RiskSignal::new(RiskLevel::High, Some("Named external condition".into())).unwrap(),
        ));
        let options = PresentationOptions {
            mascot: Mascot::Lens,
            style: RenderStyle::Unicode,
            color: false,
        };
        let view = build_view(&config, &state, options);
        assert_eq!(view.mascot.as_ref().unwrap().art, "");
        let output = render_text(&view);
        assert!(output.starts_with(
            "WARNING [HIGH RISK]: Named external condition\n\
             Signal source: explicit user input\n\n"
        ));
        assert!(output.contains("Attentive mascot in a warning-facing posture"));
        assert!(!output.contains("lens-bearing"));
    }
}
