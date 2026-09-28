use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_index_name::AstExprIndexName, location::Location,
  },
  rtti::ast_node_try_as_ptr,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{arc_as_mut::arc_as_mut, get_type},
  records::{
    function_does_not_take_self::FunctionDoesNotTakeSelf,
    function_requires_self::FunctionRequiresSelf, function_type::FunctionType, module::Module,
    overload_error_entry::OverloadErrorEntry, type_checker::TypeChecker, type_error::TypeError,
    type_pack::TypePack,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_error_data::TypeErrorData},
};

impl TypeChecker {
  // cpp TypeInfer.cpp:4849
  pub fn handle_self_call_mismatch(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprCall,
    args: &mut TypePack,
    arg_locations: &[Location],
    errors: &Vec<OverloadErrorEntry>,
  ) -> bool {
    // 试算用的 edited unifier 会向 current_module.errors 追加候选错误，
    // 判定后须回滚到 checkpoint，避免污染真实报错通道
    let module_ptr = self
      .current_module
      .as_ref()
      .map_or(null_mut::<Module>(), arc_as_mut);
    let errors_len = || {
      if module_ptr.is_null() {
        0
      } else {
        // Safety: 分支已判非空；`module_ptr` 由 `arc_as_mut` 从 self 持有的
        // `Arc<Module>` 导出，Arc 在本方法借用期内存活，只读 length 与 C++
        // `module->errors.size()` 同构。
        unsafe { (*module_ptr).errors.len() }
      }
    };
    let rollback = |checkpoint: usize| {
      if !module_ptr.is_null() {
        // Safety: 非空论证同 errors_len 闭包；truncate 是直译 C++ 试算回滚
        // （edited unifier 候选错误写穿 current_module->errors），分析单线程，
        // 本闭包调用点之间无人以其他借用访问 Module.errors。
        unsafe { (*module_ptr).errors.truncate(checkpoint) };
      }
    };

    // No overloads succeeded: scan for one that would have worked had the
    // user used `a.b()` rather than `a:b()` or vice versa.
    for e in errors {
      let Some(ftv) = get_type::get::<FunctionType>(e.fn_ty) else {
        LUAU_ASSERT!(!e.fn_ty.is_null());
        continue;
      };

      if expr.self_ {
        let edited_arg_locations = if arg_locations.len() > 1 {
          arg_locations[1..].to_vec()
        } else {
          Vec::new()
        };

        let edited_param_list = if args.head.len() > 1 {
          args.head[1..].to_vec()
        } else {
          Vec::new()
        };
        let edited_arg_pack =
          self.add_type_pack_type_pack(TypePack::new(edited_param_list, args.tail));

        let mut edited_state = self.mk_unifier(scope, &expr.base.base.location);
        let error_checkpoint = errors_len();

        self.check_argument_list(
          scope,
          // Safety: `expr.func` 是 parser 绑定进 AST arena 的被调表达式指针
          // （AstExprCall.func 恒非空），随本次类型检查全程存活；被调方仅
          // 只读遍历 AST，直译 C++ 的 `expr->func` 传参。
          unsafe { &*expr.func },
          &mut edited_state,
          edited_arg_pack,
          ftv.arg_types,
          &edited_arg_locations,
        );
        rollback(error_checkpoint);

        if edited_state.errors.is_empty() {
          edited_state.log.commit();
          self.report_error_type_error(&TypeError::type_error_location_type_error_data(
            expr.base.base.location,
            FunctionDoesNotTakeSelf::default().into(),
          ));
          return true;
        }
      } else if ftv.has_self
        // Safety: `expr.func` 为 arena 存活的非空基节点指针，满足
        // `ast_node_try_as_ptr` 契约；null/类型不符折叠为 None，命中才解引用，
        // 返回借用只读且随 AST 存活（本方法内无人写穿节点）。
        && let Some(index_name) = (unsafe { ast_node_try_as_ptr::<AstExprIndexName>(expr.func) })
      {
        let mut edited_arg_locations = Vec::with_capacity(arg_locations.len() + 1);
        // AstExprIndexName.expr 已句柄化恒非空（接收者表达式必存在，cpp 原版
        // 同样直接解引用），仅读 location。
        edited_arg_locations.push(index_name.expr.get().base.location);
        edited_arg_locations.extend(arg_locations.iter().copied());

        let mut edited_arg_list = args.head.clone();
        let error_checkpoint = errors_len();

        let receiver_type = self
          .check_expr(
            scope,
            // 同一 `index_name.expr` 已句柄化恒非空：.get() 只读遍历
            // （候选错误回滚针对 Module.errors，与 AST 无关）。
            index_name.expr.get(),
            None,
            false,
          )
          .r#type;
        rollback(error_checkpoint);

        edited_arg_list.insert(0, receiver_type);
        let edited_arg_pack =
          self.add_type_pack_type_pack(TypePack::new(edited_arg_list, args.tail));

        let mut edited_state = self.mk_unifier(scope, &expr.base.base.location);
        let error_checkpoint = errors_len();

        self.check_argument_list(
          scope,
          // Safety: 与 `expr.self_` 分支同一传参——expr.func 为非空、随
          // AST arena 存活的被调表达式，被调方只读。
          unsafe { &*expr.func },
          &mut edited_state,
          edited_arg_pack,
          ftv.arg_types,
          &edited_arg_locations,
        );
        rollback(error_checkpoint);

        let only_receiver_mismatch = edited_state.errors.len() == 1
          && matches!(edited_state.errors[0].data, TypeErrorData::TypeMismatch(_))
          // index_name.expr 已句柄化恒非空（本分支开头同一论证），仅读
          // location 与已记录的错误位置做值比较。
          && edited_state.errors[0].location == index_name.expr.get().base.location;

        if edited_state.errors.is_empty() || (only_receiver_mismatch && !args.head.is_empty()) {
          edited_state.log.commit();
          self.report_error_type_error(&TypeError::type_error_location_type_error_data(
            expr.base.base.location,
            FunctionRequiresSelf::default().into(),
          ));
          return true;
        }
      }
    }

    false
  }
}
