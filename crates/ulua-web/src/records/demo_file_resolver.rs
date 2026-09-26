//! Port of `DemoFileResolver : Luau::FileResolver` (`CLI/src/Web.cpp:16-46`).
//!
//! The luau.org/demo file resolver: 单模块内存表版 `FileResolver`，供
//! [`crate::records::demo_frontend::DemoFrontend`] 做类型检查。cpp 的四个虚函数在
//! Rust 侧直接落进 [`FileResolver`] 实现（原 `methods/` 逐虚函数拆文件 + trait 薄
//! 转发的形态属照抄 C++ vtable 的机器味结构，已合并为惯用单实现）。
//!
//! C++ member: `std::unordered_map<Luau::ModuleName, std::string> source;` — ported as
//! a typed `HashMap<ModuleName, String>` (no untyped JSON)。表用工作区统一的
//! foldhash `FixedState` 别名（先例 `ulua-common/src/collections.rs`），
//! 仅 get/insert/clear、无迭代。

use std::string::String;

use ulua_analysis::{
  records::{
    file_resolver::FileResolver, module_info::ModuleInfo, source_code::SourceCode,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::{collections::HashMap, module_name_type::ModuleName},
};
use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_global::AstExprGlobal},
  rtti::ast_node_try_as,
};

/// cpp 侧 `source` 表默认空构造，故 `Default` 即全部初始化需求。
#[derive(Debug, Default)]
pub(crate) struct DemoFileResolver {
  pub source: HashMap<ModuleName, String>,
}

impl FileResolver for DemoFileResolver {
  /// `std::optional<Luau::SourceCode> readSource(const ModuleName&)`
  /// (`CLI/src/Web.cpp:18-25`)
  fn read_source(&mut self, name: &ModuleName) -> Option<SourceCode> {
    self.source.get(name).map(|source| SourceCode {
      source: source.clone(),
      r#type: SourceCode::MODULE,
    })
  }

  /// `std::optional<Luau::ModuleInfo> resolveModule(const ModuleInfo*, AstExpr*, const TypeCheckLimits&)`
  /// (`CLI/src/Web.cpp:27-33`)
  fn resolve_module(
    &mut self,
    _context: Option<&ModuleInfo>,
    expr: &AstExpr,
    _limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    // `if (Luau::AstExprGlobal* g = expr->as<Luau::AstExprGlobal>())`：仓库的
    // 安全下转（引用进、Option<&T> 出）。
    let g = ast_node_try_as::<AstExprGlobal>(&expr.base)?;

    // `return Luau::ModuleInfo{g->name.value}`：name.value 是指名全局的驻留
    // C 字符串，其内容成为（字符串型）ModuleName。null AstName 取空串。Luau
    // 词法器的标识符恒为 ASCII，lossy 即恒等；与上游 `std::string` 的字节保真
    // 差异仅在非 ASCII AstName 下可观察（现无此形态），若未来引入须改字节保真转换。
    Some(ModuleInfo {
      name: String::from_utf8_lossy(g.name.as_bytes())
        .into_owned()
        .into(),
      optional: false,
    })
  }

  /// `std::string getHumanReadableModuleName(const ModuleName&) const`
  /// (`CLI/src/Web.cpp:35-38`)：demo 侧模块名即人类可读名。
  fn get_human_readable_module_name(&self, name: &ModuleName) -> String {
    name.to_string()
  }

  /// `std::optional<std::string> getEnvironmentForModule(const ModuleName&) const`
  /// (`CLI/src/Web.cpp:40-43`)：demo 无附加环境声明。
  fn get_environment_for_module(&self, _name: &ModuleName) -> Option<String> {
    None
  }
}
