use alloc::{string::String, vec::Vec};
use core::ptr::NonNull;

use ulua_ast::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{
    ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_local::AstExprLocal, ast_name::AstName, ast_stat_assign::AstStatAssign,
    ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction,
    ast_stat_if::AstStatIf, ast_stat_repeat::AstStatRepeat, ast_stat_while::AstStatWhile,
    ast_visitor::AstVisitor,
  },
  rtti::ast_node_try_as,
  visit::{ast_expr_visit_ref, ast_stat_visit_ref},
};
use ulua_common::records::{
  dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, dense_hash_table::DenseDefault,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  records::{
    arena_handle::{alias, alias_opt_mut, alias_ref},
    function_info::FunctionInfo,
    lint_context::LintContext,
    lint_context_handle::LintContextHandle,
  },
};

/// C++ `LintGlobalLocal::Global` (`Analysis/src/Linter.cpp:242`).
///
/// §2：`first_ref` 的 null 哨兵收口为 `Option<NonNull>`（NonNull 编码非空，
/// `==` 保持指针同一性，与 cpp 语义逐位同构）；`function_ref` 同理。
#[derive(Debug, Clone, Default)]
pub struct Global {
  pub(crate) first_ref: Option<NonNull<AstExprGlobal>>,
  pub(crate) function_ref: Vec<NonNull<AstExprFunction>>,
  pub(crate) assigned: bool,
  pub(crate) builtin: bool,
  pub(crate) defined_in_module_scope: bool,
  pub(crate) defined_as_function: bool,
  pub(crate) read_before_written: bool,
  pub(crate) deprecated: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LintGlobalLocal<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
  pub(crate) globals: DenseHashMap<AstName, Global>,
  pub(crate) global_refs: Vec<NonNull<AstExprGlobal>>,
  pub(crate) function_stack: Vec<FunctionInfo>,
}

// —— 原 methods/lint_global_local_function_info_function_info.rs ——
impl FunctionInfo {
  fn function_info_ast(ast: &mut AstExprFunction) -> Self {
    Self {
      ast: NonNull::from(ast),
      dominated_globals: DenseHashSet::new(AstName::default()),
      conditional_execution: false,
    }
  }
}

// —— 原 methods/lint_global_local_hold_conditional_execution_hold_conditional_execution_linter.rs ——
impl<'ctx> LintGlobalLocal<'ctx> {
  pub(crate) fn set_conditional_execution(&mut self) -> bool {
    if let Some(info) = self.function_stack.last_mut()
      && !info.conditional_execution
    {
      info.conditional_execution = true;
      return true;
    }
    false
  }
}

// —— 原 methods/lint_global_local_lint_global_local.rs ——
impl DenseDefault for Global {
  fn dense_default() -> Self {
    Self::default()
  }
}

// —— 原 methods/lint_global_local_process.rs ——
impl<'ctx> LintGlobalLocal<'ctx> {
  pub fn process(context: &mut LintContext) {
    let root = context.root;
    let mut pass = LintGlobalLocal {
      context: LintContextHandle::from_ref(context),
      globals: DenseHashMap::default(),
      global_refs: Vec::new(),
      function_stack: Vec::new(),
    };
    // 内置全局预载：经宿主句柄取 builtin_globals，写入 pass 自己的表。
    let mut handle = pass.context;
    for (name, global) in handle.get().builtin_globals.iter() {
      let g = pass.globals.get_or_insert(*name);
      g.builtin = true;
      g.deprecated = global.deprecated.clone();
    }
    // root 可空（null → 跳过遍历，与原 `ast_stat_visit` 的 null 早退同构）；
    // 非空时 alias_opt_mut 收口重建独占借用，遍历为单线程串行。
    if let Some(root) = alias_opt_mut(root) {
      ast_stat_visit_ref(root, &mut pass);
    }
    pass.report();
  }
}

