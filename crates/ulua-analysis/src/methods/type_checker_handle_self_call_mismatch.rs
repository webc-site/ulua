use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_index_name::AstExprIndexName, location::Location,
    node_handle::OptNode,
  },
  rtti::ast_node_try_as,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{get_type, shared_mut::shared_mut},
  records::{
    arena_handle::alias_ref, function_does_not_take_self::FunctionDoesNotTakeSelf,
    function_requires_self::FunctionRequiresSelf, function_type::FunctionType,
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
    // 判定后须回滚到 checkpoint，避免污染真实报错通道。
    // 持一份 Arc clone 而非裸指针：只读侧直接走 `Arc` 的 `Deref`（无 `&mut`，
    // 天然合法），写侧在闭包内按需物化 `shared_mut` 句柄、止于该语句。
    let module = self.current_module.clone();
    let errors_len = || module.as_ref().map_or(0, |m| m.errors.len());
    let rollback = |checkpoint: usize| {
      if let Some(m) = &module {
        shared_mut(m).errors.truncate(checkpoint);
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
          alias_ref(expr.func),
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
            FunctionDoesNotTakeSelf.into(),
          ));
          return true;
        }
      } else if ftv.has_self {
        // `expr.func` 仍是裸指针槽：经句柄门面 `OptNode::from_ptr` 把可空性
        // 折叠为 `Option`，判型下转走生命周期正确的 [`ast_node_try_as`]（null/
        // 类型不符折叠为 None 命中才解引用），借用半径由本分支局部句柄供给，
        // 不锻造假 'static；未命中即如旧跳过本分支（循环体末尾，continue 等价）。
        let func_node = OptNode::from_ptr(expr.func);
        let Some(index_name) = func_node
          .get()
          .and_then(|f| ast_node_try_as::<AstExprIndexName>(f))
        else {
          continue;
        };
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
          alias_ref(expr.func),
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
            FunctionRequiresSelf.into(),
          ));
          return true;
        }
      }
    }

    false
  }
}
