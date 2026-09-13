use ulua_ast::records::{ast_name::AstName, location::Location};

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{lint_deprecated_api::LintDeprecatedApi, table_type::TableType},
};

impl LintDeprecatedApi {
  pub fn check_location_ast_name_ast_name(
    &mut self,
    location: &Location,
    global: AstName,
    index: AstName,
  ) {
    // SAFETY: context 在 linter 存活期内有效。
    let Some(global_value) = (unsafe { &*self.context }).builtin_globals.find(&global) else {
      return;
    };

    let Some(table) = get_type_id::<TableType>(global_value.r#type) else {
      return;
    };

    let index_name = index.as_str().unwrap_or("");
    if let Some(prop) = table.props.get(index_name)
      && prop.deprecated
    {
      self.report_property(location, prop, global.as_str(), index_name);
    }
  }
}
