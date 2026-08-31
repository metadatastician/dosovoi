// SPDX-License-Identifier: MPL-2.0

use dosovoi_core::{
    build_view, render_text, AppState, Config, Event, Mascot, Mode, PresentationOptions,
    RenderStyle, RiskLevel, RiskSignal,
};

#[test]
fn all_state_transitions_are_deterministic() {
    let config = Config::default();
    for &mode in Mode::ALL {
        let events = [
            Event::SetEnabled(false),
            Event::SetEnabled(true),
            Event::SetMode(mode),
            Event::SetCare(mode == Mode::Care),
            Event::SetRisk(
                RiskSignal::new(RiskLevel::Caution, Some("Explicit test signal".into()))
                    .expect("valid signal"),
            ),
            Event::SetRisk(RiskSignal::clear()),
        ];
        let mut left = AppState::from_config(&config);
        let mut right = AppState::from_config(&config);
        for event in events {
            left.apply(event.clone());
            right.apply(event);
        }
        assert_eq!(left, right);
    }
}

#[test]
fn invalid_inputs_are_rejected() {
    assert!("haste".parse::<Mode>().is_err());
    assert!("dragon".parse::<Mascot>().is_err());
    assert!("animated".parse::<RenderStyle>().is_err());
    assert!(RiskSignal::new(RiskLevel::High, None).is_err());
    assert!(RiskSignal::new(RiskLevel::Caution, Some("   ".into())).is_err());
}

#[test]
fn safe_defaults_are_local_static_and_uncoloured() {
    let config = Config::default();
    assert!(config.enabled);
    assert_eq!(config.render, RenderStyle::Ascii);
    assert!(!config.color);
    assert!(!config.motion);
    assert_eq!(config.default_mode, Mode::Serenity);
    assert!(config.care.disable_color);
}

#[test]
fn configuration_parses_and_unknown_keys_fail_visibly() {
    let config = Config::parse(
        "enabled = off\nmascot = lens\nrender = plain\ncolor = on\nmotion = off\n\
         default_mode = care\ncare_plain_text = on\ncare_disable_color = on\n",
    )
    .unwrap();
    assert!(!config.enabled);
    assert_eq!(config.mascot, Mascot::Lens);
    assert_eq!(config.render, RenderStyle::Plain);
    assert_eq!(config.default_mode, Mode::Care);
    assert!(Config::parse("telemetry = true").is_err());
}

#[test]
fn rendering_is_deterministic_and_plain_fallback_has_status_text() {
    let config = Config::default();
    let state = AppState::from_config(&config);
    let options = PresentationOptions {
        mascot: Mascot::Puff,
        style: RenderStyle::Plain,
        color: false,
    };
    let first = render_text(&build_view(&config, &state, options));
    let second = render_text(&build_view(&config, &state, options));
    assert_eq!(first, second);
    assert!(first.contains("Mascot status: Puff mascot"));
    assert!(!first.contains('\u{1b}'));
}
