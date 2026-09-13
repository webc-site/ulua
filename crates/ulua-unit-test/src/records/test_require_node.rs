use alloc::string::String;
use std::collections::HashMap;

use ulua_analysis::type_aliases::module_name_type::ModuleName;
#[derive(Debug, Clone)]
pub struct TestRequireNode {
  pub module_name: ModuleName,
  pub all_sources: *const HashMap<ModuleName, String>,
}
