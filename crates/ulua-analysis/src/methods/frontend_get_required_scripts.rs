use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

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
        exprs: DenseHashMap::default(),
        require_list: Vec::new(),
      })
      .clone();

    if self.is_dirty(name, false) {
      let Some(source_code) = self.file_resolver.read_source(name) else {
        return Vec::new();
      };

      // 配置读取直接用 `config_resolver` 字段（`Box<dyn ConfigResolver>` 独占、
      // 自动 deref，零 unsafe）；`.clone()` 后即释放对 `self` 的借用。
      let config = self.config_resolver.get_config(name, limits);
      let mut opts = config.parse_options.clone();
      opts.capture_comments = true;
      let mut result =
        self.parse_module_name_string_view_parse_options(name, &source_code.source, &opts);
      result.r#type = source_code.r#type;
      // 根块已句柄化：`Handle::get_mut` 物化本函数独占借用交 RequireTracer
      // （cpp 契约：刚解析出的 AST、按非 const `AstStatBlock*` 遍历），
      // 原裸 deref 的 unsafe 随之消亡。
      let root = result
        .root
        .expect("getRequiredModules: 解析后根块应在场（cpp 直取 result.root）");
      require = trace_requires(
        &mut *self.file_resolver,
        root.get_mut(),
        name.clone(),
        limits,
      );
    }

    let mut required_module_names: Vec<ModuleName> = Vec::with_capacity(require.require_list.len());
    for (module_name, _) in require.require_list.iter() {
      required_module_names.push(module_name.clone());
    }
    required_module_names
  }
}
