use alloc::{string::String, vec::Vec};
use core::ptr::{from_mut, from_ref, null_mut};

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
#[derive(Debug, Clone)]
pub struct Global {
  pub(crate) first_ref: *mut AstExprGlobal,
  pub(crate) function_ref: Vec<*mut AstExprFunction>,
  pub(crate) assigned: bool,
  pub(crate) builtin: bool,
  pub(crate) defined_in_module_scope: bool,
  pub(crate) defined_as_function: bool,
  pub(crate) read_before_written: bool,
  pub(crate) deprecated: Option<String>,
}

impl Default for Global {
  fn default() -> Self {
    Self {
      first_ref: null_mut(),
      function_ref: Vec::new(),
      assigned: false,
      builtin: false,
      defined_in_module_scope: false,
      defined_as_function: false,
      read_before_written: false,
      deprecated: None,
    }
  }
}

impl Global {
  pub fn first_ref(&self) -> *mut AstExprGlobal {
    self.first_ref
  }

  pub fn function_ref(&self) -> &[*mut AstExprFunction] {
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
  pub(crate) globals: DenseHashMap<AstName, Global>,
  pub(crate) global_refs: Vec<*mut AstExprGlobal>,
  pub(crate) function_stack: Vec<FunctionInfo>,
}

// —— 原 methods/lint_global_local_function_info_function_info.rs ——
impl FunctionInfo {
  pub fn function_info_ast(ast: *mut AstExprFunction) -> Self {
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
      // Safety: `gv` 取自 `global_refs`，由 visit 期 `track_global_ref` 从 visitor
      // 交来的 `&mut AstExprGlobal`（`from_mut` 转换）登记，指向 parser arena 内
      // 存活且非空的节点；仅 Copy 读其 `name` 作查表键，`find` 借用 `&self.globals`
      // （与 AST arena 地址空间不相交），无别名冲突。
      let g = unsafe { self.globals.find(&(*gv).name) };
      match g {
        None => unsafe {
          // Safety: `gv` 为 global_refs 登记的存活非空 AstExprGlobal，location/name
          // 为 Copy 字段读取。
          emit_warning(
            context.get(),
            Code::UnknownGlobal,
            (*gv).base.base.location,
            format_args!(
              "Unknown global '{}'; consider assigning to it first",
              (*gv).name
            ),
          );
        },
        Some(g) if !g.assigned && !g.builtin => unsafe {
          // Safety: `gv` 为 global_refs 登记的存活非空 AstExprGlobal，location/name
          // 为 Copy 字段读取。
          emit_warning(
            context.get(),
            Code::UnknownGlobal,
            (*gv).base.base.location,
            format_args!(
              "Unknown global '{}'; consider assigning to it first",
              (*gv).name
            ),
          );
        },
        Some(g) => {
          if let Some(ref replacement) = g.deprecated {
            unsafe {
              // Safety: `gv` 存活非空，location/name 为 Copy 字段读取。
              if !replacement.is_empty() {
                emit_warning(
                  context.get(),
                  Code::DeprecatedGlobal,
                  (*gv).base.base.location,
                  format_args!(
                    "Global '{}' is deprecated, use '{}' instead",
                    (*gv).name,
                    replacement
                  ),
                );
              } else {
                emit_warning(
                  context.get(),
                  Code::DeprecatedGlobal,
                  (*gv).base.base.location,
                  format_args!("Global '{}' is deprecated", (*gv).name),
                );
              }
            }
          }
        }
      }
    }
    for (_name, g) in self.globals.iter() {
      // Safety: `g.function_ref` 非空蕴含该全局至少被 `track_global_ref` 登记过一次
      // （读、写路径皆会登记），故 `first_ref` 已由登记写入为指向 arena 内存活
      // `AstExprGlobal` 的非空指针；条件里只 Copy 读其 `name` 与 placeholder 比较。
      if !g.function_ref.is_empty() && g.assigned && unsafe { (*g.first_ref).name } != placeholder {
        // 条件里 `!g.function_ref.is_empty()` 蕴含 last() 命中 Some。
        let top = *g
          .function_ref
          .last()
          .expect("同一条件 !function_ref.is_empty() 蕴含非空");
        unsafe {
          // Safety: `top` 取自 `function_ref.last()`，元素由 `track_global_ref` 从
          // function_stack 各 entry 的存活 `&mut AstExprFunction`（`from_mut`）推入，
          // 非空且比本 pass 长寿；`debugname` 先判 `is_null` 再格式化，else 分支读
          // `base.base.location.begin.line` 均为 Copy 字段读取。
          if !(*top).debugname.is_null() {
            emit_warning(
              context.get(),
              Code::GlobalUsedAsLocal,
              (*g.first_ref).base.base.location,
              format_args!(
                "Global '{}' is only used in the enclosing function '{}'; consider changing it to local",
                (*g.first_ref).name,
                (*top).debugname
              ),
            );
          } else {
            emit_warning(
              context.get(),
              Code::GlobalUsedAsLocal,
              (*g.first_ref).base.base.location,
              format_args!(
                "Global '{}' is only used in the enclosing function defined at line {}; consider changing it to local",
                (*g.first_ref).name,
                (*top).base.base.location.begin.line + 1
              ),
            );
          }
        }
      } else if g.assigned
        && !g.read_before_written
        && !g.defined_in_module_scope
        // Safety: `g.assigned` 为真意味着曾走过写入路径 → `track_global_ref` 登记 →
        // `first_ref` 非空且指向 arena 内存活 `AstExprGlobal`；仅 Copy 读 `name` 比较。
        && unsafe { (*g.first_ref).name } != placeholder
      {
        unsafe {
          // Safety: `first_ref` 由上面的 assigned 蕴含为非空存活 AstExprGlobal，
          // location/name 为 Copy 字段读取。
          emit_warning(
            context.get(),
            Code::GlobalUsedAsLocal,
            (*g.first_ref).base.base.location,
            format_args!(
              "Global '{}' is never read before being written. Consider changing it to local",
              (*g.first_ref).name
            ),
          );
        }
      }
    }
  }
}