// —— 原 methods/lint_global_local_report.rs ——
impl<'ctx> LintGlobalLocal<'ctx> {
  pub fn report(&mut self) {
    let mut context = self.context;
    let placeholder = context.get().placeholder;
    for &gv in &self.global_refs {
      // gv 由 visit 期 `track_global_ref` 登记，指向 parser arena 内存活
      // 非空节点；alias_ref 收口重建只读借用，仅 Copy 读 name/location。
      let gv = alias_ref(gv.as_ptr());
      let g = self.globals.find(&gv.name);
      match g {
        Some(g) if g.assigned || g.builtin => {
          if let Some(replacement) = &g.deprecated {
            if replacement.is_empty() {
              emit_warning(
                context.get(),
                Code::DeprecatedGlobal,
                gv.base.base.location,
                format_args!("Global '{}' is deprecated", gv.name),
              );
            } else {
              emit_warning(
                context.get(),
                Code::DeprecatedGlobal,
                gv.base.base.location,
                format_args!(
                  "Global '{}' is deprecated, use '{}' instead",
                  gv.name, replacement
                ),
              );
            }
          }
        }
        // Unknown：未登记或「未赋值且非内置」两原分支产出同一警告，合并复用。
        _ => emit_warning(
          context.get(),
          Code::UnknownGlobal,
          gv.base.base.location,
          format_args!(
            "Unknown global '{}'; consider assigning to it first",
            gv.name
          ),
        ),
      }
    }
    for (_name, g) in self.globals.iter() {
      // function_ref 非空蕴含 first_ref 已由登记写入（读、写路径皆会登记）；
      // Option/NonNull 化后原 `(*g.first_ref)` 裸解引用全部变为字段读取。
      let first = g.first_ref.map(|f| alias_ref(f.as_ptr()));
      let top = g.function_ref.last().map(|f| alias_ref(f.as_ptr()));
      if let (Some(first), Some(top)) = (first, top)
        && g.assigned
        && first.name != placeholder
      {
        if !top.debugname.is_null() {
          emit_warning(
            context.get(),
            Code::GlobalUsedAsLocal,
            first.base.base.location,
            format_args!(
              "Global '{}' is only used in the enclosing function '{}'; consider changing it to local",
              first.name, top.debugname
            ),
          );
        } else {
          emit_warning(
            context.get(),
            Code::GlobalUsedAsLocal,
            first.base.base.location,
            format_args!(
              "Global '{}' is only used in the enclosing function defined at line {}; consider changing it to local",
              first.name,
              top.base.base.location.begin.line + 1
            ),
          );
        }
      } else if g.assigned
        && !g.read_before_written
        && !g.defined_in_module_scope
        && let Some(first) = first
        && first.name != placeholder
      {
        emit_warning(
          context.get(),
          Code::GlobalUsedAsLocal,
          first.base.base.location,
          format_args!(
            "Global '{}' is never read before being written. Consider changing it to local",
            first.name
          ),
        );
      }
    }
  }
}

// —— 原 methods/lint_global_local_track_global_ref.rs ——
impl<'ctx> LintGlobalLocal<'ctx> {
  /// cpp `trackGlobalRef(AstExprGlobal*)`：`node` 为分析期存活、由 arena 持有的
  /// 全局引用节点共享借用；全局引用以对象身份（`NonNull`）登记进
  /// `global_refs`/`first_ref`（cpp 语义），由借用地址无损还原。
  fn track_global_ref(&mut self, node: &AstExprGlobal) {
    // `name` 为 Copy 字段，先自共享借用只读取值；引用地址即节点对象身份。
    let name = node.name;
    let node = NonNull::from(node);
    let current_function_refs: Vec<NonNull<AstExprFunction>> =
      self.function_stack.iter().map(|entry| entry.ast).collect();
    self.global_refs.push(node);
    let g = self.globals.get_or_insert(name);
    if g.first_ref.is_none() {
      g.first_ref = Some(node);
      if !g.builtin {
        g.function_ref.clear();
        g.function_ref.reserve(current_function_refs.len());
        g.function_ref.extend(current_function_refs);
      }
    } else if !g.builtin {
      // 求两栈的公共前缀长度：zip 自动截到较短侧，等价 C++ 双边界判断
      let prefix = g
        .function_ref
        .iter()
        .zip(current_function_refs.iter())
        .take_while(|(a, b)| a == b)
        .count();
      g.function_ref.truncate(prefix);
    }
  }
}

