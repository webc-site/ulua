use alloc::{collections::BTreeMap, string::String};

use crate::records::autocomplete_entry::AutocompleteEntry;

pub type AutocompleteEntryMap = BTreeMap<String, AutocompleteEntry>;
