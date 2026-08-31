<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# ADR 0001: Separate risk signals from mascot presentation

- Status: Accepted for prototype
- Date: 2026-08-18

## Context

A mascot’s posture is decorative and may be interpreted differently by
different users. If it owns or infers risk, a calm frame can erase a warning or
be mistaken for evidence that a task is safe.

## Decision

Risk level and mascot mode are distinct Rust types and state fields. Risk enters
only through an explicit signal plus a required reason for caution/high levels.
View construction creates a separate warning view, and text rendering emits it
before mascot output. Mascot selection and mode/care transitions do not modify
risk. Disabling decoration does not clear a stored warning.

Future adapters must supply a typed signal through this boundary, identify the
real mechanism and limited result, and cannot encode risk solely in mascot
state. The prototype does not infer risk from terminal content.

## Consequences

- Calm presentation is never a programmatic synonym for low risk.
- Warnings remain understandable with art, Unicode, colour, and mascot output
  disabled.
- The CLI must expose explicit signal and clear operations.
- Stale or dishonest external signals remain possible; DOSovoi is not a signal
  validator or safety mechanism.
- Tests must exercise warning persistence across all presentation changes.

