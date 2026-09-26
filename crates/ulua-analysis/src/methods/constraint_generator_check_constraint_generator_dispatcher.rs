//! 手移植分派器：cpp `ConstraintGenerator::check(const ScopePtr&, AstExpr*, ...)`
//! （`Analysis/src/ConstraintGenerator.cpp:3164`）。
//!
//! C++ 按表达式的静态类型重载 `check`；此处以 `class_index` match 恢复该分派，
//! 转发到各 `check_expr_*` 逐节点实现。三个便捷入口镜像 cpp 的默认实参
//! （`expectedType = {}`、`forceSingleton = false`、`generalize = true`）。
use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup, ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate, ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal, ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion, ast_expr_unary::AstExprUnary,
    ast_expr_varargs::AstExprVarargs, ast_node::AstNode,
  },
  rtti::AstNodeClass,
};
use ulua_common::{LUAU_ASSERT, dfint};

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{constraint_generator::ConstraintGenerator, inference::Inference},
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

    let node: &AstNode = &expr.base;

    // match 臂的类索引与 `ast_node_try_as` 的判定完全同一（cpp `as<T>()` 命中后
    // 直接 static_cast 亦假定成功），故下转必然成功，expect 为逻辑不可达分支；
    // 家族类索引互斥由 rtti 的 rtti_indices_unique 测试保证，臂序无关语义。
    let result: Inference = match node.class_index {
      AstExprGroup::CLASS_INDEX => {
        let group = expr_downcast::<AstExprGroup>(node);
        // expr 已句柄化：get() 只读借用出自 group 存活引用，递归转调与 cpp 直传同构。
        self.check_expr_full(
          scope,
          group.expr.get(),
          expected_type,
          force_singleton,
          generalize,
        )
      }
      AstExprConstantString::CLASS_INDEX => {
        self.check_expr_constant_string(scope, expr_downcast(node), expected_type, force_singleton)
      }
      AstExprConstantNumber::CLASS_INDEX => {
        Inference::no_refinement(self.builtin_types.get().number_type)
      }
      AstExprConstantInteger::CLASS_INDEX => {
        Inference::no_refinement(self.builtin_types.get().integer_type)
      }
      AstExprConstantBool::CLASS_INDEX => {
        self.check_expr_constant_bool(scope, expr_downcast(node), expected_type, force_singleton)
      }
      AstExprConstantNil::CLASS_INDEX => {
        Inference::no_refinement(self.builtin_types.get().nil_type)
      }
      AstExprLocal::CLASS_INDEX => self.check_expr_local(scope, expr_downcast(node)),
      AstExprGlobal::CLASS_INDEX => self.check_expr_global(scope, expr_downcast(node)),
      AstExprVarargs::CLASS_INDEX => {
        // SAFETY: expr_ptr 即本函数受检节点自身（契约：分析期存活、只读），
        // 满足 check_pack 对 cpp `checkPack(scope, expr)` 形参的存活契约。
        let pack = unsafe {
          self.check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
            scope,
            expr_ptr,
            &Vec::new(),
            true,
          )
        };
        self.flatten_pack(scope, expr.base.location, pack)
      }
      AstExprCall::CLASS_INDEX => {
        let call = expr_downcast::<AstExprCall>(node);
        // SAFETY: call 派生自受检节点 `expr` 本身（match 臂类索引命中），
        // 地址即 cpp `expr->as<AstExprCall>()` 的 static_cast 结果，被调按
        // 存活只读契约消费。
        let call_ptr: *mut AstExprCall = (call as *const AstExprCall).cast_mut();
        let pack = unsafe { self.check_pack_scope_ptr_ast_expr_call(scope, call_ptr) };
        self.flatten_pack(scope, expr.base.location, pack)
      }
      AstExprFunction::CLASS_INDEX => {
        self.check_expr_function(scope, expr_downcast(node), expected_type, generalize)
      }
      AstExprIndexName::CLASS_INDEX => self.check_expr_index_name(scope, expr_downcast(node)),
      AstExprIndexExpr::CLASS_INDEX => self.check_expr_index_expr(scope, expr_downcast(node)),
      AstExprTable::CLASS_INDEX => self.check_expr_table(scope, expr_downcast(node), expected_type),
      AstExprUnary::CLASS_INDEX => self.check_expr_unary(scope, expr_downcast(node)),
      AstExprBinary::CLASS_INDEX => {
        // C++: check(scope, binary, expectedType) returns the full
        // Inference (type + refinement). `check_expr_binary` 的旧包装
        // 会丢弃 refinement（只回 `.ty`），故直连 `check_ast_expr_binary` 保持忠实。
        let binary = expr_downcast::<AstExprBinary>(node);
        // left/right 已句柄化；check_ast_expr_binary 为既有裸指针 API，经 as_ptr 桥接。
        self.check_ast_expr_binary(
          scope,
          binary.base.base.location,
          binary.op,
          binary.left.as_ptr(),
          binary.right.as_ptr(),
          expected_type,
        )
      }
      AstExprIfElse::CLASS_INDEX => {
        self.check_expr_if_else(scope, expr_downcast(node), expected_type)
      }
      AstExprTypeAssertion::CLASS_INDEX => {
        self.check_expr_type_assertion(scope, expr_downcast(node))
      }
      AstExprInterpString::CLASS_INDEX => self.check_expr_interp_string(scope, expr_downcast(node)),
      AstExprInstantiate::CLASS_INDEX => self.check_expr_instantiate(scope, expr_downcast(node)),
      AstExprError::CLASS_INDEX => {
        // Open question: Should we traverse into this?
        let error = expr_downcast::<AstExprError>(node);
        for sub_expr in error.expressions.iter_nodes() {
          self.check_expr(scope, sub_expr);
        }
        Inference::no_refinement(self.builtin_types.get().error_type)
      }
      _ => {
        LUAU_ASSERT!(false);
        Inference::no_refinement(self.fresh_type(scope, self.polarity))
      }
    };

    *self.inferred_expr_cache.get_or_insert(expr_ptr) = result.clone();

    LUAU_ASSERT!(!result.ty.is_null());

    if let Some(module) = &self.module {
      let module_ptr = arc_as_mut(module);
      // SAFETY: module_ptr 是本 crate 约定的 Arc<Module> 共享写句柄
      // （arc_as_mut 契约：单线程、独占 `&mut self` 窗口内写），此处仅按
      // cpp `module->astTypes[expr] = result.ty` 写两张身份键表。
      unsafe {
        *(*module_ptr)
          .ast_types
          .get_or_insert(expr_ptr as *const AstExpr) = result.ty;
        if let Some(et) = expected_type {
          *(*module_ptr)
            .ast_expected_types
            .get_or_insert(expr_ptr as *const AstExpr) = et;
        }
      }
    }

    result
  }
}

pub(crate) use crate::functions::ast_node_downcast::ast_node_downcast as expr_downcast;
