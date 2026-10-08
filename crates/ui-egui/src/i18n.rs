//! Interface translations. Command ids, document text and file names remain stable.
//! Untranslated labels fall back to English so coverage can grow incrementally.
//!
//! Catalogs are flat `(English, translated)` tables: [`ja`] for Japanese and the files under
//! [`es`] for Spanish, one per area of the interface. The app publishes the chosen language once
//! per frame ([`Language::make_current`]), so any widget can call the free functions [`tr`] and
//! [`trf`] without threading the app through.

use std::collections::HashMap;
use std::fmt::Display;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU8, Ordering};

mod es;
mod ja;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    En,
    Ja,
    Es,
}

/// The language the free functions translate into; written by [`Language::make_current`].
static CURRENT: AtomicU8 = AtomicU8::new(0);

impl Language {
    pub const ALL: [Self; 3] = [Self::En, Self::Ja, Self::Es];

    pub fn name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Ja => "日本語",
            Self::Es => "Español",
        }
    }

    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "en" => Some(Self::En),
            "ja" => Some(Self::Ja),
            "es" => Some(Self::Es),
            _ => None,
        }
    }

    /// Make this the language [`tr`] and [`trf`] use from now on.
    pub fn make_current(self) {
        CURRENT.store(self as u8, Ordering::Relaxed);
    }

    /// The language [`tr`] and [`trf`] use.
    pub fn current() -> Self {
        match CURRENT.load(Ordering::Relaxed) {
            1 => Self::Ja,
            2 => Self::Es,
            _ => Self::En,
        }
    }

    /// The `(English, translated)` tables of this language; empty for English itself.
    fn catalogs(self) -> &'static [&'static [(&'static str, &'static str)]] {
        match self {
            Self::En => &[],
            Self::Ja => &[ja::ENTRIES],
            Self::Es => es::ALL,
        }
    }

    /// The lookup table, built on first use. Duplicate keys keep their first translation.
    fn map(self) -> &'static HashMap<&'static str, &'static str> {
        static JA: OnceLock<HashMap<&str, &str>> = OnceLock::new();
        static ES: OnceLock<HashMap<&str, &str>> = OnceLock::new();
        static EN: OnceLock<HashMap<&str, &str>> = OnceLock::new();
        let cell = match self {
            Self::En => &EN,
            Self::Ja => &JA,
            Self::Es => &ES,
        };
        cell.get_or_init(|| {
            let mut map = HashMap::new();
            for (english, translated) in self.catalogs().iter().flat_map(|c| c.iter()) {
                map.entry(*english).or_insert(*translated);
            }
            map
        })
    }

    pub fn tr(self, text: &str) -> &str {
        self.map().get(text).copied().unwrap_or(text)
    }
}

/// `text` in the current interface language, or `text` itself when there is no translation.
pub fn tr(text: &str) -> &str {
    Language::current().tr(text)
}

/// Like [`tr`] for a label with `{}` placeholders, filled in order from `args`.
/// `trf("Page {} of {}", &[&3, &10])` gives "Página 3 de 10" in Spanish.
pub fn trf(text: &str, args: &[&dyn Display]) -> String {
    let template = tr(text);
    let mut out = String::with_capacity(template.len() + args.len() * 8);
    let mut args = args.iter();
    let mut rest = template;
    while let Some(at) = rest.find("{}") {
        out.push_str(&rest[..at]);
        match args.next() {
            Some(arg) => out.push_str(&arg.to_string()),
            None => out.push_str("{}"),
        }
        rest = &rest[at + 2..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(catalog: &[(&'static str, &'static str)]) -> Vec<&'static str> {
        catalog.iter().map(|(en, _)| *en).collect()
    }

    #[test]
    fn translations_are_unique_and_preserve_unknown_text() {
        for language in [Language::Ja, Language::Es] {
            let mut seen: HashMap<&str, &str> = HashMap::new();
            for (en, translated) in language.catalogs().iter().flat_map(|c| c.iter()) {
                assert!(!translated.is_empty(), "{language:?}: empty translation for {en:?}");
                assert_eq!(en.matches("{}").count(), translated.matches("{}").count(), "{language:?}: placeholder count differs for {en:?}");
                if let Some(other) = seen.insert(en, translated) {
                    assert_eq!(other, *translated, "{language:?}: {en:?} translated two different ways");
                }
                assert_eq!(Language::En.tr(en), *en);
            }
        }
        assert_eq!(Language::Ja.tr("File"), "ファイル");
        assert_eq!(Language::Ja.tr("日本語の文書.pdf"), "日本語の文書.pdf");
        assert_eq!(Language::Es.tr("File"), "Archivo");
        assert_eq!(Language::Es.tr("Informe anual.pdf"), "Informe anual.pdf");
        assert_eq!(Language::parse("es"), Some(Language::Es));
        assert_eq!(Language::parse("xx"), None);
    }

    #[test]
    fn spanish_covers_every_japanese_label() {
        // Spanish is the most complete catalog; whatever Japanese translates, Spanish must too.
        let spanish = Language::Es.map();
        for key in keys(ja::ENTRIES) {
            assert!(spanish.contains_key(key), "Spanish catalog lacks {key:?}");
        }
    }

    #[test]
    fn free_functions_follow_the_current_language_and_fill_placeholders() {
        Language::Es.make_current();
        assert_eq!(Language::current(), Language::Es);
        assert_eq!(tr("File"), "Archivo");
        assert_eq!(trf("Page {} of {}", &[&3, &10]), "Página 3 de 10");
        assert_eq!(trf("{} extra {}", &[&1]), "1 extra {}");
        Language::En.make_current();
        assert_eq!(tr("File"), "File");
        assert_eq!(trf("Page {} of {}", &[&3, &10]), "Page 3 of 10");
    }

    #[test]
    fn language_persists_and_invalid_input_keeps_current_language() {
        let mut app = crate::PdfCraftApp::default();
        app.set_option("language", "ja").unwrap();
        assert_eq!(app.language, Language::Ja);
        assert!(app.set_option("language", "xx").is_err());
        assert_eq!(app.language, Language::Ja);
        app.set_option("language", "es").unwrap();
        assert_eq!(app.language, Language::Es);
        let mut restored = crate::PdfCraftApp::default();
        restored.restore(&app.persist());
        assert_eq!(restored.language, Language::Es);
        restored.restore(r#"{"language":"xx"}"#);
        assert_eq!(restored.language, Language::Es);
        let mut legacy = crate::PdfCraftApp::default();
        legacy.restore("{}");
        assert_eq!(legacy.language, Language::En);
    }
}
