use alloc::{string::String, vec::Vec};
use core::ptr;

use ulua_ast::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{
    ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_local::AstExprLocal, ast_name::AstName, ast_stat_assign::AstStatAssign,
    ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction,
    ast_stat_if::AstStatIf, ast_stat_repeat::AstStatRepeat, ast_stat_while::AstStatWhile,
    ast_visitor::AstVisitor,
  },
  rtti::{ast_node_is_ptr, ast_node_try_as_ptr},
  visit::{ast_expr_visit, ast_stat_visit},
};
use ulua_common::records::{
  dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, dense_hash_table::DenseDefault,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  records::{
    function_info::FunctionInfo, lint_context::LintContext, lint_context_handle::LintContextHandle,
  },
};

/// C++ `LintGlobalLocal::Global` (`Analysis/src/Linter.cpp:242`).
#[derive(Debug, Clone, Default)]
pub struct Global<'ctx> {
  pub(crate) first_ref: Option<&'ctx AstExprGlobal>,
  pub(crate) function_ref: Vec<&'ctx AstExprFunction>,
  pub(crate) assigned: bool,
  pub(crate) builtin: bool,
  pub(crate) defined_in_module_scope: bool,
  pub(crate) defined_as_function: bool,
  pub(crate) read_before_written: bool,
  pub(crate) deprecated: Option<String>,
}

impl<'ctx> Global<'ctx> {
  pub fn first_ref(&self) -> Option<&'ctx AstExprGlobal> {
    self.first_ref
  }

  pub fn function_ref(&self) -> &[&'ctx AstExprFunction] {
    &self.function_ref
  }

  pub fn assigned(&self) -> bool {
    self.assigned
  }

  pub fn builtin(&self) -> bool {
    self.builtin
  }

  pub fn defined_in_module_scope(&self) -> bool {
    self.defined_in_module_scope
  }

  pub fn defined_as_function(&self) -> bool {
    self.defined_as_function
  }

  pub fn read_before_written(&self) -> bool {
    self.read_before_written
  }

  pub fn deprecated(&self) -> Option<&str> {
    self.deprecated.as_deref()
  }
}

#[derive(Debug, Clone)]
pub struct LintGlobalLocal<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
  pub(crate) globals: DenseHashMap<AstName, Global<'ctx>>,
  pub(crate) global_refs: Vec<&'ctx AstExprGlobal>,
  pub(crate) function_stack: Vec<FunctionInfo<'ctx>>,
}

