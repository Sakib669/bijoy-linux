use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use bijoy::{Engine, Layer, Layout};

/// The generated XKB symbols file, embedded so the binary is self-contained.
const XKB_SYMBOLS: &str = include_str!("../xkb/symbols/bijoy");

/// A full keymap that includes the bijoy symbols, compiled on demand.
const KMAP: &str = "xkb_keymap {\n  xkb_keycodes  { include \"evdev+aliases(qwerty)\" };\n  xkb_types     { include \"complete\" };\n  xkb_compat    { include \"complete\" };\n  xkb_symbols   { include \"pc+us+bijoy\" };\n};\n";

const HELP: &str = "\
bijoy — Bijoy (Unijoy) Bangla keyboard for Linux

Usage:
  bijoy layout [base|shift|altgr|dead|independent]   print the key map
  bijoy type \"<keystrokes>\"                         simulate typing, print Bangla
  bijoy xkb                                         print the XKB symbols file
  bijoy test                                        run the built-in self checks
  bijoy list                                        show layout and install status
  bijoy toggle                                      flip between English and Bijoy
  bijoy install                                     user-level install (no sudo)
  bijoy install --system                            system-wide install (needs sudo)
  bijoy uninstall                                   remove the installed layout

Typing notation for `bijoy type`:
  a..z   base key           A..Z  Shift key           \\X   AltGr + X
  Example: `bijoy type \"j g k\"` types ক + hasanta + ত (the conjunct form).
";

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    match cmd {
        "layout" => cmd_layout(args.get(2).map(|s| s.as_str())),
        "type" => cmd_type(&args[2..]),
        "xkb" => cmd_xkb(),
        "test" => cmd_test(),
        "list" => cmd_list(),
        "toggle" => cmd_toggle(),
        "install" => {
            if args.iter().any(|a| a == "--system") {
                cmd_install_system()
            } else {
                cmd_install_user()
            }
        }
        "uninstall" => cmd_uninstall(),
        "enable" => cmd_enable(),
        "disable" => cmd_disable(),
        _ => print!("{}", HELP),
    }
}

fn sorted(map: &std::collections::HashMap<char, String>) -> Vec<(char, String)> {
    let mut v: Vec<(char, String)> = map.iter().map(|(k, v)| (*k, v.clone())).collect();
    v.sort_by_key(|(k, _)| *k as u32);
    v
}

fn cmd_layout(filter: Option<&str>) {
    let l = Layout::new();
    let show = |name: &str, map: &std::collections::HashMap<char, String>| {
        if filter.is_none() || filter == Some(name) {
            println!("== {} ==", name);
            for (k, v) in sorted(map) {
                println!("  {} -> {}", k, v);
            }
            println!();
        }
    };
    show("base", l.base());
    show("shift", l.shift());
    show("altgr", l.altgr());
    show("dead", l.dead());
    show("independent", l.independent());
}

/// Parse "\\X" = AltGr+X, uppercase = Shift, otherwise Base.
fn parse_events(input: &str) -> Vec<(Layer, char)> {
    let mut events = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('\\') => events.push((Layer::Base, '\\')),
                Some(next) => events.push((Layer::AltGr, next)),
                None => events.push((Layer::Base, '\\')),
            }
        } else if c.is_ascii_uppercase() {
            events.push((Layer::Shift, c));
        } else {
            events.push((Layer::Base, c));
        }
    }
    events
}

fn cmd_type(args: &[String]) {
    let input = args.join("");
    let mut engine = Engine::new();
    let events = parse_events(&input);
    let output = engine.type_events(&events);
    println!("{}", output);
}

fn cmd_xkb() {
    print!("{}", XKB_SYMBOLS);
}