// —— 原 methods/lint_global_local_track_global_ref.rs ——
impl<'ctx> LintGlobalLocal<'ctx> {
  /// cpp `trackGlobalRef(AstExprGlobal*)`：`node` 为分析期存活、由 arena 持有的全局
  /// 引用节点共享借用（cpp 裸指针形参的 Rust 对应）。全局引用以裸指针身份登记进
  /// `global_refs`/`first_ref`（cpp 语义），由该借用地址安全还原。
  pub fn track_global_ref(&mut self, node: &AstExprGlobal) {
    // `name` 为 Copy 字段，先自共享借用只读取值；再以引用地址还原裸指针对象身份。
    let name = node.name;
    let node = from_ref(node).cast_mut();
    let current_function_refs = self
      .function_stack
      .iter()
      .map(|entry| entry.ast)
      .collect::<Vec<_>>();
    self.global_refs.push(node);
    let g = self.globals.get_or_insert(name);
    if g.first_ref.is_null() {
      g.first_ref = node;
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
  /// # Safety
  /// `node` 必须非空并指向 parser arena 中存活的 `AstExprFunction`（本 crate 内
  /// 唯一入口是下方 `AstVisitor::visit_expr_function`，其实参由 `from_mut` 取自
  /// visitor 分发给出的 `&mut` 借用）。另依赖 parser 不变量：函数节点的 `body`
  /// 恒为非空 `AstStatBlock`（cpp `node->body->visit(visitor)` 的同款前提）。
  pub(crate) fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
    let node_ref = unsafe { &mut *node };
    self
      .function_stack
      .push(FunctionInfo::function_info_ast(node));
    ast_stat_block_visit(node_ref.body.get_mut(), self);
    self.function_stack.pop();
    false
  }
}
impl<'ctx> AstVisitor for LintGlobalLocal<'ctx> {
  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    self.visit_ast_expr_function(from_mut(node))
  }
  fn visit_expr_global(&mut self, node: &mut AstExprGlobal) -> bool {
    self.visit_ast_expr_global(from_mut(node))
  }
  fn visit_expr_local(&mut self, node: &mut AstExprLocal) -> bool {
    self.visit_ast_expr_local(from_mut(node))
  }
  fn visit_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    self.visit_ast_stat_assign(from_mut(node))
  }
  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    self.visit_ast_stat_function(from_mut(node))
  }
  fn visit_stat_if(&mut self, node: &mut AstStatIf) -> bool {
    self.visit_ast_stat_if(from_mut(node))
  }
  fn visit_stat_while(&mut self, node: &mut AstStatWhile) -> bool {
    self.visit_ast_stat_while(from_mut(node))
  }
  fn visit_stat_repeat(&mut self, node: &mut AstStatRepeat) -> bool {
    self.visit_ast_stat_repeat(from_mut(node))
  }
  fn visit_stat_for(&mut self, node: &mut AstStatFor) -> bool {
    self.visit_ast_stat_for(from_mut(node))
  }
  fn visit_stat_for_in(&mut self, node: &mut AstStatForIn) -> bool {
    self.visit_ast_stat_for_in(from_mut(node))
  }
}
impl<'ctx> LintGlobalLocal<'ctx> {
  /// # Safety
  /// `node` 必须非空并指向遍历期间存活的 `AstExprGlobal`（唯一入口是下方
  /// `visit_expr_global` 的 `from_mut` 转换）。
  pub(crate) fn visit_ast_expr_global(&mut self, node: *mut AstExprGlobal) -> bool {
    // Safety: node 按函数级契约为存活非空指针，读出 Copy 字段 name 仅一次
    // 偏移内取值，不产生引用。
    let name = unsafe { (*node).name };
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
    // Safety: `node` 是本函数级契约承诺的存活非空 `AstExprGlobal` 指针，重建只读
    // 借用交给 `track_global_ref`（其内部仅把该身份存入 global_refs/first_ref）。
    self.track_global_ref(unsafe { &*node });
    let placeholder = self.context.get().placeholder;
    if name == placeholder {
      emit_warning(
        self.context.get(),
        Code::PlaceholderRead,
        // Safety: node 存活非空（函数级契约），location 为 Copy 字段读取。
        unsafe { (*node).base.base.location },
        format_args!("Placeholder value '_' is read here; consider using a named variable"),
      );
    }
    true
  }
  /// # Safety
  /// `node` 必须非空并指向遍历期间存活的 `AstExprLocal`（唯一入口是
  /// `visit_expr_local` 的 `from_mut` 转换）；`node.local` 允许为 null，非空
  /// 时必须指向 parser 作用符号表持有、存活至 lint 结束的 `AstLocal`。
  pub(crate) fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
    // Safety: node 按函数级契约存活非空；local 槽已句柄化恒非空，指向 parser
    // arena 里存活的 AstLocal（cpp Parser::allocLocal 的分配区域），只读 name 字段
    //（旧 is_null 死守卫随类型消失，等价 cpp 无判空形态）。
    let local = unsafe { (*node).local };
    let reads_placeholder = local.get().name == self.context.get().placeholder;
    if reads_placeholder {
      emit_warning(
        self.context.get(),
        Code::PlaceholderRead,
        // Safety: node 存活非空，location 是 Copy 字段读取。
        unsafe { (*node).base.base.location },
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
  /// # Safety
  /// `node` 必须非空并指向遍历期间存活的 `AstStatAssign`，其 `vars`/`values`
  /// 两个 `AstArray` 的槽位均为 parser 写入的非空表达式指针（cpp
  /// `AstStatAssign` 构造不变量）。唯一入口是 `visit_stat_assign` 的
  /// `from_mut` 转换。
  pub(crate) fn visit_ast_stat_assign(&mut self, node: *mut AstStatAssign) -> bool {
    // Safety: node 由 visit_stat_assign 以 `from_mut(&mut _)` 传入，指向遍历经
    // 验内存活的 repr(C) AstStatAssign；这里只 Copy 两个 (data, size) 指针对。
    let (vars, values) = unsafe { ((*node).vars, (*node).values) };
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
        // `gv` 即上面的 `ast_node_try_as_ptr` 命中结果（`var` 的 AstExprGlobal 借用），
        // 直接交给 `track_global_ref`，省掉裸指针 cast。
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
  /// # Safety
  /// `node` 必须非空并指向遍历期间存活的 `AstStatFunction`，其 `name` 为
  /// parser 写入的非空表达式指针；唯一入口是 `visit_stat_function` 的
  /// `from_mut` 转换。
  pub(crate) fn visit_ast_stat_function(&mut self, node: *mut AstStatFunction) -> bool {
    // Safety: node 由 visit_stat_function 的 `from_mut(&mut _)` 传入，存活非空；
    // name 读出的是指针值。
    let name = unsafe { (*node).name };
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
      // try_as_ptr 命中已证明 gv 是存活 AstExprGlobal 的只读引用，直接交给
      // track_global_ref。
      self.track_global_ref(gv);
    }
    true
  }
  /// # Safety
  /// `node` 必须非空并指向遍历期间存活的 `AstStatIf`：`condition` 与
  /// `thenbody` 按 parser 不变量恒为非空，`elsebody` 可为 null（块内已判）。
  /// 唯一入口是 `visit_stat_if` 的 `from_mut` 转换。
  pub(crate) fn visit_ast_stat_if(&mut self, node: *mut AstStatIf) -> bool {
    let reset_to_false = self.set_conditional_execution();
    // Safety: node 存活非空（函数级契约）。condition 为 parser 保证的非空表达式
    // 指针；thenbody 恒非空，其 `&mut` 再借用只活过这次 block 分发（本 pass
    // 独占 AST arena，无并发别名）；elsebody 在调用 ast_stat_visit 前判空，
    // 两个 visit 的「null 或存活节点 + arena 独占」契约因此都满足。
    let node_ref = unsafe { &mut *node };
    unsafe {
      ast_expr_visit(node_ref.condition.as_ptr(), self);
      ast_stat_block_visit(node_ref.thenbody.get_mut(), self);
      if !node_ref.elsebody.is_null() {
        ast_stat_visit(node_ref.elsebody.as_ptr(), self);
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
  /// # Safety
  /// `node` 必须非空并指向遍历期间存活的 `AstStatWhile`，其 `condition`/
  /// `body` 由 parser 恒置为非空。唯一入口是 `visit_stat_while` 的
  /// `from_mut` 转换。
  pub(crate) fn visit_ast_stat_while(&mut self, node: *mut AstStatWhile) -> bool {
    let reset_to_false = self.set_conditional_execution();
    // Safety: node 存活非空（函数级契约）；condition、body 都是 parser 写入的
    // 非空 arena 指针，本 pass 独占 AST arena，visit 契约（null 或存活节点）
    // 与 `&mut` 再借用的唯一性都成立。
    let node_ref = unsafe { &mut *node };
    unsafe {
      ast_expr_visit(node_ref.condition.as_ptr(), self);
      ast_stat_block_visit(node_ref.body.get_mut(), self);
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
  /// # Safety
  /// `node` 必须非空并指向遍历期间存活的 `AstStatRepeat`，其 `condition`/
  /// `body` 由 parser 恒置为非空。唯一入口是 `visit_stat_repeat` 的
  /// `from_mut` 转换。
  pub(crate) fn visit_ast_stat_repeat(&mut self, node: *mut AstStatRepeat) -> bool {
    let reset_to_false = self.set_conditional_execution();
    // Safety: node 存活非空（函数级契约）；condition、body 已句柄化为 Node
    // （parser 非空由类型层承载），as_ptr/get_mut 桥交既有指针/引用门面。
    let node_ref = unsafe { &mut *node };
    unsafe {
      ast_expr_visit(node_ref.condition.as_ptr(), self);
      ast_stat_block_visit(node_ref.body.get_mut(), self);
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
  /// # Safety
  /// `node` 必须非空并指向遍历期间存活的 `AstStatFor`：`from`/`to`/`body`
  /// 按 parser 不变量非空，`step` 可为 null（块内已判）。唯一入口是
  /// `visit_stat_for` 的 `from_mut` 转换。
  pub(crate) fn visit_ast_stat_for(&mut self, node: *mut AstStatFor) -> bool {
    let reset_to_false = self.set_conditional_execution();
    // Safety: node 存活非空（函数级契约）。from/to/body 已句柄化为 Node（非空
    // 由类型层承载），as_ptr/get_mut 桥交既有指针/引用门面；step 落可空 OptNode
    // 先判空再 visit（对应 cpp `if (node->step)`），且本 pass 对 AST arena 有
    // 独占访问。
    let node_ref = unsafe { &mut *node };
    unsafe {
      ast_expr_visit(node_ref.from.as_ptr(), self);
      ast_expr_visit(node_ref.to.as_ptr(), self);
      if node_ref.step.is_some() {
        ast_expr_visit(node_ref.step.as_ptr(), self);
      }
      ast_stat_block_visit(node_ref.body.get_mut(), self);
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
  /// # Safety
  /// `node` 必须非空并指向遍历期间存活的 `AstStatForIn`，其 `values` 数组
  /// 槽位与 `body` 均为 parser 写入的非空指针。唯一入口是
  /// `visit_stat_for_in` 的 `from_mut` 转换。
  pub(crate) fn visit_ast_stat_for_in(&mut self, node: *mut AstStatForIn) -> bool {
    let reset_to_false = self.set_conditional_execution();
    // Safety: node 由 visit_stat_for_in 的 `from_mut(&mut _)` 传入，存活非空；
    // values 读出的是 (data, size) 指针对拷贝。
    let values = unsafe { (*node).values };
    for &value in values.as_slice() {
      // Safety: value 是 parser 写入 values 数组的非空 arena 表达式指针，
      // 满足 ast_expr_visit 的「存活节点 + arena 独占」契约。
      unsafe {
        ast_expr_visit(value, self);
      }
    }
    // Safety: body 按 parser 不变量非空，&mut 再借用只活过这次块分发，
    // 期间 AST arena 由本 pass 独占。
    unsafe {
      ast_stat_block_visit(&mut (*node).body, self);
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
}
