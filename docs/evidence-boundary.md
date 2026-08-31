<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Evidence boundary

No external evidence review or human-subject study was conducted for version
0.1.0. This document therefore distinguishes tested implementation facts from
questions and rejected claims. It contains no manufactured citations.

## 1. Implemented behaviour

- Three selectable mascots have static ASCII and Unicode frames for all five
  modes, plus descriptive plain-text output.
- Typed state transitions and text rendering are deterministic in automated
  tests.
- Caution and high-risk signals require an explicit, non-empty user-supplied
  reason and retain typed provenance. DOSovoi does not infer risk.
- Warning output is generated separately, printed before mascot output, and
  persists across mode, mascot, care, and disable operations in tests.
- Every unsignalled view explicitly states that no signal was supplied and no
  safety assessment was performed. Supplied warnings name their source.
- High-risk views suppress mascot art while retaining descriptive status text.
- `NO_COLOR`, ASCII, plain-text, care, and immediate one-shot exit paths exist.
- The code has no dependencies and implements no network, model inference,
  telemetry, arbitrary command execution, animation, or terminal observation.

These statements describe reviewed code and passing tests, not an audit or
formal proof. They do not establish how people will perceive or use the tool.

## 2. Design hypotheses under investigation

- A quiet baseline may reduce distraction for some terminal users.
- Explicitly entering Flow may help some users mark a focus context.
- A restrained mascot facing a clearly labelled warning may increase, decrease,
  or leave vigilance unchanged.
- Care and Detachment may make disengagement easier for some users.
- Static, low-semantic postures may reduce—but cannot eliminate—cultural
  ambiguity or parasocial interpretation.

Each hypothesis could be false, population-dependent, or outweighed by adverse
effects. [`evaluation-outline.md`](evaluation-outline.md) describes questions,
not a validation result.

## 3. Claims DOSovoi does not make

DOSovoi does not claim to change an AI’s cognition, goals, humility, agency, or
alignment; prevent agentic drift or superintelligence; regulate dopamine;
clinically reduce anxiety; be NICE-compatible or neuroscience-validated;
produce universally safe cross-cultural interpretations; reduce P(doom) or
extend an AI-risk timeline; or make a dangerous operation safer by being
present.

It is not a replacement for accurate warnings, named verification mechanisms,
access control, sandboxing, peer review, or operator judgement.

## Research questions

Priority questions include warning recall, false reassurance, time to disengage,
distraction, annoyance, accessibility, cultural interpretation, emotional
attachment, and differences between novice and expert terminal users. Evidence
review should begin with relevant systematic reviews, standards, primary
research, and authoritative human-factors guidance, with exact citations and a
clear distinction between evidence and inference.