fn cmd_test() {
    let mut ok = 0;
    let mut fail = 0;
    let mut check = |name: &str, got: &str, want: &str| {
        if got == want {
            ok += 1;
            println!("PASS  {} -> {}", name, got);
        } else {
            fail += 1;
            println!("FAIL  {}: got {:?} want {:?}", name, got, want);
        }
    };

    let l = Layout::new();
    check("j (base)", l.lookup(Layer::Base, 'j').unwrap(), "ক");
    check("d (base)", l.lookup(Layer::Base, 'd').unwrap(), "ি");
    check("J (shift)", l.lookup(Layer::Shift, 'J').unwrap(), "খ");
    check("G (shift)", l.lookup(Layer::Shift, 'G').unwrap(), "।");
    check("AltGr f", l.lookup(Layer::AltGr, 'f').unwrap(), "আ");
    check("z (reph)", l.lookup(Layer::Base, 'z').unwrap(), "্র");
    check("A (ra+hasanta)", l.lookup(Layer::Shift, 'A').unwrap(), "র্");

    let mut e = Engine::new();
    e.key(Layer::Base, 'g');
    check("g then f", &e.key(Layer::Base, 'f'), "আ");

    let mut e = Engine::new();
    e.key(Layer::Base, 'j');
    e.key(Layer::Base, 'g');
    check("j g k (conjunct)", &e.key(Layer::Base, 'k'), "\u{09CD}\u{09A4}");

    println!();
    println!("{} passed, {} failed", ok, fail);
    if fail > 0 {
        std::process::exit(1);
    }
}

fn cmd_list() {
    let l = Layout::new();
    println!("Layout: Bijoy (Unijoy) — fixed key-to-letter, not Avro/phonetic");
    println!("Layers: base={}, shift={}, altgr={}, dead={}, independent={}",
             l.base().len(), l.shift().len(), l.altgr().len(),
             l.dead().len(), l.independent().len());

    let sys = Path::new("/usr/share/X11/xkb/symbols/bijoy").exists();
    let user = user_cfg_dir().join("xkb/symbols/bijoy").exists();
    let active = user_cfg_dir().join(".active").exists();
    println!("installed (system): {}", sys);
    println!("installed (user):   {}", user);
    println!("bijoy active now:   {}", active);

    if let Ok(out) = Command::new("setxkbmap").arg("-query").output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout);
            for line in s.lines() {
                if line.starts_with("layout") || line.starts_with("variant") {
                    println!("{}", line);
                }
            }
        }
    }
}

fn user_cfg_dir() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/bijoy")
}

/// Apply the Bijoy keymap to the live X display (no sudo required).
fn apply_bijoy_keymap() -> bool {
    let cfg = user_cfg_dir();
    let kmap = cfg.join("bijoy.kmap");
    if !kmap.exists() {
        return false;
    }
    let xkb_user = cfg.join("xkb");
    let display = env::var("DISPLAY").unwrap_or_else(|_| ":0".to_string());
    let inc1 = "-I/usr/share/X11/xkb".to_string();
    let inc2 = format!("-I{}", xkb_user.display());
    Command::new("xkbcomp")
        .args([
            inc1.as_str(),
            inc2.as_str(),
            kmap.to_str().unwrap(),
            &display,
        ])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn cmd_toggle() {
    let active = user_cfg_dir().join(".active");
    if active.exists() {
        let _ = Command::new("setxkbmap").args(["-layout", "us,us"]).status();
        let _ = fs::remove_file(&active);
        println!("switched to English");
    } else {
        if apply_bijoy_keymap() {
            let _ = fs::write(&active, "1");
            println!("switched to Bijoy");
        } else {
            eprintln!("could not apply the Bijoy layout; run `bijoy install` first");
            std::process::exit(1);
        }
    }
}

fn self_path() -> String {
    env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "bijoy".to_string())
}

fn local_bin() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".local/bin")
}

fn cmd_install_user() {
    let cfg = user_cfg_dir();
    let symbols_dir = cfg.join("xkb/symbols");
    if fs::create_dir_all(&symbols_dir).is_err() {
        eprintln!("could not create {}", symbols_dir.display());
        std::process::exit(1);
    }
    let sym = symbols_dir.join("bijoy");
    let kmap = cfg.join("bijoy.kmap");
    if fs::write(&sym, XKB_SYMBOLS).is_err() || fs::write(&kmap, KMAP).is_err() {
        eprintln!("could not write layout files under {}", cfg.display());
        std::process::exit(1);
    }
    println!("  symbols -> {}", sym.display());
    println!("  keymap  -> {}", kmap.display());

    // Install the binary to ~/.local/bin so Ctrl+Alt+B can call it.
    let bin_dir = local_bin();
    let _ = fs::create_dir_all(&bin_dir);
    let dest = bin_dir.join("bijoy");
    if fs::copy(self_path(), &dest).is_ok() {
        println!("  binary  -> {}", dest.display());
    }

    cmd_enable();
    println!("done. Press Ctrl+Alt+B to toggle between English and Bijoy.");
}

