//! Spanish catalogs, one file per area of the interface. A key may appear in several files only
//! with the same translation (the i18n tests check); the first one wins.

mod annotate;
mod chrome;
mod core;
mod dialogs;
mod edit;

pub(super) const ALL: &[&[(&str, &str)]] = &[core::ENTRIES, chrome::ENTRIES, dialogs::ENTRIES, annotate::ENTRIES, edit::ENTRIES];
