<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Configuration

DOSovoi uses a deliberately small `key = value` format. It has no include,
command-substitution, network, or executable syntax. Unknown keys and invalid
values are errors so a misspelled safety-relevant preference does not disappear
silently.

```text
enabled = true
mascot = bunny
render = ascii
color = false
motion = false
default_mode = serenity
care_plain_text = false
care_disable_color = true
```

`mascot` accepts `bunny`, `puff`, or `lens`. `render` accepts `ascii`, `unicode`,
or `plain`. `default_mode` accepts `serenity`, `flow`, `vigilance`,
`detachment`, or `care`. Booleans accept `true`/`false` and `on`/`off`.

`motion` is reserved and defaults off. Version 0.1.0 has no animation; setting
it on does not cause or falsely report motion. In care, `care_plain_text` can
remove art and `care_disable_color` can suppress ANSI colour. Motion remains
off in all cases.

The CLI stores runtime state beside the selected configuration as
`<config-path>.state`. It contains only enabled state, the selected mode, care
state, and an explicitly supplied risk level, reason, and typed source. The
current CLI source is `explicit-user-input`. It never contains observed terminal
content because DOSovoi does not observe terminal content. Clearing a signal
removes its reason and source. State files written before source provenance was
added remain readable and are interpreted as explicit CLI input. Delete or
relocate this state file only with the same care as any other local
configuration data.

There is no telemetry, network configuration, notification setting, or
credential field because those capabilities do not exist.