fn sudo_write(path: &str, contents: &str) -> bool {
    use std::io::Write;
    let child = Command::new("sudo")
        .args(["tee", path])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
    let mut child = match child {
        Ok(c) => c,
        Err(_) => return false,
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(contents.as_bytes());
    }
    child.wait().map(|s| s.success()).unwrap_or(false)
}

fn register_evdev_xml() -> bool {
    let path = "/usr/share/X11/xkb/rules/evdev.xml";
    let Ok(xml) = fs::read_to_string(path) else { return false; };
    if xml.contains("<name>bijoy</name>") {
        return true;
    }
    let block = "\n        <layout>\n          <configItem>\n            <name>bijoy</name>\n            <shortDescription>bijoy</shortDescription>\n            <description>Bijoy (Unijoy)</description>\n            <languageList><iso639Id>ben</iso639Id></languageList>\n          </configItem>\n        </layout>";
    let Some(insert_at) = xml.rfind("</layoutList>") else { return false; };
    let new_xml = format!("{}{}\n    {}", &xml[..insert_at], block, &xml[insert_at..]);
    sudo_write(path, &new_xml)
}

fn register_evdev_lst() -> bool {
    let path = "/usr/share/X11/xkb/rules/evdev.lst";
    let Ok(lst) = fs::read_to_string(path) else { return false; };
    if lst.contains("  bijoy") {
        return true;
    }
    let mut new_lst = lst.trim_end().to_string();
    new_lst.push_str("\n  bijoy           Bijoy (Unijoy)\n");
    sudo_write(path, &new_lst)
}

fn cmd_install_system() {
    println!("installing Bijoy (Unijoy) system-wide (needs sudo)…");
    if !sudo_write("/usr/share/X11/xkb/symbols/bijoy", XKB_SYMBOLS) {
        eprintln!("failed to write XKB symbols (sudo needed)");
        std::process::exit(1);
    }
    println!("  XKB symbols -> /usr/share/X11/xkb/symbols/bijoy");
    if register_evdev_xml() {
        println!("  registered in rules/evdev.xml");
    }
    if register_evdev_lst() {
        println!("  registered in rules/evdev.lst");
    }
    let bin = self_path();
    if Command::new("sudo").args(["cp", &bin, "/usr/local/bin/bijoy"]).status().map(|s| s.success()).unwrap_or(false) {
        println!("  binary -> /usr/local/bin/bijoy");
    }
    cmd_enable();
    println!("done. Press Ctrl+Alt+B to toggle.");
}

fn cmd_uninstall() {
    println!("removing Bijoy layout…");
    let cfg = user_cfg_dir();
    let _ = fs::remove_dir_all(&cfg);
    let _ = fs::remove_file(local_bin().join("bijoy"));
    let _ = Command::new("setxkbmap").args(["-layout", "us,us"]).status();
    let _ = Command::new("sudo").args(["rm", "-f", "/usr/share/X11/xkb/symbols/bijoy"]).status();
    let _ = Command::new("sudo").args(["rm", "-f", "/usr/local/bin/bijoy"]).status();
    cmd_disable();
    println!("done.");
}

fn cmd_enable() {
    let slot = "/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/";
    let toggle_cmd = format!("{}/bijoy toggle", local_bin().display());
    let _ = Command::new("gsettings").args(["set",
        "org.gnome.settings-daemon.plugins.media-keys", "custom-keybindings",
        &format!("['{}']", slot)]).status();
    let _ = Command::new("gsettings").args(["set",
        &format!("org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:{}", slot),
        "name", "Toggle Bijoy"]).status();
    let _ = Command::new("gsettings").args(["set",
        &format!("org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:{}", slot),
        "command", &toggle_cmd]).status();
    let _ = Command::new("gsettings").args(["set",
        &format!("org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:{}", slot),
        "binding", "<Control><Alt>b"]).status();
    println!("  Ctrl+Alt+B -> {}", toggle_cmd);
}

fn cmd_disable() {
    let _ = Command::new("gsettings").args(["set",
        "org.gnome.settings-daemon.plugins.media-keys", "custom-keybindings", "[]"]).status();
    println!("  hotkey cleared");
}
