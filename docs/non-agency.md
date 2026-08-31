<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Non-agency architecture and threat model

DOSovoi mascots are deterministic interface components. They are not agents,
sub-agents, assistants, companions with simulated needs, or safety monitors.

## Implemented boundary

The core accepts only explicit typed values: mode, mascot, render style, care,
enabled state, and a risk signal with a reason. A pure view-building step maps
those values to fixed art and status text. It has no I/O.

The CLI may read its selected configuration/state files and write its state
file after an explicit mutating command. It may read `NO_COLOR` and standard
configuration-location environment variables. It does not read stdin, command
history, shell output, the terminal screen, unrelated files, or credentials.

There is no:

- model inference, planning, self-modification, or personality memory;
- autonomous tool or arbitrary command execution;
- web or network access, telemetry, or notification service;
- hidden model channel or provider integration;
- daemon, shell hook, plugin runtime, or background process;
- random selection, animation, reward, streak, or engagement scheduler.

## Threats and controls

**False reassurance.** A calm frame could be misread as a safety judgement.
Every unsignalled view explicitly says that no signal was supplied and no
safety assessment was performed. Warning state is separate, textual, first,
and independent of mascot state. High-risk views suppress mascot art. These
controls still require evaluation with people.

**Signal spoofing or stale signals.** DOSovoi does not authenticate the source
or current accuracy of a user-supplied reason. The interface labels only its
level, reason, and provenance; it never says it performed a check. The current
CLI source is typed and rendered as `explicit user input`. Users must clear
stale signals explicitly.

**Sensitive reason persistence.** An explicitly entered reason is stored in a
local state file. Users should avoid putting secrets in it. No terminal content
is captured automatically.

**Adapter expansion.** A future adapter must supply the same minimal typed
events, identify its source and limitations, remain inspectable, and preserve
risk precedence. No Wokelang contract is invented here; integration requires an
actual specification and a separate decision record.

**Dependency or network creep.** Both crates currently have no third-party
dependencies. Any capability that adds inference, networking, command
observation, autonomy, or telemetry is outside this prototype boundary and
requires explicit review rather than an incidental patch.
