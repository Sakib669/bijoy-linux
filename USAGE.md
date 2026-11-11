# How to use bijoy-linux (beginner guide)

This page explains, in plain words, how to install and type Bangla with this
program. You do not need to know anything about programming to use it.

## What this is

`bijoy-linux` lets you type Bangla on a Linux computer the **Bijoy** way.

The Bijoy way means each English key always types one fixed Bangla letter. For
example, press `j` and you get ক. Press `d` and you get ি. It is **not** Avro
(where you spell a word in English letters and it converts), and it is **not**
the Inscript/National layout.

## What you need

- A Linux computer (tested on Linux Mint; works on any Linux with X11).
- Rust, only if you build it yourself (one command, shown below).

## Install (3 steps, no password needed)

Open the Terminal app and type these, one at a time, pressing Enter after each:

```bash
cd bijoy-linux
cargo build --release
./target/release/bijoy install
```

If you do not have Rust yet, install it first with this one command and then
repeat the three steps above:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When it finishes you will see `done. Press Ctrl+Alt+B to toggle...`.

## Switch between English and Bangla

Press **Ctrl + Alt + B** together.

- Press it once and you are typing Bangla.
- Press it again and you are back to English.

You can also type `bijoy toggle` in the Terminal to do the same thing.

## The keyboard map

Here is what each key types. No Shift means press the key normally.

### Numbers (no Shift)

| Key | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|
| Bangla | ১ | ২ | ৩ | ৪ | ৫ | ৬ | ৭ | ৮ | ৯ | ০ |

### Letters (no Shift)

| Key | q | w | e | r | t | y | u | i | o | p | \\ |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Bangla | ঙ | য | ড | প | ট | চ | জ | হ | গ | ড় | ঃ |

| Key | a | s | d | f | g | h | j | k | l |
|---|---|---|---|---|---|---|---|---|---|
| Bangla | ৃ | ু | ি | া | ্ | ব | ক | ত | দ |

| Key | z | x | c | v | b | n | m |
|---|---|---|---|---|---|---|---|---|
| Bangla | ্র | ো | ে | র | ন | স | ম |

### Letters (with Shift)

Hold Shift and press the key:

| Key | Q | W | E | R | T | Y | U | I | O | P |
|---|---|---|---|---|---|---|---|---|---|---|
| Bangla | ং | য় | ঢ | ফ | ঠ | ছ | ঝ | ঞ | ঘ | ঢ় |

| Key | A | S | D | F | G | H | J | K | L |
|---|---|---|---|---|---|---|---|---|---|---|
| Bangla | র্ | ূ | ী | অ | । | ভ | খ | থ | ধ |

| Key | Z | X | C | V | B | N | M |
|---|---|---|---|---|---|---|---|---|
| Bangla | ্য | ৌ | ৈ | ল | ণ | ষ | শ |

### Useful symbols

| You type | You get | What it is |
|---|---|---|
| `$` (Shift+4) | ৳ | taka sign |
| `~` (Shift+\`) | ‍ | zero width joiner |
| `^` (Shift+6) | ÷ | division sign |
| `&` (Shift+7) | ঁ | chandrabindu |
| `*` (Shift+8) | × | multiplication sign |
| `|` (Shift+\) | ৎ | khanda ta |
| `\` | ঃ | visarga |

## The hasanta and full vowels

The key `g` types the **hasanta** (্). It is used to join letters, and it has a
special job: type `g` and then a vowel key, and you get the full (independent)
vowel instead of the vowel sign.

- `g` then `f` gives আ
- `g` then `d` gives ই
- `g` then `s` gives উ
- `g` then `x` gives ও
- `g` then `c` gives এ
- `g` then `G` gives ॥

You can get the same full vowels with the **Right Alt (AltGr)** key:

- AltGr + `f` = আ
- AltGr + `d` = ই
- AltGr + `s` = উ
- AltGr + `x` = ও
- AltGr + `c` = এ
- AltGr + `e` = ঈ
- AltGr + `u` = ঊ
- AltGr + `i` = ঐ
- AltGr + `o` = ঔ
- AltGr + `a` = ঋ
- AltGr + `.` = ় (nukta)

## Examples

| You type | You get |
|---|---|
| `j` `k` | কত |
| `j` `g` `k` | ক্ত (joined) |
| `b` `v` `c` `l` | ন র ে দ (the letters for "নরেদ") |
| `g` `f` | আ |

## Try it yourself

Type `bijoy type "jgk"` in the Terminal. It prints the Bangla for those keys
without you having to switch anything.

## Trouble? Check this

- **Ctrl+Alt+B does nothing** — run `bijoy install` again, then log out and
  back in. On some desktops the shortcut needs a fresh login.
- **It types Bangla but I want English back** — press Ctrl+Alt+B again.
- **Letters look like boxes** — your font does not have Bangla. Most Linux
  systems already have Noto Sans Bengali installed; you can install it with
  `sudo apt install fonts-noto-core`.

## Remove it

```bash
bijoy uninstall
```

This removes the layout, the program, and the Ctrl+Alt+B shortcut.

## A small note about three special keys

The keys `z` (্র), `Z` (্য), and `A` (র্) produce two joined marks in real
Bijoy. On the keyboard they type only the first part here, because of a limit
in how Linux keyboard layouts work. The `bijoy type` command types them fully.
Everything else works exactly like Bijoy.
