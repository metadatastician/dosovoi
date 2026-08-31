// SPDX-License-Identifier: MPL-2.0

use crate::{Mascot, Mode, RenderStyle};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MascotFrame {
    pub art: &'static str,
    pub alt_text: String,
}

#[must_use]
pub fn mascot_frame(mascot: Mascot, mode: Mode, style: RenderStyle) -> MascotFrame {
    let art = match style {
        RenderStyle::Ascii => ascii_art(mascot, mode),
        RenderStyle::Unicode => unicode_art(mascot, mode),
        RenderStyle::Plain => "",
    };
    MascotFrame {
        art,
        alt_text: format!("{}.", description(mascot, mode)),
    }
}

const fn description(mascot: Mascot, mode: Mode) -> &'static str {
    match (mascot, mode) {
        (Mascot::Bunny, Mode::Serenity) => "Bunny-like mascot in a quiet, still posture",
        (Mascot::Bunny, Mode::Flow) => "Bunny-like mascot in a compact focus posture",
        (Mascot::Bunny, Mode::Vigilance) => "Bunny-like mascot facing the warning area",
        (Mascot::Bunny, Mode::Detachment) => "Bunny-like mascot in a resting posture",
        (Mascot::Bunny, Mode::Care) => "Bunny-like mascot in a low-stimulation posture",
        (Mascot::Puff, Mode::Serenity) => "Puff mascot in a quiet, still posture",
        (Mascot::Puff, Mode::Flow) => "Puff mascot in a compact focus posture",
        (Mascot::Puff, Mode::Vigilance) => "Puff mascot facing the warning area",
        (Mascot::Puff, Mode::Detachment) => "Puff mascot in a resting posture",
        (Mascot::Puff, Mode::Care) => "Puff mascot in a low-stimulation posture",
        (Mascot::Lens, Mode::Serenity) => "Attentive mascot in a quiet, still posture",
        (Mascot::Lens, Mode::Flow) => "Attentive mascot in a compact focus posture",
        (Mascot::Lens, Mode::Vigilance) => "Attentive mascot in a warning-facing posture",
        (Mascot::Lens, Mode::Detachment) => "Attentive mascot in a resting posture",
        (Mascot::Lens, Mode::Care) => "Attentive mascot in a low-stimulation posture",
    }
}

const fn ascii_art(mascot: Mascot, mode: Mode) -> &'static str {
    match (mascot, mode) {
        (Mascot::Bunny, Mode::Serenity | Mode::Care) => "(\\_/)\n( . .)\n(___)",
        (Mascot::Bunny, Mode::Flow) => "(\\_/)\n( o o)\n(___)",
        (Mascot::Bunny, Mode::Vigilance) => "(\\_/)\n( O O)\n(___)",
        (Mascot::Bunny, Mode::Detachment) => "(\\_/)\n( - -)\n(___)",
        (Mascot::Puff, Mode::Serenity) => " .--.\n( .. )\n `--'",
        (Mascot::Puff, Mode::Flow) => " .--.\n( oo )\n `--'",
        (Mascot::Puff, Mode::Vigilance) => " .--. !\n( OO )\n `--'",
        (Mascot::Puff, Mode::Detachment) => " .--.\n( -- )\n `--'",
        (Mascot::Puff, Mode::Care) => " .--.\n( .. )\n  ~~",
        (Mascot::Lens, Mode::Serenity) => "  /\\\n (o o)\n /|\\",
        (Mascot::Lens, Mode::Flow) => "  /\\\n (o o)\n /|_",
        (Mascot::Lens, Mode::Vigilance) => "  /\\\n (O O)\n /|\\",
        (Mascot::Lens, Mode::Detachment) => "  /\\\n (- ) .\n /|_",
        (Mascot::Lens, Mode::Care) => "  /\\\n (o ) .\n /|_",
    }
}

const fn unicode_art(mascot: Mascot, mode: Mode) -> &'static str {
    match (mascot, mode) {
        (Mascot::Bunny, Mode::Serenity | Mode::Care) => "╭ᵔᵔ╮\n│··│\n╰──╯",
        (Mascot::Bunny, Mode::Flow) => "╭ᵔᵔ╮\n│••│\n╰──╯",
        (Mascot::Bunny, Mode::Vigilance) => "╭ᵔᵔ╮\n│○○│\n╰──╯",
        (Mascot::Bunny, Mode::Detachment) => "╭ᵔᵔ╮\n│──│\n╰──╯",
        (Mascot::Puff, Mode::Serenity) => "  ☁\n · ·",
        (Mascot::Puff, Mode::Flow) => "  ☁\n • •",
        (Mascot::Puff, Mode::Vigilance) => "  ☁  !\n ○ ○",
        (Mascot::Puff, Mode::Detachment) => "  ☁\n ─ ─",
        (Mascot::Puff, Mode::Care) => "  ☁\n · ·  ≈",
        (Mascot::Lens, Mode::Serenity) => "  △\n (··)\n ╱│╲",
        (Mascot::Lens, Mode::Flow) => "  △\n (••)\n ╱│_",
        (Mascot::Lens, Mode::Vigilance) => "  △\n (○○)\n ╱│╲",
        (Mascot::Lens, Mode::Detachment) => "  △\n (─) ·\n ╱│_",
        (Mascot::Lens, Mode::Care) => "  △\n (·) ·\n ╱│_",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_mascot_has_every_static_mode_and_style() {
        for &mascot in Mascot::ALL {
            for &mode in Mode::ALL {
                for &style in RenderStyle::ALL {
                    let first = mascot_frame(mascot, mode, style);
                    let second = mascot_frame(mascot, mode, style);
                    assert_eq!(first, second);
                    assert!(!first.alt_text.is_empty());
                    if style == RenderStyle::Plain {
                        assert!(first.art.is_empty());
                    } else {
                        assert!(!first.art.is_empty());
                    }
                }
            }
        }
    }

    #[test]
    fn ascii_frames_are_ascii_only() {
        for &mascot in Mascot::ALL {
            for &mode in Mode::ALL {
                assert!(mascot_frame(mascot, mode, RenderStyle::Ascii)
                    .art
                    .is_ascii());
            }
        }
    }

    #[test]
    fn attentive_mascot_has_no_lens_probe_or_action_mark() {
        for &mode in Mode::ALL {
            let ascii = mascot_frame(Mascot::Lens, mode, RenderStyle::Ascii);
            let unicode = mascot_frame(Mascot::Lens, mode, RenderStyle::Unicode);
            for frame in [&ascii, &unicode] {
                assert!(frame.alt_text.starts_with("Attentive mascot"));
                assert!(!frame.alt_text.to_ascii_lowercase().contains("lens"));
                assert!(!frame.art.contains('!'));
                assert!(!frame.art.contains("-o"));
                assert!(!frame.art.contains("─○"));
            }
        }
    }

    #[test]
    fn status_copy_does_not_duplicate_mode_and_bunny_base_is_low_semantic() {
        for &mode in Mode::ALL {
            let bunny = mascot_frame(Mascot::Bunny, mode, RenderStyle::Ascii);
            assert!(!bunny.alt_text.contains("Mode:"));
            assert_eq!(bunny.art.lines().last(), Some("(___)"));
            assert!(!bunny.art.contains('>'));
        }
    }
}
