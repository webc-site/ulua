use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall,
    ast_expr_varargs::AstExprVarargs,
  },
  rtti::ast_node_is_ptr,
};
use ulua_common::{LUAU_ASSERT, dfint};

use crate::{
  enums::type_context::TypeContext,
  functions::{arc_as_mut::arc_as_mut, checkpoint::checkpoint},
  records::{
    constraint_generator::ConstraintGenerator, in_conditional_context::InConditionalContext,
    inference_pack::InferencePack, recursion_counter::RecursionCounter,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId},
};

impl ConstraintGenerator {
  pub fn check_pack_scope_ptr_ast_array_ast_expr_vector_optional_type_id(
    &mut self,
    scope: &ScopePtr,
    exprs: AstArray<*mut AstExpr>,
    expected_types: &[Option<TypeId>],
  ) -> InferencePack {
    let mut head: Vec<TypeId> = Vec::new();
    let mut tail: Option<TypePackId> = None;

    for (i, &expr) in exprs.as_slice().iter().enumerate() {
      if i < exprs.size - 1 {
        let expected_type = expected_types.get(i).copied().flatten();
        // SAFETY: expr 是 `exprs.as_slice()` 元素，parse arena 持有的存活
        // AstExpr，只取共享引用递归（cpp 直接透传同一指针）。
        head.push(
          self
            .check_expr_expected(scope, unsafe { &*expr }, expected_type)
            .ty,
        );
      } else {
        let expected_tail_types: Vec<Option<TypeId>> =
          expected_types.get(i..).unwrap_or_default().to_vec();
        tail = Some(
          // Safety: `expr` 是 `exprs.as_slice()` 尾元素，即 parse arena 持有的存活
          // AstExpr；`scope` 由本函数契约透传（C++:2713 `checkPack(scope, expr,
          // expectedTailTypes, generalize)`）。
          unsafe {
            self.check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
              scope,
              expr,
              &expected_tail_types,
              true,
            )
          }
          .tp,
        );
      }
    }

