use alloc::string::String;
use core::ptr::from_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_stat_block::AstStatBlock, ast_stat_function::AstStatFunction,
    ast_stat_local_function::AstStatLocalFunction, ast_visitor::AstVisitor, location::Location,
    node_handle::OptNode,
  },
  rtti::ast_node_try_as,
  visit::ast_stat_visit_ref,
};
use ulua_common::records::dense_hash_map::DenseHashMap;
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{
    arena_handle::alias_ref, lint_context::LintContext, lint_context_handle::LintContextHandle,
  },
};

#[derive(Debug, Clone)]
pub struct LintDuplicateFunction<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
  pub(crate) defns: DenseHashMap<String, Location>,
}

impl<'ctx> AstVisitor for LintDuplicateFunction<'ctx> {
  fn visit_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    self.visit_ast_stat_block(from_mut(node))
  }
}

// —— 原 methods/lint_duplicate_function_build_name.rs ——
impl<'ctx> LintDuplicateFunction<'ctx> {
  pub(crate) fn build_name(&self, expr: *mut AstExpr) -> String {
    // `expr` 形参仍是裸指针：先经句柄门面 `OptNode::from_ptr` 借出基类引用，
    // 判型下转全部走句柄上生命周期正确的 `try_as`（null/不命中折叠为 None，
    // 即 cpp `->as<T>()` 返 null 同语义），借用半径由各栈帧局部句柄供给，
    // 不再锻造假 'static。
    let expr_node = OptNode::from_ptr(expr);
    if let Some(local) = expr_node.try_as::<AstExprLocal>() {
      let name = local.local.name;
      if !name.is_null() {
        return name.to_string();
      }
    } else if let Some(global) = expr_node.try_as::<AstExprGlobal>() {
      let name = global.name;
      if !name.is_null() {
        return name.to_string();
      }
    } else if let Some(index_name) = expr_node.try_as::<AstExprIndexName>() {
      // expr 已句柄化恒非空；build_name 行走链为既有裸指针 API，经 as_ptr 桥接。
      let lhs = self.build_name(index_name.expr.as_ptr());
      if lhs.is_empty() {
        return lhs;
      }
      return format!("{lhs}.{}", index_name.index);
    }
    String::new()
  }
}

// —— 原 methods/lint_duplicate_function_process.rs ——
impl<'ctx> LintDuplicateFunction<'ctx> {
  lint_stat_process!(LintDuplicateFunction {
    defns: DenseHashMap::default()
  });
}

// —— 原 methods/lint_duplicate_function_report.rs ——
impl<'ctx> LintDuplicateFunction<'ctx> {
  #[inline]
  pub fn report(&mut self, name: &str, location: Location, other_location: Location) {
    self.report_location_c_char_location(name, location, other_location);
  }
  pub fn report_location_c_char_location(
    &mut self,
    name: &str,
    location: Location,
    other_location: Location,
  ) {
    emit_warning(
      self.context.get(),
      Code::DuplicateFunction,
      location,
      format_args!(
        "Duplicate function definition: '{}' also defined on line {}",
        name,
        other_location.begin.line + 1
      ),
    );
  }
}

// —— 原 methods/lint_duplicate_function_track_function.rs ——
impl<'ctx> LintDuplicateFunction<'ctx> {
  pub fn track_function(&mut self, location: Location, name: &str) {
    if name.is_empty() {
      return;
    }
    let mut other_location = None;
    {
      // 查询走 &str 借用口（r7-rc-4）：同名函数重复登记（热路径）零分配，
      // 仅缺席插入时才物化键；语义与原 get_or_insert(String::from(name)) 逐位等价
      // （新键值即 Location::default() 全零，与原置零判定 *defn = location 同形）。
      if let Some(defn) = self.defns.get_mut_str(name) {
        if defn.end.line == 0 && defn.end.column == 0 {
          *defn = location;
        } else {
          other_location = Some(*defn);
        }
      } else {
        self.defns.insert(String::from(name), location);
      }
    }
    if let Some(defn) = other_location {
      self.report(name, location, defn);
    }
  }
}

// —— 原 methods/lint_duplicate_function_visit.rs ——
impl<'ctx> LintDuplicateFunction<'ctx> {
  pub(crate) fn visit_ast_stat_block(&mut self, block: *mut AstStatBlock) -> bool {
    self.defns.clear();
    let block_ref = alias_ref(block);
    let body = &block_ref.body;
    for stat in body.iter() {
      if let Some(func) = ast_node_try_as::<AstStatFunction>(stat) {
        // name 已句柄化为 Node<AstExpr>：`.get()` 即安全只读视图（原 unsafe 死
        // 守卫消失），`as_ptr` 桥交仍以指针形态消费的 build_name。
        let name_loc = func.name.get().base.location;
        self.track_function(name_loc, &self.build_name(func.name.as_ptr()));
        continue;
      }
      if let Some(local_func) = ast_node_try_as::<AstStatLocalFunction>(stat) {
        // name 已句柄化为 Node<AstLocal>：`.get()` 即安全只读视图，无裸指针解引用。
        let local_name = local_func.name.get();
        let name = local_name.name;
        if !name.is_null() {
          let name = name.as_str_or_empty();
          self.track_function(local_name.location, name);
        }
      }
    }
    true
  }
}
