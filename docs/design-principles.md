<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Draft design principles

These are prototype design constraints and hypotheses, not a safety standard or
clinical guidance.

## Serenity

Serenity is the quiet default presentation. It minimises decoration and motion;
it never means “safe”, “checked”, or “trusted”. Treating serenity as a useful
baseline is a design hypothesis requiring evaluation.

## Flow

Flow is entered explicitly and uses a compact, low-distraction static frame.
The prototype does not infer focus, productivity, or cognitive state. “Flow as
aspiration” is a design hypothesis, not a promised outcome.

## Vigilance

Vigilance directs visual attention toward a separately supplied caution signal.
It neither discovers nor evaluates hazards. Warning text comes first, remains
legible without the mascot, and names the supplied reason and signal source.
High-risk presentation suppresses mascot art so decoration cannot compete with
the warning.

## Detachment

Detachment supports stopping and leaving. Its output contains no return prompt,
guilt, sadness, dependency, streak, reward, or engagement bait. Because the CLI
is one-shot, normal process exit is immediate and quiet.

## Care

Care is an explicit overlay entered by `dosovoi care on` (or `mode care`). It
uses the Care static frame and can force plain text and no colour. It does not
hide or downgrade an active warning. `care off` restores the underlying selected
mode. Care is an accessibility and disengagement control, not treatment.

All transitions are explicit and deterministic. Risk is not inferred from
terminal text, and no mascot mode changes risk.