    InferencePack {
      tp: self.add_type_pack(head, tail),
      refinements: Vec::new(),
    }
  }

  /// # Safety
  /// 对应 C++ `checkPack(const ScopePtr& scope, AstExpr* expr,
  /// const std::vector<std::optional<TypeId>>&, bool)`（ConstraintGenerator.cpp:2747）
  /// 的裸指针形参前提，逐参数：
  /// * `scope`：调用期存活的 `Scope`（C++ ScopePtr 引用），其 `vararg_pack` 等
  ///   字段在解引用 `&*scope`（此处经 Arc）期间保持有效；
  /// * `expr`：本模块 parse arena 持有的存活 `AstExpr` 节点，`as<AstExprCall>`
  ///   下转型与 `(*expr).base.location` 读取均要求该节点及其首字段 AstNode 有效；
  ///
  /// 调用方还须保证 `self` 的构造期不变量（arena/builtin_types/module 非空且
  /// 与会话同寿），与 C++ 成员 NotNull 对应。
  pub unsafe fn check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
    &mut self,
    scope: &ScopePtr,
    expr: *mut AstExpr,
    expected_types: &[Option<TypeId>],
    generalize: bool,
  ) -> InferencePack {
    // Safety: `&mut self.recursion_count` 派生的 *mut i32 指向本 ConstraintGenerator
    // 存活字段；RAII 计数器的 Drop 在函数体内、`&mut self` 借用结束前归还计数
    // （C++:2753 `RecursionCounter counter{&recursionCount}`）。
    let _counter = RecursionCounter::recursion_counter_i32(&mut self.recursion_count);

    if self.recursion_count >= dfint::LuauConstraintGeneratorRecursionLimit.get() {
      // Safety: `expr` 按本函数契约为存活 AstExpr 节点，仅读其 AstNode 首字段的
      // location（C++:2756 `reportCodeTooComplex(expr->location)`）。
      self.report_code_too_complex(unsafe { (*expr).base.location });
      return InferencePack {
        // Safety: 构造期注入的 builtin_types 非空单例，error_type_pack 为不可变
        // 常量槽（C++:2757 `builtinTypes->errorTypePack`）。
        tp: { self.builtin_types.get().error_type_pack },
        refinements: Vec::new(),
      };
    }

    let result: InferencePack;

    // Safety: AstExpr 各生成类型均 repr(C) 且 AstNode 为首字段，`expr` 按契约
    // 存活，判型命中后 `expr.cast::<AstExprCall>()` 与 cpp static_cast 同为基址
    // 重合下转（C++:2765 `expr->as<AstExprCall>()`）。
    if unsafe { ast_node_is_ptr::<AstExprCall>(expr) } {
      // Safety: 上一守卫判定 AstExprCall 命中，即存活节点；scope 原样透传
      // （C++:2771 `result = checkPack(scope, call)`）。
      result =
        unsafe { self.check_pack_scope_ptr_ast_expr_call(scope, expr.cast::<AstExprCall>()) };
    } else if unsafe { ast_node_is_ptr::<AstExprVarargs>(expr) } {
      if let Some(vararg_pack) = scope.as_ref().vararg_pack {
        result = InferencePack {
          tp: vararg_pack,
          refinements: Vec::new(),
        };
      } else {
        result = InferencePack {
          // Safety: 与递归超限分支同一 builtin_types 单例常槽读取，本处对应
          // C++:2779 varargPack 缺失时的 errorTypePack。
          tp: { self.builtin_types.get().error_type_pack },
          refinements: Vec::new(),
        };
      }
    } else {
      let mut expected_type: Option<TypeId> = None;
      if !expected_types.is_empty() {
        expected_type = expected_types[0];
      }
      // SAFETY: expr 按本函数契约为存活 AstExpr 节点，只取共享引用
      // （cpp 同款透传）。
      let t: TypeId = self
        .check_expr_full(scope, unsafe { &*expr }, expected_type, false, generalize)
        .ty;
      result = InferencePack {
        // Safety: arena 独占追加窗口；t 为 check 刚产出的 arena 驻留 TypeId，
        // 单元素 Vec 拷贝无别名（C++:2797 `arena->addTypePack({t})`）。
        tp: {
          self
            .arena
            .get_mut()
            .add_type_pack_initializer_list_type_id(&[t])
        },
        refinements: Vec::new(),
      };
    }

    LUAU_ASSERT!(!result.tp.is_null());
    if let Some(module) = &self.module {
      let module_ptr = arc_as_mut(module);
      // Safety: module Arc 目标由 self.module 持有、与会话同寿，ast_type_packs
      // 键 `expr` 为存活 AST 指针、值 result.tp 为 arena 驻留 TypePackId
      // （C++:2800-2801 `module->astTypePacks[expr] = result.tp`）。
      unsafe {
        *(*module_ptr)
          .ast_type_packs
          .get_or_insert(expr as *const AstExpr) = result.tp;
      }
    }
    result
  }

  /// # Safety
  /// 对应 C++ `checkPack(const ScopePtr& scope, AstExprCall* call,
  /// std::optional<TypeId>)`（ConstraintGenerator.cpp:2804）的裸指针形参前提：
  /// * `scope`：调用期存活 `Scope`（C++ ScopePtr 引用）；
  /// * `call`：非空且指向本模块 parse arena 持有的存活 `AstExprCall`，其
  ///   `func`/`args` 子节点同源存活；调用期无人以他途可变借用该 AST 节点。
  pub unsafe fn check_pack_scope_ptr_ast_expr_call(
    &mut self,
    scope: &ScopePtr,
    call: *mut AstExprCall,
  ) -> InferencePack {
    let func_begin = checkpoint(self);
    let fn_type = {
      // `&mut self.type_context` 指向本对象存活字段；InConditionalContext
      // 保存旧值并在 Drop（本块末，先于 `&mut self` 借用结束）还原，独占窗口与
      // C++ RAII 局部 icc2 一致（cpp:2809）。构造器已 safe 化,无需 unsafe 块。
      let _in_context = InConditionalContext::new(&mut self.type_context, TypeContext::Default);
      // Safety: `call` 按本函数契约为存活 AstExprCall，只读 func 指针字段并透传
      // 存活 scope（C++:2811 `check(scope, call->func)`）；子指针仅取共享引用。
      self.check_expr(scope, unsafe { &*(*call).func }).ty
    };
    let func_end = checkpoint(self);
    // Safety: scope/call 满足被调 `check_expr_call` 对存活 Scope/AstExprCall 裸
    // 指针的同类契约，fn_type 刚由 check 产出为 arena 驻留 TypeId
    // （C++:2816 `checkExprCall(scope, call, fnType, ...)`）。
    // SAFETY: call 按本函数契约为存活 AstExprCall，取共享引用交安全被调。
    self.check_expr_call(scope, unsafe { &*call }, fn_type, func_begin, func_end)
  }
}
