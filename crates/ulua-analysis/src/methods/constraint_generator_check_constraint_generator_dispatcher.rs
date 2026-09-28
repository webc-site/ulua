//! 手移植分派器：cpp `ConstraintGenerator::check(const ScopePtr&, AstExpr*, ...)`
//! （`Analysis/src/ConstraintGenerator.cpp:3164`）。
//!
//! C++ 按表达式的静态类型重载 `check`；此处以 `AstExprRef` 模式匹配恢复该分派，
//! 转发到各 `check_expr_*` 逐节点实现。三个便捷入口镜像 cpp 的默认实参
//! （`expectedType = {}`、`forceSingleton = false`、`generalize = true`）。
use alloc::vec::Vec;

use ulua_ast::{enums::ast_expr_ref::AstExprRef, records::ast_expr::AstExpr};
use ulua_common::{LUAU_ASSERT, dfint};

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{
    arena_handle::{alias, alias_ref},
    constraint_generator::ConstraintGenerator,
    inference::Inference,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl ConstraintGenerator {
  /// cpp `check(scope, expr)`（默认实参便捷入口）。
  pub fn check_expr(&mut self, scope: &ScopePtr, expr: &AstExpr) -> Inference {
    self.check_expr_full(scope, expr, None, false, true)
  }

  /// cpp `check(scope, expr, expectedType)`。
  pub fn check_expr_expected(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExpr,
    expected_type: Option<TypeId>,
  ) -> Inference {
    self.check_expr_full(scope, expr, expected_type, false, true)
  }

  /// cpp `check(scope, expr, expectedType, forceSingleton)`。
  pub fn check_expr_singleton(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExpr,
    expected_type: Option<TypeId>,
    force_singleton: bool,
  ) -> Inference {
    self.check_expr_full(scope, expr, expected_type, force_singleton, true)
  }

  /// 五参全量实现（cpp `check(scope, expr, expectedType, forceSingleton,
  /// generalize)`）：前后各一次手工增减递归计数（cpp `RecursionCounter`），
  /// 实体在 [`Self::check_expr_dispatch`]。
  pub fn check_expr_full(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExpr,
    expected_type: Option<TypeId>,
    force_singleton: bool,
    generalize: bool,
  ) -> Inference {
    // RecursionCounter counter{&recursionCount};
    self.recursion_count += 1;
    let result = self.check_expr_dispatch(scope, expr, expected_type, force_singleton, generalize);
    self.recursion_count -= 1;
    result
  }

  fn check_expr_dispatch(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExpr,
    expected_type: Option<TypeId>,
    force_singleton: bool,
    generalize: bool,
  ) -> Inference {
    // 节点的裸指针值：cpp 全程以 `AstExpr*` 作缓存/astTypes 身份键并把同一
    // 指针透传给只读契约的被调方；此处从共享引用取同一地址，仅作身份键与
    // 透传使用，从不据此写入。
    let expr_ptr: *mut AstExpr = (expr as *const AstExpr).cast_mut();

    if self.recursion_count >= dfint::LuauConstraintGeneratorRecursionLimit.get() {
      self.report_code_too_complex(expr.base.location);
      return Inference::no_refinement(self.builtin_types.get().error_type);
    }

    // We may recurse a given expression more than once when checking
    // compound assignment, so we store and cache expressions here.
    if self.inferred_expr_cache.contains(&expr_ptr) {
      return self.inferred_expr_cache.get_or_insert(expr_ptr).clone();
    }

    let result: Inference = match expr.as_expr_ref() {
      AstExprRef::Group(group) => {
        // expr 已句柄化：get() 只读借用出自 group 存活引用，递归转调与 cpp 直传同构。
        self.check_expr_full(
          scope,
          group.expr.get(),
          expected_type,
          force_singleton,
          generalize,
        )
      }
      AstExprRef::ConstantString(str_expr) => {
        self.check_expr_constant_string(scope, str_expr, expected_type, force_singleton)
      }
      AstExprRef::ConstantNumber(_) => {
        Inference::no_refinement(self.builtin_types.get().number_type)
      }
      AstExprRef::ConstantInteger(_) => {
        Inference::no_refinement(self.builtin_types.get().integer_type)
      }
      AstExprRef::ConstantBool(bool_expr) => {
        self.check_expr_constant_bool(scope, bool_expr, expected_type, force_singleton)
      }
      AstExprRef::ConstantNil(_) => Inference::no_refinement(self.builtin_types.get().nil_type),
      AstExprRef::Local(local) => self.check_expr_local(scope, local),
      AstExprRef::Global(global) => self.check_expr_global(scope, global),
      AstExprRef::Varargs(_) => {
        // expr 即本函数受检节点自身（契约：分析期存活、只读），以共享引用
        // 直传满足 check_pack 对 cpp `checkPack(scope, expr)` 形参的存活契约。
        let pack = self.check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
          scope,
          expr,
          &Vec::new(),
          true,
        );
        self.flatten_pack(scope, expr.base.location, pack)
      }
      AstExprRef::Call(call) => {
        let pack = self.check_pack_scope_ptr_ast_expr_call(scope, call);
        self.flatten_pack(scope, expr.base.location, pack)
      }
      AstExprRef::Function(function) => {
        self.check_expr_function(scope, function, expected_type, generalize)
      }
      AstExprRef::IndexName(index_name) => self.check_expr_index_name(scope, index_name),
      AstExprRef::IndexExpr(index_expr) => self.check_expr_index_expr(scope, index_expr),
      AstExprRef::Table(table) => self.check_expr_table(scope, table, expected_type),
      AstExprRef::Unary(unary) => self.check_expr_unary(scope, unary),
      AstExprRef::Binary(binary) => {
        // C++: check(scope, binary, expectedType) returns the full
        // Inference (type + refinement). `check_expr_binary` 的旧包装
        // 会丢弃 refinement（只回 `.ty`），故直连 `check_ast_expr_binary` 保持忠实。
        // left/right 已句柄化，alias_ref 收口为共享引用直传。
        self.check_ast_expr_binary(
          scope,
          binary.base.base.location,
          binary.op,
          alias_ref(binary.left.as_ptr()),
          alias_ref(binary.right.as_ptr()),
          expected_type,
        )
      }
      AstExprRef::IfElse(if_else) => self.check_expr_if_else(scope, if_else, expected_type),
      AstExprRef::TypeAssertion(type_assertion) => {
        self.check_expr_type_assertion(scope, type_assertion)
      }
      AstExprRef::InterpString(interp_string) => {
        self.check_expr_interp_string(scope, interp_string)
      }
      AstExprRef::Instantiate(instantiate) => self.check_expr_instantiate(scope, instantiate),
      AstExprRef::Error(error) => {
        // Open question: Should we traverse into this?
        for sub_expr in error.expressions.iter_nodes() {
          self.check_expr(scope, sub_expr);
        }
        Inference::no_refinement(self.builtin_types.get().error_type)
      }
    };

    *self.inferred_expr_cache.get_or_insert(expr_ptr) = result.clone();

    LUAU_ASSERT!(!result.ty.is_null());

    if let Some(module) = &self.module {
      let module_ptr = arc_as_mut(module);
      // module_ptr 是本 crate 约定的 Arc<Module> 写句柄（alias 门面即时物化，
      // 单线程、独占 `&mut self` 窗口内写），此处仅按
      // cpp `module->astTypes[expr] = result.ty` 写两张身份键表。
      let module_ref = alias(module_ptr);
      *module_ref
        .ast_types
        .get_or_insert(expr_ptr as *const AstExpr) = result.ty;
      if let Some(et) = expected_type {
        *module_ref
          .ast_expected_types
          .get_or_insert(expr_ptr as *const AstExpr) = et;
      }
    }

    result
  }
}
