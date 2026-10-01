use alloc::vec::Vec;
use core::ptr::from_ref;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall},
};
use ulua_common::{LUAU_ASSERT, dfint};

use crate::{
  enums::type_context::TypeContext,
  functions::{checkpoint::checkpoint, shared_mut::shared_mut},
  records::{
    arena_handle::{alias, alias_ref},
    constraint_generator::ConstraintGenerator,
    in_conditional_context::InConditionalContext,
    inference_pack::InferencePack,
    recursion_counter::RecursionCounter,
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
        // expr 是 `exprs.as_slice()` 元素，parse arena 持有的存活
        // AstExpr，alias_ref 收口后只取共享引用递归（cpp 直接透传同一指针）。
        head.push(
          self
            .check_expr_expected(scope, alias_ref(expr), expected_type)
            .ty,
        );
      } else {
        let expected_tail_types: Vec<Option<TypeId>> =
          expected_types.get(i..).unwrap_or_default().to_vec();
        tail = Some(
          // `expr` 是 `exprs.as_slice()` 尾元素，即 parse arena 持有的存活
          // AstExpr；`scope` 原样透传（C++:2713 `checkPack(scope, expr,
          // expectedTailTypes, generalize)`）。
          self
            .check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
              scope,
              alias_ref(expr),
              &expected_tail_types,
              true,
            )
            .tp,
        );
      }
    }

    InferencePack {
      tp: self.add_type_pack(head, tail),
      refinements: Vec::new(),
    }
  }

  /// C++ `checkPack(const ScopePtr& scope, AstExpr* expr,
  /// const std::vector<std::optional<TypeId>>&, bool)`（ConstraintGenerator.cpp:2747）。
  /// 形参链已引用化（原 `# Safety` 契约由签名承担）：`expr` 为 parse arena 持有的
  /// 存活 `AstExpr` 共享借用，判型下转与 location 读取均只读；`self` 的
  /// arena/builtin_types/module 构造期非空不变量对应 C++ 成员 NotNull。
  pub fn check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExpr,
    expected_types: &[Option<TypeId>],
    generalize: bool,
  ) -> InferencePack {
    // RecursionCounter 的 RAII Drop 在函数体内、`&mut self` 借用结束前归还计数
    // （C++:2753 `RecursionCounter counter{&recursionCount}`）。
    let _counter = RecursionCounter::recursion_counter_i32(&mut self.recursion_count);

    if self.recursion_count >= dfint::LuauConstraintGeneratorRecursionLimit.get() {
      self.report_code_too_complex(expr.base.location);
      return InferencePack {
        // 构造期注入的 builtin_types 非空单例，error_type_pack 为不可变
        // 常量槽（C++:2757 `builtinTypes->errorTypePack`）。
        tp: { self.builtin_types.get().error_type_pack },
        refinements: Vec::new(),
      };
    }

    let result: InferencePack;

    // AstExpr 各生成类型均 repr(C) 且 AstNode 为首字段；as_expr_ref() 安全判别
    // 具体表达式类型（C++:2765 `expr->as<AstExprCall>()`）。
    match expr.as_expr_ref() {
      AstExprRef::Call(call) => {
        // C++:2771 `result = checkPack(scope, call)`
        result = self.check_pack_scope_ptr_ast_expr_call(scope, call);
      }
      AstExprRef::Varargs(_) => {
        if let Some(vararg_pack) = scope.as_ref().vararg_pack {
          result = InferencePack {
            tp: vararg_pack,
            refinements: Vec::new(),
          };
        } else {
          result = InferencePack {
            // 与递归超限分支同一 builtin_types 单例常槽读取，本处对应
            // C++:2779 varargPack 缺失时的 errorTypePack。
            tp: { self.builtin_types.get().error_type_pack },
            refinements: Vec::new(),
          };
        }
      }
      _ => {
        let mut expected_type: Option<TypeId> = None;
        if !expected_types.is_empty() {
          expected_type = expected_types[0];
        }
        // expr 为本函数参数里的存活共享借用（cpp 同款透传）。
        let t: TypeId = self
          .check_expr_full(scope, expr, expected_type, false, generalize)
          .ty;
        result = InferencePack {
          // arena 独占追加窗口；t 为 check 刚产出的 arena 驻留 TypeId，
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
    }

    LUAU_ASSERT!(!result.tp.is_null());
    if let Some(module) = &self.module {
      let module_ptr = shared_mut(module);
      // module Arc 目标由 self.module 持有、与会话同寿，ast_type_packs
      // 键 `expr` 为存活 AST 指针、值 result.tp 为 arena 驻留 TypePackId
      // （C++:2800-2801 `module->astTypePacks[expr] = result.tp`）；写经 alias 收口。
      *alias(module_ptr)
        .ast_type_packs
        .get_or_insert(from_ref(expr)) = result.tp;
    }
    result
  }

  /// 对应 C++ `checkPack(const ScopePtr& scope, AstExprCall* call,
  /// std::optional<TypeId>)`（ConstraintGenerator.cpp:2804）。
  pub fn check_pack_scope_ptr_ast_expr_call(
    &mut self,
    scope: &ScopePtr,
    call: &AstExprCall,
  ) -> InferencePack {
    let func_begin = checkpoint(self);
    let fn_type = {
      // `&mut self.type_context` 指向本对象存活字段；InConditionalContext
      // 保存旧值并在 Drop（本块末，先于 `&mut self` 借用结束）还原，独占窗口与
      // C++ RAII 局部 icc2 一致（cpp:2809）。
      let _in_context = InConditionalContext::new(&mut self.type_context, TypeContext::Default);
      // `call.func` 为存活 AstExpr 指针，alias_ref 收口后取共享引用透传
      // （C++:2811 `check(scope, call->func)`）。
      self.check_expr(scope, alias_ref(call.func)).ty
    };
    let func_end = checkpoint(self);
    self.check_expr_call(scope, call, fn_type, func_begin, func_end)
  }
}
