# bijoy-linux

A Bijoy (Unijoy) Bangla keyboard for Linux, implemented in Rust.

It is a **fixed key-to-letter layout**: you press the same keys you would on
the classic Bijoy keyboard and get the same Bangla letters. It is **not**
Avro (phonetic) and **not** the Inscript/National layout.

New to this? Start with the [beginner guide](USAGE.md), which covers install,
the full key map, and switching with plain step-by-step instructions.

Examples of the Bijoy manner:

| You type | You get |
|---|---|
| `j` | ক |
| `d` | ি |
| `k` | ত |
| `J` (Shift) | খ |
| `g` then `f` | আ (hasanta dead key turns a vowel sign into the full vowel) |
| `j` `g` `k` | ক্ত (hasanta makes the conjunct form) |
| AltGr+`f` | আ (independent vowels also live on the AltGr layer) |
| `4` `$` | ৪ ৳ (digits and the taka sign) |

## What is inside

- **`bijoy-core`** (the library, `src/lib.rs`): the complete layout table plus
  a stateful typing engine with the base, Shift, and AltGr layers and the
  hasanta dead key.
- **`bijoy`** (the command, `src/main.rs`): install, toggle, and inspect the
  layout.
- **`xkb/symbols/bijoy`**: an XKB keyboard file generated from the same table,
  so the layout works as a real system keyboard in every X11 application.
- **`tools/generate.py`**: regenerates `src/layout.rs` and the XKB file from
  the public `bn-unijoy.mim` definition (single source of truth).

The mapping is generated from `bn-unijoy.mim` (m17n-db, LGPL-2.1), so there are
no hand-copied mistakes. See `NOTICE` for the license attribution.

## Build

```bash
cargo build --release
```

## Commands

```bash
bijoy layout                    # print the whole key map (or a single layer)
bijoy type "jgk"                # simulate keystrokes and print the Bangla
bijoy type "j\fj"               # \X means AltGr+X  (here: ক আ ক)
bijoy xkb                       # print the XKB symbols file
bijoy test                      # run the built-in self checks
bijoy list                      # show layout + install status
bijoy toggle                    # flip between English and Bijoy
```

## Install and use

```bash
cd ~/bijoy-linux && cargo build --release && ./target/release/bijoy install
```

The default install is user-level (no sudo). It writes the layout and a keymap
to `~/.config/bijoy/`, installs the binary to `~/.local/bin/bijoy`, and binds
**Ctrl+Alt+B** to toggle between English and Bijoy. Press Ctrl+Alt+B (or run
`bijoy toggle`) to switch.

For a system-wide install that also registers the layout in XKB rules so it
shows up in the desktop layout switcher:

```bash
sudo ./target/release/bijoy install --system
```

```bash
bijoy uninstall                  # remove the user-level and system files
```

## One XKB limitation

An XKB keysym is a single Unicode code point, so the three Bijoy keys that
produce two code points type only their first code point in the XKB layout:

- `z` -> ্র (hasanta + ra)  — types hasanta here
- `Z` -> ্য (hasanta + ya)  — types hasanta here
- `A` -> র্ (ra + hasanta)  — types র here

The Rust engine (`bijoy type`) reproduces all three in full. If you need the
full behavior inside every app, the follow-up is a Rust IBus engine.