// —— 原 methods/lint_global_local_function_info_function_info.rs ——
impl<'ctx> FunctionInfo<'ctx> {
  pub fn function_info_ast(ast: &'ctx AstExprFunction) -> Self {
    Self {
      ast,
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
impl<'ctx> DenseDefault for Global<'ctx> {
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
    // SAFETY: root 为 null 或贯穿整趟 lint pass 存活的 arena AstStat；遍历为
    // 单线程串行，宿主 LintContext 的写句柄由本 pass 独占。
    unsafe {
      ast_stat_visit(root, &mut pass);
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
      let g = self.globals.find(&gv.name);
      match g {
        None => {
          emit_warning(
            context.get(),
            Code::UnknownGlobal,
            gv.base.base.location,
            format_args!(
              "Unknown global '{}'; consider assigning to it first",
              gv.name
            ),
          );
        }
        Some(g) if !g.assigned && !g.builtin => {
          emit_warning(
            context.get(),
            Code::UnknownGlobal,
            gv.base.base.location,
            format_args!(
              "Unknown global '{}'; consider assigning to it first",
              gv.name
            ),
          );
        }
        Some(g) => {
          if let Some(ref replacement) = g.deprecated {
            if !replacement.is_empty() {
              emit_warning(
                context.get(),
                Code::DeprecatedGlobal,
                gv.base.base.location,
                format_args!(
                  "Global '{}' is deprecated, use '{}' instead",
                  gv.name,
                  replacement
                ),
              );
            } else {
              emit_warning(
                context.get(),
                Code::DeprecatedGlobal,
                gv.base.base.location,
                format_args!("Global '{}' is deprecated", gv.name),
              );
            }
          }
        }
      }
    }
    for (_name, g) in self.globals.iter() {
      if !g.function_ref.is_empty() && g.assigned && g.first_ref.unwrap().name != placeholder {
        // 条件里 `!g.function_ref.is_empty()` 蕴含 last() 命中 Some。
        let top = *g
          .function_ref
          .last()
          .expect("同一条件 !function_ref.is_empty() 蕴含非空");
        let first_ref = g.first_ref.unwrap();
        if !top.debugname.is_null() {
          emit_warning(
            context.get(),
            Code::GlobalUsedAsLocal,
            first_ref.base.base.location,
            format_args!(
              "Global '{}' is only used in the enclosing function '{}'; consider changing it to local",
              first_ref.name,
              top.debugname
            ),
          );
        } else {
          emit_warning(
            context.get(),
            Code::GlobalUsedAsLocal,
            first_ref.base.base.location,
            format_args!(
              "Global '{}' is only used in the enclosing function defined at line {}; consider changing it to local",
              first_ref.name,
              top.base.base.location.begin.line + 1
            ),
          );
        }
      } else if g.assigned
        && !g.read_before_written
        && !g.defined_in_module_scope
        && g.first_ref.unwrap().name != placeholder
      {
        let first_ref = g.first_ref.unwrap();
        emit_warning(
          context.get(),
          Code::GlobalUsedAsLocal,
          first_ref.base.base.location,
          format_args!(
            "Global '{}' is never read before being written. Consider changing it to local",
            first_ref.name
          ),
        );
      }
    }
  }
}

// —— 原 methods/lint_global_local_track_global_ref.rs ——
impl<'ctx> LintGlobalLocal<'ctx> {
  /// cpp `trackGlobalRef(AstExprGlobal*)`：`node` 为分析期存活、由 arena 持有的全局
  /// 引用节点共享借用（cpp 裸指针形参的 Rust 对应）。
  pub fn track_global_ref(&mut self, node: &'ctx AstExprGlobal) {
    let name = node.name;
    let current_function_refs = self
      .function_stack
      .iter()
      .map(|entry| entry.ast)
      .collect::<Vec<_>>();
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
        .take_while(|(a, b)| ptr::eq(*a, *b))
        .count();
      g.function_ref.truncate(prefix);
    }
  }
}

// —— 原 methods/lint_global_local_visit_linter.rs ——
impl<'ctx> LintGlobalLocal<'ctx> {
  pub(crate) fn visit_ast_expr_function(&mut self, node: &'ctx AstExprFunction) -> bool {
    self
      .function_stack
      .push(FunctionInfo::function_info_ast(node));
    let mut body = node.body;
    ast_stat_block_visit(body.get_mut(), self);
    self.function_stack.pop();
    false
  }
}
impl<'ctx> AstVisitor for LintGlobalLocal<'ctx> {
  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    // Safety: node 在遍历期存活于 AST arena，其生命周期覆盖 'ctx。
    let node_ref: &'ctx AstExprFunction = unsafe { &*(node as *const AstExprFunction) };
    self.visit_ast_expr_function(node_ref)
  }
  fn visit_expr_global(&mut self, node: &mut AstExprGlobal) -> bool {
    // Safety: node 在遍历期存活于 AST arena，其生命周期覆盖 'ctx。
    let node_ref: &'ctx AstExprGlobal = unsafe { &*(node as *const AstExprGlobal) };
    self.visit_ast_expr_global(node_ref)
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
  pub(crate) fn visit_ast_expr_global(&mut self, node: &'ctx AstExprGlobal) -> bool {
    let name = node.name;
    if !self.function_stack.is_empty()
      && !self
        .function_stack
        .last()
        .expect("&& 前一子句已判 !function_stack.is_empty()")
        .dominated_globals
        .contains(&name)
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

  pub(crate) fn visit_ast_expr_local(&mut self, node: &AstExprLocal) -> bool {
    let local = node.local;
    let reads_placeholder = local.get().name == self.context.get().placeholder;
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

/// cpp `var->is<AstExprLocal>()` 的等价：RTTI 边界门面 `ast_node_is_ptr` 把 null
/// 折叠为 `false`（非空才只读 class_index，绝不解引用空指针）。
///
/// # Safety
/// `var` 须为 null 或指向 arena 存活 repr(C) 节点（同 `ast_node_is_ptr`）。
#[inline]
unsafe fn ast_node_is_local(var: *mut AstExpr) -> bool {
  unsafe { ast_node_is_ptr::<AstExprLocal>(var) }
}

impl<'ctx> LintGlobalLocal<'ctx> {
  pub(crate) fn visit_ast_stat_assign(&mut self, node: &AstStatAssign) -> bool {
    let (vars, values) = (node.vars, node.values);
    for &var in vars.as_slice() {
      // Safety: var 是 parser 写入 vars 数组的非空槽位，指向 arena 内存活节点；
      // try_as_ptr 只读偏移 0 判型，命中返回的 &'static 引用借用同一节点，
      // 本次循环迭代内无人改写该 AST 内存。
      if let Some(gv) = unsafe { ast_node_try_as_ptr::<AstExprGlobal>(var) } {
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
        self.track_global_ref(gv);
      } else if unsafe { ast_node_is_local(var) } {
        // Safety: var 为 parser 保证非空的 arena 存活表达式指针，只读判定。
        // Local writes are not local reads.
      } else {
        // Safety: var 为存活非空表达式指针；ast_expr_visit 要求 null 或存活
        // 节点且调用方独占该 arena——本 pass 由 process 驱动、遍历期间无人并
        // 行写 AST，独占成立。
        unsafe {
          ast_expr_visit(var, self);
        }
      }
    }
    for &val in values.as_slice() {
      // Safety: val 与 var 同源——parser 写入 values 数组的非空槽位指针，
      // 独占的 arena 内存活节点，满足 ast_expr_visit 契约。
      unsafe {
        ast_expr_visit(val, self);
      }
    }
    false
  }

  pub(crate) fn visit_ast_stat_function(&mut self, node: &AstStatFunction) -> bool {
    let name = node.name.as_ptr();
    // Safety: name 是存活非空的表达式槽位指针，判型命中后的 &'static 引用
    // 借用同一 arena 节点，仅在本 if 体内使用。
    if let Some(gv) = unsafe { ast_node_try_as_ptr::<AstExprGlobal>(name) } {
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
      self.track_global_ref(gv);
    }
    true
  }

  pub(crate) fn visit_ast_stat_if(&mut self, node: &mut AstStatIf) -> bool {
    let reset_to_false = self.set_conditional_execution();
    unsafe {
      ast_expr_visit(node.condition.as_ptr(), self);
      ast_stat_block_visit(node.thenbody.get_mut(), self);
      if !node.elsebody.is_null() {
        ast_stat_visit(node.elsebody.as_ptr(), self);
      }
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

  pub(crate) fn visit_ast_stat_while(&mut self, node: &mut AstStatWhile) -> bool {
    let reset_to_false = self.set_conditional_execution();
    unsafe {
      ast_expr_visit(node.condition.as_ptr(), self);
      ast_stat_block_visit(node.body.get_mut(), self);
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

  pub(crate) fn visit_ast_stat_repeat(&mut self, node: &mut AstStatRepeat) -> bool {
    let reset_to_false = self.set_conditional_execution();
    unsafe {
      ast_expr_visit(node.condition.as_ptr(), self);
      ast_stat_block_visit(node.body.get_mut(), self);
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

  pub(crate) fn visit_ast_stat_for(&mut self, node: &mut AstStatFor) -> bool {
    let reset_to_false = self.set_conditional_execution();
    unsafe {
      ast_expr_visit(node.from.as_ptr(), self);
      ast_expr_visit(node.to.as_ptr(), self);
      if node.step.is_some() {
        ast_expr_visit(node.step.as_ptr(), self);
      }
      ast_stat_block_visit(node.body.get_mut(), self);
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

  pub(crate) fn visit_ast_stat_for_in(&mut self, node: &mut AstStatForIn) -> bool {
    let reset_to_false = self.set_conditional_execution();
    let values = node.values;
    for &value in values.as_slice() {
      unsafe {
        ast_expr_visit(value, self);
      }
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
