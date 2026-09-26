use core::str::from_utf8;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
    ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal,
    ast_stat_function::AstStatFunction, ast_stat_type_alias::AstStatTypeAlias,
    ast_type_reference::AstTypeReference,
  },
  rtti::{AstNodePtr, ast_node_try_as_ptr},
};

use crate::{
  records::{symbol::Symbol, usage_finder::UsageFinder},
  type_aliases::name_type::Name,
};

impl UsageFinder {
  /// # Safety
  /// `expr` 必须非空并指向 parser arena 中存活的 `AstExprConstantString`（crate
  /// 内唯一入口是 `AstVisitor::visit_expr_constant_string` 的 `from_mut` 转
  /// 换）；其 `value` 的 {指针, 长度} 成对来自 parser，字节区域在遍历期存活。
  pub(crate) fn visit_ast_expr_constant_string(
    &mut self,
    expr: *mut AstExprConstantString,
  ) -> bool {
    // Safety: expr 由 visitor 分发的 `&mut` 经 from_mut 而来，`&*expr` 是对同
    // 一位置的共享再借用：非空、对齐且节点存活；as_bytes 只读 value 缓冲区，
    // 与 `self`（收集器状态）不相交。
    let expr_ref = unsafe { &*expr };
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
    // Safety: alias 存活且对齐（函数级契约），`&*alias` 为同一位置的共享再借
    // 用；只读 name 并 to_string 拷贝，不留指向节点的引用。
    let alias_ref = unsafe { &*alias };
    let name_str = alias_ref.name.as_str_or_empty().to_string();
    self.declared_aliases.insert(Name::from(name_str));
    true
  }

  /// # Safety
  /// `ref_` 必须非空并指向 parser arena 中存活的 `AstTypeReference`（唯一入口
  /// `AstVisitor::visit_type_reference`）；`prefix`/`name` 内的 C 字符串由
  /// `AstNameTable` 持有，遍历期存活。
  pub(crate) fn visit_ast_type_reference(&mut self, ref_: *mut AstTypeReference) -> bool {
    // Safety: ref_ 由 visitor 的 `&mut AstTypeReference` 经 from_mut 转来，
    // `&*ref_` 共享再借用合法；读出后即拷贝为 String，引用不长于本帧的独占
    // 借用，无并发写路径。
    let ref_ = unsafe { &*ref_ };
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
    // Safety: dfg 裸指针来自构造本 finder 的调用方（函数级契约：非空、指向存
    // 活的 DataFlowGraph），下面的查询全部经 `&self` 只读路径进行，共享引用
    // 与 `&mut self`（finder 收集状态）指向不相交的对象。
    let dfg = unsafe { &*self.dfg };

    if let Some(opt) = dfg.get_def_optional(expr) {
      self.mentioned_defs.insert(opt);
    }

    let ref_ = dfg.get_refinement_key(expr);
    if !ref_.is_null() {
      // Safety: 已判非空；ref_ 指向 DFG 自持内存中的 RefinementKey，其存活随
      // dfg 引用一同成立；def() 只按值拷出裸 DefId。
      self.mentioned_defs.insert(unsafe { (*ref_).def() });
    }

    // Safety: expr 是 visitor 分发来的存活 `AstExpr` 指针；ast_node_try_as_ptr
    // 判空并按 class index 下转，命中后按 repr(C) 基址重合把同一地址借为
    // `&'static AstExprLocal`（cpp `expr->as<AstExprLocal>()` 的 const 下转
    // 同款）；`local` 字段只被拷成裸指针入表，遍历期该节点无写路径。
    if let Some(local) = unsafe { ast_node_try_as_ptr::<AstExprLocal>(expr.as_ast_node()) } {
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
    // Safety: dfg 非空且指向存活图（函数级契约），get_def 只读查询；共享引用
    // 与 `&mut self` 收集状态不相交。
    let dfg = unsafe { &*self.dfg };
    // Safety: global 由 visitor `from_mut` 保证存活，`name` 是拷出的
    // `*const c_char` 包装值，字符串本体归 AstNameTable 持有。
    let name = unsafe { (*global).name };
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
    // Safety: function 由 visitor 分发的 `&mut` 经 from_mut 而来，`&*function`
    // 是对同一位置的合法共享再借用；下面只读 name 指针值，不写节点。
    let function_ref = unsafe { &*function };

    let name_expr = function_ref.name;
    // null 判定与类型下转一并折叠进 try_as_ptr（其契约：null → None），与 cpp
    // `name->as<AstExprGlobal>()` 判空后 const 下转同款语义。
    if let Some(global) = unsafe {
      // Safety: name_expr 为 null 或指向 arena 遍历期存活的 `AstExpr` 节点
      // （函数级契约）；ast_node_try_as_ptr 先判 null 再按 class index 判定，
      // 命中后借 repr(C) 基址重合把同一地址转为共享引用，该左值子树在本帧
      // 只有这一条只读路径，与 `self`（收集器状态）不相交。
      ast_node_try_as_ptr::<AstExprGlobal>(name_expr.as_ast_node())
    } {
      let global_name = global.name;
      self.global_functions_referenced.push(global_name);
    }

    true
  }
}
