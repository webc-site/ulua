use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_common::records::dense_hash_map::DenseHashMap;
use ulua_config::records::config::Config;

use crate::{
  functions::trace_requires::trace_requires,
  records::{
    frontend::Frontend, require_trace_result::RequireTraceResult,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};
impl Frontend {
  pub fn get_required_scripts(
    &mut self,
    name: &ModuleName,
    limits: &TypeCheckLimits,
  ) -> Vec<ModuleName> {
    // C++: RequireTraceResult require = requireTrace[name];
    // operator[] default-inserts an empty value when the key is absent.
    let mut require = self
      .require_trace
      .entry(name.clone())
      .or_insert_with(|| RequireTraceResult {
        exprs: DenseHashMap::new(null_mut()),
        require_list: Vec::new(),
      })
      .clone();

    if self.is_dirty(name, false) {
      let Some(source_code) = self.file_resolver_mut().read_source(name) else {
        return Vec::new();
      };

      let config: &Config = unsafe {
        let get_config = (*self.config_resolver)
          .get_config
          .expect("ConfigResolver::getConfig is not set");
        &*get_config(self.config_resolver, name, limits)
      };
      let mut opts = config.parse_options.clone();
      opts.capture_comments = true;
      let mut result =
        self.parse_module_name_string_view_parse_options(name, &source_code.source, &opts);
      result.r#type = source_code.r#type;
      // SAFETY: result.root 指向本函数刚解析出的 AST，此处独占（C++ 契约）；
      // RequireTracer 按 cpp `AstStatBlock*` 非 const 语义取 `&mut`。
      require = unsafe {
        trace_requires(
          self.file_resolver_mut(),
          &mut *result.root,
          name.clone(),
          limits,
        )
      };
    }

    let mut required_module_names: Vec<ModuleName> = Vec::with_capacity(require.require_list.len());
    for (module_name, _) in require.require_list.iter() {
      required_module_names.push(module_name.clone());
    }
    required_module_names
  }
}
