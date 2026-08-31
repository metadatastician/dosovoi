<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# DOSovoi

DOSovoi is an early, opt-in, local-first human-factors experiment: small,
deterministic terminal mascots for calm attention, explicit vigilance, focused
work, breaks, and rapid disengagement. Invoking the one-shot command is the
opt-in; there is no daemon, shell hook, background process, telemetry, model, or
network feature.

The name combines “DOS” (disk operating system) with “Domovoi”. Domovoi comes
from Slavic folklore. This project is wordplay, not an authoritative portrayal
of Slavic belief, and its prototype creature choices await cultural review.

## Status

Version 0.1.0 is a working vertical-slice prototype. It provides three static
mascots, five presentation modes, an explicit and separately typed risk signal,
care on/off, immediate disabling, ASCII/Unicode/plain rendering, local
configuration and state, and automated invariant and CLI tests.

Every unsupplied-risk view states that no signal or safety assessment was
provided. Caution and high-risk views name explicit user input as their source
and are separated from decoration. High-risk views suppress mascot art while
retaining descriptive status text.

The first internal presentation review is recorded in
[`docs/audits/2026-08-18/`](docs/audits/2026-08-18/README.md). It identifies
false-reassurance and accessibility questions; it is not cultural consultation
or human-subject evidence.

It does not inspect commands or terminal contents. A warning appears only after
the user supplies a signal explicitly. Serenity describes visual quietness; it
does not mean that a command, output, model, or environment is safe.

## Quick start

Stable Rust 1.80 or newer is required.

```sh
cargo run -p dosovoi-cli -- help
cargo run -p dosovoi-cli -- preview
cargo run -p dosovoi-cli -- preview --pet bunny --mode serenity
cargo run -p dosovoi-cli -- mode flow
cargo run -p dosovoi-cli -- care on
cargo run -p dosovoi-cli -- signal caution --reason "Review generated command before execution"
cargo run -p dosovoi-cli -- status
cargo run -p dosovoi-cli -- signal clear
cargo run -p dosovoi-cli -- disable
```

Every invocation renders once and exits. `disable` suppresses the mascot
immediately. If a stored warning exists, explicitly running DOSovoi still shows
that warning before reporting that the mascot is disabled; disabling decoration
does not silently clear risk state.

## Configuration and accessibility

Use `dosovoi config init` to create a configuration without overwriting an
existing file. Resolution order is `--config PATH`, `DOSOVOI_CONFIG`,
`$XDG_CONFIG_HOME/dosovoi/config`, then `$HOME/.config/dosovoi/config`. See
[`dosovoi.example.conf`](dosovoi.example.conf) and
[`docs/configuration.md`](docs/configuration.md).

The default is static ASCII without colour or motion. `NO_COLOR` always wins
over configuration. `--plain` emits descriptive status without mascot art;
`--ascii` avoids Unicode. No essential warning depends on colour or expression.

## Architectural boundary

`dosovoi-core` owns typed deterministic state and renderer-neutral views.
`dosovoi-cli` parses explicit arguments, reads small local files, and renders
text. Risk and mascot presentation are separate subsystems; see
[`docs/adr/0001-separate-risk-and-mascot.md`](docs/adr/0001-separate-risk-and-mascot.md).
There is an event boundary suitable for a future adapter, but no Wokelang or AI
integration is claimed or implemented.

## Explicit non-claims

DOSovoi is not an AI alignment or safety solution, an agent, a mental-health
treatment, a supernatural system, or evidence that an operation is safe. This
prototype does not claim to change AI cognition, goals, humility, agency, or
alignment; prevent agentic drift; regulate dopamine; clinically reduce anxiety;
offer neuroscience, NICE, or universal cross-cultural validation; reduce
P(doom); extend an AI-risk timeline; or replace warnings, verification, access
control, sandboxing, and review.

The complete evidence boundary is in
[`docs/evidence-boundary.md`](docs/evidence-boundary.md). Design intentions are
not validation results.

## Development

```sh
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

Source code is MPL-2.0. Original documentation is CC-BY-SA-4.0. Per-file SPDX
identifiers and [`LICENSE`](LICENSE) state the applicable terms.
