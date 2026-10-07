use alloc::string::String;

use ulua_analysis::{
  records::extern_type::ExternType, type_aliases::autocomplete_entry_map::AutocompleteEntryMap,
};

pub fn null_callback(
  _tag: String,
  _extern_type: Option<&'static ExternType>,
  _contents: Option<String>,
) -> Option<AutocompleteEntryMap> {
  None
}
