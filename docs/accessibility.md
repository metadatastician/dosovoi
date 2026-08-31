<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Accessibility and disengagement

Version 0.1.0 is static. Motion is always off even if the reserved configuration
field is enabled. There is no loop to quit: each command prints once and exits.
`dosovoi disable` locally suppresses mascot output, and normal shell interruption
remains available.

## Available paths

- `NO_COLOR` overrides colour configuration.
- `--no-color` disables colour for a preview.
- `--ascii` restricts art to ASCII.
- `--plain` removes art and retains descriptive status text.
- `care_plain_text` and `care_disable_color` reduce care-mode stimulation.
- Warning level and reason are literal text, appear before decoration, and do
  not rely on colour, expression, or animation.
- Unsignalled views explicitly state that no signal or assessment was supplied;
  supplied warnings name their source.
- High-risk views suppress art while preserving status text.
- Each frame has descriptive status text suitable for linear reading.

## Known limits

No testing with screen-reader users, low-vision users, neurodivergent users, or
people with motion sensitivity has occurred. Terminal/reader combinations may
announce punctuation-heavy art poorly; plain mode is the current fallback, not
proof of accessibility. ANSI colour detection is manual except for `NO_COLOR`.
The prototype has no localisation, width adaptation, or formal WCAG assessment.

Evaluation must measure time to disable and leave, warning comprehension without
art/colour, reading order, interruption cost, distraction, and whether Care
actually reduces rather than adds burden. Adverse feedback must be able to stop
a session immediately without requiring explanation.