// —— 原 methods/lint_global_local_visit_linter.rs ——
impl<'ctx> LintGlobalLocal<'ctx> {
  /// cpp `visit(AstExprFunction*)` 的引用化形态：`node` 即本次借用期内
  /// 存活且可独占的证明（parser 不变量：`body` 恒非空，对应 cpp
  /// `node->body->visit(visitor)`）。
  pub(crate) fn visit_ast_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    self
      .function_stack
      .push(FunctionInfo::function_info_ast(node));
    ast_stat_block_visit(node.body.get_mut(), self);
    self.function_stack.pop();
    false
  }
}
impl<'ctx> AstVisitor for LintGlobalLocal<'ctx> {
  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    self.visit_ast_expr_function(node)
  }
  fn visit_expr_global(&mut self, node: &mut AstExprGlobal) -> bool {
    self.visit_ast_expr_global(node)
  }
  fn visit_expr_local(&mut self, node: &mut AstExprLocal) -> bool {
    self.visit_ast_expr_local(node)
  }
  fn visit_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    self.visit_ast_stat_assign(node)
  }
  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    self.visit_ast_stat_function(node)
  }
  fn visit_stat_if(&mut self, node: &mut AstStatIf) -> bool {
    self.visit_ast_stat_if(node)
  }
  fn visit_stat_while(&mut self, node: &mut AstStatWhile) -> bool {
    self.visit_ast_stat_while(node)
  }
  fn visit_stat_repeat(&mut self, node: &mut AstStatRepeat) -> bool {
    self.visit_ast_stat_repeat(node)
  }
  fn visit_stat_for(&mut self, node: &mut AstStatFor) -> bool {
    self.visit_ast_stat_for(node)
  }
  fn visit_stat_for_in(&mut self, node: &mut AstStatForIn) -> bool {
    self.visit_ast_stat_for_in(node)
  }
}
impl<'ctx> LintGlobalLocal<'ctx> {
  /// cpp `visit(AstExprGlobal*)` 的引用化形态。
  pub(crate) fn visit_ast_expr_global(&mut self, node: &mut AstExprGlobal) -> bool {
    let name = node.name;
    if let Some(top) = self.function_stack.last()
      && !top.dominated_globals.contains(&name)
    {
      let g = self.globals.get_or_insert(name);
      g.read_before_written = true;
    }
    self.track_global_ref(node);
    let placeholder = self.context.get().placeholder;
    if name == placeholder {
      emit_warning(
        self.context.get(),
        Code::PlaceholderRead,
        node.base.base.location,
        format_args!("Placeholder value '_' is read here; consider using a named variable"),
      );
    }
    true
  }
  /// cpp `visit(AstExprLocal*)` 的引用化形态；`local` 已句柄化恒非空，
  /// 指向 parser 作用符号表持有、存活至 lint 结束的 `AstLocal`。
  pub(crate) fn visit_ast_expr_local(&mut self, node: &mut AstExprLocal) -> bool {
    let reads_placeholder = node.local.get().name == self.context.get().placeholder;
    if reads_placeholder {
      emit_warning(
        self.context.get(),
        Code::PlaceholderRead,
        node.base.base.location,
        format_args!("Placeholder value '_' is read here; consider using a named variable"),
      );
    }
    true
  }
}
impl<'ctx> LintGlobalLocal<'ctx> {
  /// cpp `visit(AstStatAssign*)` 的引用化形态。`vars`/`values` 的数组槽位
  /// 均为 parser 写入的非空表达式指针（cpp `AstStatAssign` 构造不变量），
  /// 逐槽经 `alias` 收口重建独占借用后走 safe 引用门面。
  pub(crate) fn visit_ast_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    let (vars, values) = (node.vars, node.values);
    for &var in vars.as_slice() {
      // var 是 parser 写入 vars 数组的非空槽位，指向 arena 内存活节点；
      // 本 pass 由 process 驱动、遍历期间独占 AST arena。
      let var = alias(var);
      if let Some(gv) = ast_node_try_as::<AstExprGlobal>(var) {
        let global_name = gv.name;
        let location = gv.base.base.location;
        let g = self.globals.get_or_insert(global_name);
        if self.function_stack.is_empty() {
          g.defined_in_module_scope = true;
        } else if !self
          .function_stack
          .last()
          .expect("is_empty() 的 else 支保证函数栈非空")
          .conditional_execution
        {
          self
            .function_stack
            .last_mut()
            .expect("is_empty() 的 else 支保证函数栈非空")
            .dominated_globals
            .insert(global_name);
        }
        if g.builtin {
          emit_warning(
            self.context.get(),
            Code::BuiltinGlobalWrite,
            location,
            format_args!(
              "Built-in global '{}' is overwritten here; consider using a local or changing the name",
              global_name
            ),
          );
        } else {
          g.assigned = true;
        }
        // `gv` 即判型命中的 AstExprGlobal 借用，直接交给 `track_global_ref`。
        self.track_global_ref(gv);
      } else if ast_node_try_as::<AstExprLocal>(var).is_some() {
        // Local writes are not local reads.
      } else {
        ast_expr_visit_ref(var, self);
      }
    }
    for &val in values.as_slice() {
      // val 与 var 同源：parser 写入 values 数组的非空槽位指针。
      ast_expr_visit_ref(alias(val), self);
    }
    false
  }
  /// cpp `visit(AstStatFunction*)` 的引用化形态；`name` 为 parser 写入的
  /// 非空表达式句柄（`Node<AstExpr>`）。
  pub(crate) fn visit_ast_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    if let Some(gv) = ast_node_try_as::<AstExprGlobal>(node.name.get()) {
      let global_name = gv.name;
      let location = gv.base.base.location;
      let g = self.globals.get_or_insert(global_name);
      if g.builtin {
        emit_warning(
          self.context.get(),
          Code::BuiltinGlobalWrite,
          location,
          format_args!(
            "Built-in global '{}' is overwritten here; consider using a local or changing the name",
            global_name
          ),
        );
      } else {
        g.assigned = true;
        g.defined_as_function = true;
        g.defined_in_module_scope = self.function_stack.is_empty();
      }
      // track_global_ref 只读 gv 身份与 name，借用与 self 借用互不冲突。
      self.track_global_ref(gv);
    }
    true
  }
  /// cpp `visit(AstStatIf*)` 的引用化形态：`condition`/`thenbody` 按 parser
  /// 不变量恒非空（`Node` 类型层承载），`elsebody` 为 `OptNode` 判空后分发。
  pub(crate) fn visit_ast_stat_if(&mut self, node: &mut AstStatIf) -> bool {
    let reset_to_false = self.set_conditional_execution();
    ast_expr_visit_ref(node.condition.get_mut(), self);
    ast_stat_block_visit(node.thenbody.get_mut(), self);
    if let Some(elsebody) = node.elsebody.get_mut() {
      ast_stat_visit_ref(elsebody, self);
    }
    if reset_to_false {
      self
        .function_stack
        .last_mut()
        .expect("reset_to_false 仅在本轮成功 push 函数帧后置位，栈顶即该帧")
        .conditional_execution = false;
    }
    false
  }
  /// cpp `visit(AstStatWhile*)` 的引用化形态；`condition`/`body` parser 恒非空。
  pub(crate) fn visit_ast_stat_while(&mut self, node: &mut AstStatWhile) -> bool {
    let reset_to_false = self.set_conditional_execution();
    ast_expr_visit_ref(node.condition.get_mut(), self);
    ast_stat_block_visit(node.body.get_mut(), self);
    if reset_to_false {
      self
        .function_stack
        .last_mut()
        .expect("reset_to_false 仅在本轮成功 push 函数帧后置位，栈顶即该帧")
        .conditional_execution = false;
    }
    false
  }
  /// cpp `visit(AstStatRepeat*)` 的引用化形态；`condition`/`body` parser 恒非空。
  pub(crate) fn visit_ast_stat_repeat(&mut self, node: &mut AstStatRepeat) -> bool {
    let reset_to_false = self.set_conditional_execution();
    ast_expr_visit_ref(node.condition.get_mut(), self);
    ast_stat_block_visit(node.body.get_mut(), self);
    if reset_to_false {
      self
        .function_stack
        .last_mut()
        .expect("reset_to_false 仅在本轮成功 push 函数帧后置位，栈顶即该帧")
        .conditional_execution = false;
    }
    false
  }
  /// cpp `visit(AstStatFor*)` 的引用化形态：`from`/`to`/`body` 恒非空，
  /// `step` 为 `OptNode` 判空后分发（对应 cpp `if (node->step)`）。
  pub(crate) fn visit_ast_stat_for(&mut self, node: &mut AstStatFor) -> bool {
    let reset_to_false = self.set_conditional_execution();
    ast_expr_visit_ref(node.from.get_mut(), self);
    ast_expr_visit_ref(node.to.get_mut(), self);
    if let Some(step) = node.step.get_mut() {
      ast_expr_visit_ref(step, self);
    }
    ast_stat_block_visit(node.body.get_mut(), self);
    if reset_to_false {
      self
        .function_stack
        .last_mut()
        .expect("reset_to_false 仅在本轮成功 push 函数帧后置位，栈顶即该帧")
        .conditional_execution = false;
    }
    false
  }
  /// cpp `visit(AstStatForIn*)` 的引用化形态；`values` 数组槽位为 parser
  /// 写入的非空指针，`body` 恒非空。
  pub(crate) fn visit_ast_stat_for_in(&mut self, node: &mut AstStatForIn) -> bool {
    let reset_to_false = self.set_conditional_execution();
    let values = node.values;
    for &value in values.as_slice() {
      // value 是 parser 写入 values 数组的非空 arena 表达式指针。
      ast_expr_visit_ref(alias(value), self);
    }
    ast_stat_block_visit(node.body.get_mut(), self);
    if reset_to_false {
      self
        .function_stack
        .last_mut()
        .expect("reset_to_false 仅在本轮成功 push 函数帧后置位，栈顶即该帧")
        .conditional_execution = false;
    }
    false
  }
}
