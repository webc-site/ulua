use alloc::{boxed::Box, string::String};
use core::option::Option;

use crate::{
  records::extern_type::ExternType, type_aliases::autocomplete_entry_map::AutocompleteEntryMap,
};

pub type StringCompletionCallback =
  Box<dyn Fn(String, Option<*const ExternType>, Option<String>) -> Option<AutocompleteEntryMap>>;
