//! `usage_finder` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{string::ToString, vec::Vec};
use core::str::from_utf8;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
  ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal, ast_stat_function::AstStatFunction,
  ast_stat_type_alias::AstStatTypeAlias, ast_type_reference::AstTypeReference,
  node_handle::OptNode,
};

use crate::{
  records::{
    arena_handle::alias_ref, data_flow_graph::DataFlowGraph, symbol::Symbol,
    usage_finder::UsageFinder,
  },
  type_aliases::name_type::Name,
};

impl UsageFinder {
  pub fn new(dfg: *mut DataFlowGraph) -> Self {
    // Field initializers: referencedBindings{""}, referencedImportedBindings{{"", ""}}.
    // We explicitly suggest that the usage finder populate types for instance and enum by default
    // These are common enough types that sticking them in the environment is a good idea
    // and it lets magic functions work correctly too.
    let referenced_bindings: Vec<Name> =
      alloc::vec!["".to_string(), "Instance".to_string(), "Enum".to_string()];
    let referenced_imported_bindings: Vec<(Name, Name)> =
      alloc::vec![("".to_string(), "".to_string())];

    UsageFinder {
      dfg,
      declared_aliases: Default::default(),
      local_bindings_referenced: Vec::new(),
      mentioned_defs: Default::default(),
      referenced_bindings,
      referenced_imported_bindings,
      global_defs_to_pre_populate: Vec::new(),
      global_functions_referenced: Vec::new(),
      symbols_to_refine: Vec::new(),
    }
  }
}

impl UsageFinder {
  /// # Safety
  /// `expr` 必须非空并指向 parser arena 中存活的 `AstExprConstantString`（crate
  /// 内唯一入口是 `AstVisitor::visit_expr_constant_string` 的 `from_mut` 转
  /// 换）；其 `value` 的 {指针, 长度} 成对来自 parser，字节区域在遍历期存活。
  pub(crate) fn visit_ast_expr_constant_string(
    &mut self,
    expr: *mut AstExprConstantString,
  ) -> bool {
    let expr_ref = alias_ref(expr);
    let value_slice = expr_ref.value.as_bytes();
    let name = from_utf8(value_slice).unwrap_or("");
    self.referenced_bindings.push(name.into());
    true
  }

  pub fn visit_ast_type(&mut self) -> bool {
    true
  }

  pub fn visit_ast_type_pack(&mut self) -> bool {
    true
  }

  /// # Safety
  /// `alias` 必须非空并指向 parser arena 中存活的 `AstStatTypeAlias`（唯一入口
  /// `AstVisitor::visit_stat_type_alias`）；其 `name` 指向 `AstNameTable` 持有
  /// 的字符串，生命周期覆盖本次遍历。
  pub(crate) fn visit_ast_stat_type_alias(&mut self, alias: *mut AstStatTypeAlias) -> bool {
    let alias_ref = alias_ref(alias);
    let name_str = alias_ref.name.as_str_or_empty().to_string();
    self.declared_aliases.insert(Name::from(name_str));
    true
  }

  /// # Safety
  /// `ref_` 必须非空并指向 parser arena 中存活的 `AstTypeReference`（唯一入口
  /// `AstVisitor::visit_type_reference`）；`prefix`/`name` 内的 C 字符串由
  /// `AstNameTable` 持有，遍历期存活。
  pub(crate) fn visit_ast_type_reference(&mut self, ref_: *mut AstTypeReference) -> bool {
    let ref_ = alias_ref(ref_);
    if let Some(prefix) = ref_.prefix {
      let prefix_value = prefix.as_str_or_empty().to_string();
      let name_value = ref_.name.as_str_or_empty().to_string();
      self
        .referenced_imported_bindings
        .push((prefix_value, name_value));
    } else {
      let name_value = ref_.name.as_str_or_empty().to_string();
      self.referenced_bindings.push(name_value);
    }
    true
  }

  /// # Safety
  /// `expr` 必须非空并指向 parser arena 中存活的 `AstExpr`（由
  /// `AstVisitor::visit_expr` 分发保证）；`self.dfg` 必须已由
  /// `UsageFinder::new` 写入指向存活 `DataFlowGraph` 的裸指针，且在遍历期间
  /// 无人改写该图（所有查询接口都只读）。
  pub fn visit_ast_expr(&mut self, expr: *mut AstExpr) -> bool {
    let dfg = alias_ref(self.dfg);

    if let Some(opt) = dfg.get_def_optional(expr) {
      self.mentioned_defs.insert(opt);
    }

    let ref_ = dfg.get_refinement_key(expr);
    if !ref_.is_null() {
      self.mentioned_defs.insert(alias_ref(ref_).def());
    }

    // expr 是 visitor 分发来的存活 `AstExpr` 指针：局部句柄折叠判空，下转走
    // 生命周期正确的 `try_as`（cpp `expr->as<AstExprLocal>()` 的 const 下转
    // 同款）；`local` 字段只被拷成裸指针入表，遍历期该节点无写路径。
    let expr_slot = OptNode::from_ptr(expr);
    if let Some(local) = expr_slot.try_as::<AstExprLocal>() {
      let def = dfg.get_def(expr as *const AstExpr);
      // local 槽已句柄化恒非空；入表键值/符号为既有裸指针形态，经 as_ptr 桥接。
      self
        .local_bindings_referenced
        .push((def, local.local.as_ptr()));
      self
        .symbols_to_refine
        .push((def, Symbol::from_local(local.local.as_ptr())));
    }

    true
  }

  /// # Safety
  /// `global` 必须非空并指向 parser arena 中存活的 `AstExprGlobal`（唯一入口
  /// `AstVisitor::visit_expr_global`）；`self.dfg` 同 [`Self::visit_ast_expr`]
  /// 的约定：指向遍历期只读的存活 `DataFlowGraph`。
  // C++ `bool UsageFinder::visit(AstExprGlobal* global)` (FragmentAutocomplete.cpp:641-647):
  //   globalDefsToPrePopulate.emplace_back(global->name, dfg->getDef(global));
  //   auto def = dfg->getDef(global);
  //   symbolsToRefine.emplace_back(def, Symbol(global->name));
  //   return true;
  pub(crate) fn visit_ast_expr_global(&mut self, global: *mut AstExprGlobal) -> bool {
    let dfg = alias_ref(self.dfg);
    let name = alias_ref(global).name;
    let def = dfg.get_def(global as *const AstExpr);

    self.global_defs_to_pre_populate.push((name, def));
    self
      .symbols_to_refine
      .push((def, Symbol::from_global(name)));

    true
  }

  /// # Safety
  /// `function` 必须非空并指向 parser arena 中存活的 `AstStatFunction`（唯一
  /// 入口 `AstVisitor::visit_stat_function`）；其 `name` 表达式（若非空）指向
  /// 遍历期存活的 `AstExpr` 节点。
  pub(crate) fn visit_ast_stat_function(&mut self, function: *mut AstStatFunction) -> bool {
    let function_ref = alias_ref(function);

    let name_expr = function_ref.name;
    // name 槽已句柄化（可空性由 NonNull→Option 承载），`Node::try_as` 判型
    // 下转借用半径由本帧 `function_ref` 供给，与 cpp
    // `name->as<AstExprGlobal>()` const 下转同款语义。
    if let Some(global) = name_expr.try_as::<AstExprGlobal>() {
      let global_name = global.name;
      self.global_functions_referenced.push(global_name);
    }

    true
  }
}
