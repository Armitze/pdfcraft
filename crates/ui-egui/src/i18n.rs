//! Interface translations. Command ids, document text and file names remain stable.
//! Untranslated labels fall back to English so coverage can grow incrementally.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    En,
    Ja,
    Es,
}

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

    /// The `(English, translated)` pairs of this language; empty for English itself.
    fn catalog(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::En => &[],
            Self::Ja => JAPANESE,
            Self::Es => SPANISH,
        }
    }

    pub fn tr(self, text: &str) -> &str {
        if let Some((_, translated)) = self.catalog().iter().find(|(english, _)| *english == text) {
            return translated;
        }
        text
    }
}

const JAPANESE: &[(&str, &str)] = &[
    ("Menu", "メニュー"),
    ("File", "ファイル"),
    ("Edit", "編集"),
    ("Pages", "ページ"),
    ("View", "表示"),
    ("Help", "ヘルプ"),
    ("Preferences", "環境設定"),
    ("Preferences…", "環境設定…"),
    ("Interface language", "表示言語"),
    ("Identity", "個人情報"),
    ("Name on new comments", "新しい注釈の作成者名"),
    ("Open…", "開く…"),
    ("New blank PDF", "空白の PDF を作成"),
    ("Create PDF from file…", "ファイルから PDF を作成…"),
    ("Create PDF from images…", "画像から PDF を作成…"),
    ("Create PDF from clipboard", "クリップボードから PDF を作成"),
    ("Combine files…", "ファイルを結合…"),
    ("Save", "保存"),
    ("Save as…", "別名で保存…"),
    ("Close file", "ファイルを閉じる"),
    ("Close all", "すべて閉じる"),
    ("Revert", "保存済みの状態に戻す"),
    ("Print…", "印刷…"),
    ("Document properties…", "文書のプロパティ…"),
    ("Undo", "取り消し"),
    ("Redo", "やり直し"),
    ("Find…", "検索…"),
    ("Advanced search…", "高度な検索…"),
    ("Copy pages", "ページをコピー"),
    ("Cut pages", "ページを切り取り"),
    ("Paste pages", "ページを貼り付け"),
    ("Fit visible", "表示範囲に合わせる"),
    ("Marquee zoom", "範囲指定ズーム"),
    ("Take a snapshot", "スナップショットを作成"),
    ("Full screen mode", "全画面表示"),
    ("Read mode", "閲覧モード"),
    ("Switch light / dark theme", "明るい／暗いテーマを切り替え"),
    ("Comments panel", "コメントパネル"),
    ("Form fields panel", "フォームフィールドパネル"),
    ("Clear form", "フォームをクリア"),
    ("Find tools and commands…", "ツールとコマンドを検索…"),
    ("Zoom", "ズーム"),
    ("Actual size", "実際のサイズ"),
    ("Zoom to page level", "ページ全体を表示"),
    ("Fit to width", "幅に合わせる"),
    ("Display theme", "表示テーマ"),
    ("Side panels", "サイドパネル"),
    ("Enable Acrobat JavaScript", "Acrobat JavaScript を有効にする"),
    ("OK", "OK"),
];

const SPANISH: &[(&str, &str)] = &[
    ("Menu", "Menú"),
    ("File", "Archivo"),
    ("Edit", "Edición"),
    ("Pages", "Páginas"),
    ("View", "Ver"),
    ("Help", "Ayuda"),
    ("Preferences", "Preferencias"),
    ("Preferences…", "Preferencias…"),
    ("Interface language", "Idioma de la interfaz"),
    ("Identity", "Identidad"),
    ("Name on new comments", "Nombre en los comentarios nuevos"),
    ("Open…", "Abrir…"),
    ("New blank PDF", "Nuevo PDF en blanco"),
    ("Create PDF from file…", "Crear PDF desde archivo…"),
    ("Create PDF from images…", "Crear PDF desde imágenes…"),
    ("Create PDF from clipboard", "Crear PDF desde el portapapeles"),
    ("Combine files…", "Combinar archivos…"),
    ("Save", "Guardar"),
    ("Save as…", "Guardar como…"),
    ("Close file", "Cerrar archivo"),
    ("Close all", "Cerrar todo"),
    ("Revert", "Revertir"),
    ("Print…", "Imprimir…"),
    ("Document properties…", "Propiedades del documento…"),
    ("Undo", "Deshacer"),
    ("Redo", "Rehacer"),
    ("Find…", "Buscar…"),
    ("Advanced search…", "Búsqueda avanzada…"),
    ("Copy pages", "Copiar páginas"),
    ("Cut pages", "Cortar páginas"),
    ("Paste pages", "Pegar páginas"),
    ("Fit visible", "Ajustar a lo visible"),
    ("Marquee zoom", "Zoom de selección"),
    ("Take a snapshot", "Tomar instantánea"),
    ("Full screen mode", "Modo de pantalla completa"),
    ("Read mode", "Modo de lectura"),
    ("Switch light / dark theme", "Cambiar tema claro / oscuro"),
    ("Comments panel", "Panel de comentarios"),
    ("Form fields panel", "Panel de campos de formulario"),
    ("Clear form", "Borrar formulario"),
    ("Find tools and commands…", "Buscar herramientas y comandos…"),
    ("Zoom", "Zoom"),
    ("Actual size", "Tamaño real"),
    ("Zoom to page level", "Ajustar a la página"),
    ("Fit to width", "Ajustar al ancho"),
    ("Display theme", "Tema de visualización"),
    ("Side panels", "Paneles laterales"),
    ("Enable Acrobat JavaScript", "Activar JavaScript de Acrobat"),
    ("OK", "Aceptar"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translations_are_unique_and_preserve_unknown_text() {
        for language in [Language::Ja, Language::Es] {
            let catalog = language.catalog();
            for (i, (en, translated)) in catalog.iter().enumerate() {
                assert!(!translated.is_empty());
                assert!(catalog.iter().take(i).all(|(other, _)| en != other));
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
    fn every_language_translates_the_same_labels() {
        // A label added to one catalog must be added to all of them, so menus never mix languages.
        let japanese: Vec<&str> = JAPANESE.iter().map(|(en, _)| *en).collect();
        let spanish: Vec<&str> = SPANISH.iter().map(|(en, _)| *en).collect();
        assert_eq!(japanese, spanish);
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
