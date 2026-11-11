# 0001 Bijoy (Unijoy) keyboard for Linux, in Rust

**Status**: Proposed

## Summary

Reimplement the classic Bijoy (Unijoy) Bangla keyboard on Linux as a Rust
program. It is a fixed key-to-letter layout, not Avro (phonetic) and not the
Inscript/National layout. The goal is: same key positions as Bijoy, Unicode
output, and a Ctrl+Alt+B toggle to switch between English and Bangla.

## Requirements

- A key maps to the same Bangla character it maps to on the classic Bijoy
  keyboard (for example `j` to ক, `d` to ি, `g` to the hasanta).
- Three modifier layers: base, Shift, and AltGr.
- The hasanta dead key: `g` then a vowel key produces the full independent
  vowel (`g` then `f` is আ), while `g` then a consonant keeps the hasanta to
  form conjuncts.
- A Rust command `bijoy` that installs, toggles, lists, and tests the layout.
- A toggle command bound to Ctrl+Alt+B.

## Decision

- **Language**: Rust, with no external crates (standard library only) so the
  build is dependable.
- **Single source of truth**: the public `bn-unijoy.mim` (m17n-db, LGPL-2.1).
  `tools/generate.py` parses it and emits both the Rust table (`src/layout.rs`)
  and the XKB symbols file (`xkb/symbols/bijoy`). No hand copying.
- **Shape**:
  - `bijoy-core` library: layout table + stateful typing engine (base, Shift,
    AltGr, hasanta dead key).
  - `bijoy` binary: `layout`, `type`, `xkb`, `test`, `list`, `toggle`,
    `install`, `uninstall`, `enable`, `disable`.
  - XKB symbols file for the actual system keyboard (works in every X11 app).

## Build plan

1. Scaffold the Cargo project.
2. Write `tools/generate.py` and generate the layout table and XKB file.
3. Implement the engine and unit tests.
4. Implement the CLI commands.
5. Build, test, and verify against the m17n table.
6. Write README, NOTICE (LGPL attribution), and install the layout.

## Consequences

- An XKB keysym is one code point, so the three Bijoy keys that emit two code
  points (`z`, `Z`, `A`) type only their first code point in the XKB layout.
  The Rust engine reproduces them in full. A Rust IBus engine is the follow-up
  for complete parity inside every application.
- No proprietary Bijoy software, code, or fonts are copied; only the public
  functional layout is reproduced.

## Rationale

The layout table is generated from the public `bn-unijoy.mim` (LGPL-2.1)
rather than typed by hand, so it is exactly faithful to the reference layout.
XKB was chosen as the typing mechanism because a fixed key-to-letter layout is
a keyboard layout, not a predictive input method, and XKB works in every X11
application without a daemon. A user-level (no sudo) install is the default
because it works everywhere and needs no password; `install --system` remains
available for a system-wide layout.
