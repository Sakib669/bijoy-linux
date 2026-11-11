//! Bijoy (Unijoy) Bangla keyboard, implemented in Rust.
//!
//! This is a fixed key-to-letter layout (the classic Bijoy typing manner):
//! every key maps to a Bangla letter or sign, plus a Shift layer and an
//! AltGr layer, and a hasanta dead key that turns vowel signs into the
//! corresponding independent vowels. It is not phonetic (not Avro) and not
//! the Inscript/National layout.

pub mod layout;

use std::collections::HashMap;

/// Which modifier layer a keystroke belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    /// Plain key, no Shift, no AltGr.
    Base,
    /// Shift held.
    Shift,
    /// AltGr (Right Alt) held. The key char is the shifted keysym for keys
    /// that need Shift (uppercase letters and symbols), matching the m17n
    /// `(A-X)` notation.
    AltGr,
}

/// The static Bijoy layout: key -> output for each layer.
#[derive(Debug, Clone)]
pub struct Layout {
    base: HashMap<char, String>,
    shift: HashMap<char, String>,
    altgr: HashMap<char, String>,
    dead: HashMap<char, String>,
    independent: HashMap<char, String>,
}

fn build(map: &[(&str, &str)]) -> HashMap<char, String> {
    map.iter()
        .map(|(k, v)| (k.chars().next().unwrap(), v.to_string()))
        .collect()
}

impl Layout {
    pub fn new() -> Self {
        Layout {
            base: build(layout::BASE),
            shift: build(layout::SHIFT),
            altgr: build(layout::ALTGR),
            dead: build(layout::DEAD),
            independent: build(layout::INDEPENDENT),
        }
    }

    pub fn base(&self) -> &HashMap<char, String> {
        &self.base
    }
    pub fn shift(&self) -> &HashMap<char, String> {
        &self.shift
    }
    pub fn altgr(&self) -> &HashMap<char, String> {
        &self.altgr
    }
    pub fn dead(&self) -> &HashMap<char, String> {
        &self.dead
    }
    pub fn independent(&self) -> &HashMap<char, String> {
        &self.independent
    }

    /// Look up a key in a layer. `None` means that key is not remapped in
    /// that layer and should fall through to its usual (English) value.
    pub fn lookup(&self, layer: Layer, key: char) -> Option<&str> {
        match layer {
            Layer::Base => self.base.get(&key).map(|s| s.as_str()),
            Layer::Shift => self.shift.get(&key).map(|s| s.as_str()),
            Layer::AltGr => self.altgr.get(&key).map(|s| s.as_str()),
        }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}

/// A stateful typing engine that reproduces Bijoy behavior, including the
/// hasanta dead key (`g`) and the independent-vowel continuation.
#[derive(Debug, Clone)]
pub struct Engine {
    layout: Layout,
    pending_dead: Option<String>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            layout: Layout::new(),
            pending_dead: None,
        }
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    /// Feed one keystroke and return the text it produces (possibly empty,
    /// e.g. while the hasanta dead key is pending).
    pub fn key(&mut self, layer: Layer, key: char) -> String {
        if let Some(dead) = self.pending_dead.take() {
            // After the hasanta dead key, a plain letter may produce an
            // independent vowel instead (g then f -> আ, g then G -> ॥).
            if layer != Layer::AltGr {
                if let Some(indep) = self.layout.independent.get(&key) {
                    return indep.clone();
                }
            }
            // Any other key keeps the hasanta and is then processed normally
            // (this is how conjuncts are typed: ক + গ-হসন্ত + ত).
            let mut out = dead;
            out.push_str(&self.key_no_dead(layer, key));
            return out;
        }
        self.key_no_dead(layer, key)
    }

    fn key_no_dead(&mut self, layer: Layer, key: char) -> String {
        if layer == Layer::Base {
            if let Some(d) = self.layout.dead.get(&key) {
                self.pending_dead = Some(d.clone());
                return String::new();
            }
        }
        match self.layout.lookup(layer, key) {
            Some(v) => v.to_string(),
            None => key.to_string(),
        }
    }

    /// Flush a still-pending dead key (use at end of input).
    pub fn flush(&mut self) -> String {
        self.pending_dead.take().unwrap_or_default()
    }

    /// Type a whole sequence of events and return the combined output.
    pub fn type_events(&mut self, events: &[(Layer, char)]) -> String {
        let mut out = String::new();
        for &(layer, key) in events {
            out.push_str(&self.key(layer, key));
        }
        out.push_str(&self.flush());
        out
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_layer_consonants_and_signs() {
        let l = Layout::new();
        assert_eq!(l.lookup(Layer::Base, 'j'), Some("ক"));
        assert_eq!(l.lookup(Layer::Base, 'k'), Some("ত"));
        assert_eq!(l.lookup(Layer::Base, 'd'), Some("ি"));
        assert_eq!(l.lookup(Layer::Base, 'f'), Some("া"));
        assert_eq!(l.lookup(Layer::Base, '1'), Some("১"));
    }

    #[test]
    fn shift_layer() {
        let l = Layout::new();
        assert_eq!(l.lookup(Layer::Shift, 'J'), Some("খ"));
        assert_eq!(l.lookup(Layer::Shift, 'G'), Some("।"));
        assert_eq!(l.lookup(Layer::Shift, 'F'), Some("অ"));
    }

    #[test]
    fn altgr_layer() {
        let l = Layout::new();
        assert_eq!(l.lookup(Layer::AltGr, 'f'), Some("আ"));
        assert_eq!(l.lookup(Layer::AltGr, 'd'), Some("ই"));
        assert_eq!(l.lookup(Layer::AltGr, '='), Some("≠"));
    }

    #[test]
    fn multi_codepoint_keys() {
        let l = Layout::new();
        // z -> hasanta + ra (reph), A -> ra + hasanta, Z -> hasanta + ya
        assert_eq!(l.lookup(Layer::Base, 'z'), Some("্র"));
        assert_eq!(l.lookup(Layer::Shift, 'A'), Some("র্"));
        assert_eq!(l.lookup(Layer::Shift, 'Z'), Some("্য"));
    }

    #[test]
    fn dead_key_independent_vowel() {
        let mut e = Engine::new();
        assert_eq!(e.key(Layer::Base, 'g'), "");
        assert_eq!(e.key(Layer::Base, 'f'), "আ");
    }

    #[test]
    fn dead_key_then_consonant_keeps_hasanta() {
        let mut e = Engine::new();
        e.key(Layer::Base, 'j'); // ক
        assert_eq!(e.key(Layer::Base, 'g'), "");
        // hasanta then ত -> the conjunct form "্ক"
        assert_eq!(e.key(Layer::Base, 'k'), "\u{09CD}\u{09A4}");
    }

    #[test]
    fn dead_key_then_shift_g_gives_double_danda() {
        let mut e = Engine::new();
        assert_eq!(e.key(Layer::Base, 'g'), "");
        assert_eq!(e.key(Layer::Shift, 'G'), "॥");
    }

    #[test]
    fn trailing_dead_key_flushes_hasanta() {
        let mut e = Engine::new();
        assert_eq!(e.key(Layer::Base, 'g'), "");
        assert_eq!(e.flush(), "\u{09CD}");
    }

    #[test]
    fn type_a_word() {
        let mut e = Engine::new();
        // ক (j) + ত (k) + ন (b) + ি (d) -> "কতনি"
        let events = [(Layer::Base, 'j'), (Layer::Base, 'k'), (Layer::Base, 'b'), (Layer::Base, 'd')];
        assert_eq!(e.type_events(&events), "কতনি");
    }
}
