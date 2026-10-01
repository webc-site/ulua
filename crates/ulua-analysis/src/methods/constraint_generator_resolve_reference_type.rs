use alloc::{format, vec::Vec};
use core::ptr::from_ref;

use ulua_ast::records::{
  ast_type::AstType, ast_type_or_pack::AstTypeOrPack, ast_type_reference::AstTypeReference,
};
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::polarity::Polarity,
  functions::{
    finite::finite,
    first::first,
    follow_type,
    get_mutable_type::get_mutable,
    get_type,
    magic_names::{LUAU_BLOCKED_TYPE, LUAU_ICE, LUAU_PRINT},
    shared_mut::shared_mut,
    size_type_pack::size,
  },
  records::{
    arena_handle::alias, blocked_type::BlockedType, constraint_generator::ConstraintGenerator,
    generic_error::GenericError, generic_type::GenericType,
    pending_expansion_type::PendingExpansionType, reduce_constraint::ReduceConstraint,
    type_alias_expansion_constraint::TypeAliasExpansionConstraint, type_fun::TypeFun,
    type_function_instance_type::TypeFunctionInstanceType,
    unapplied_type_function::UnappliedTypeFunction,
  },
  type_aliases::{
    constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData,
    type_id::TypeId, type_pack_id::TypePackId,
  },
};
impl ConstraintGenerator {
  /// C++ `resolveReferenceType(const ScopePtr& scope, AstType* ty, AstTypeReference* ref, bool, bool)`
  /// （ConstraintGenerator.cpp:4566）。形参链已引用化：`scope` 为调用方 `ScopePtr`
  /// 共享借用；`ty` 与 `ref_` 由唯一调用方 [`ConstraintGenerator::resolve_type_inner`]
  /// 的模式匹配确认指向同一处 parse arena 存活节点，全程只读。
  pub fn resolve_reference_type(
    &mut self,
    scope: &ScopePtr,
    ty: &AstType,
    ref_: &AstTypeReference,
    in_type_arguments: bool,
    replace_error_with_fresh: bool,
  ) -> TypeId {
    let mut result: TypeId;

    if fflag::DebugLuauMagicTypes.get() {
      let ref_name_str = ref_.name.as_str_or_empty();

      if ref_name_str == LUAU_ICE {
        self
          .ice
          .get()
          .ice_string_location(&format!("{LUAU_ICE} encountered"), &ty.base.location);
      } else if ref_name_str == LUAU_PRINT {
        let location = ty.base.location;
        // 对应 cpp `params.size != 1 || !params.data[0].type`：`as_slice().first()`
        // 仅在 `size == 1` 时读取首元素（data/size 成对由 arena 保证），`as_type`
        // 把「非 `Type` 形态（含错误态）」折叠为 `None`，与 cpp 读 null 同值。
        let print_arg: Option<&AstType> = if ref_.parameters.size == 1 {
          ref_
            .parameters
            .as_slice()
            .first()
            .and_then(AstTypeOrPack::as_type)
        } else {
          None
        };
        if let Some(arg_ty) = print_arg {
          return self.resolve_type_inner(scope, arg_ty, in_type_arguments, false);
        } else {
          let err = GenericError::new(format!("{LUAU_PRINT} requires one generic parameter"));
          self.report_error(location, TypeErrorData::GenericError(err));
          // `self.module` 由构造断言为 `Some`、Arc 被 generator 持有至会话末，
          // 经 alias 收口写 `module->astResolvedTypes`（单线程独占写）。
          let module_ptr = shared_mut(
            self
              .module
              .as_ref()
              .expect("构造断言 module 为 Some 且 generator 持有 Arc 至会话末"),
          );
          *alias(module_ptr)
            .ast_resolved_types
            .get_or_insert(from_ref(ty)) = self.builtin_types.get().error_type;
          return self.builtin_types.get().error_type;
        }
      } else if ref_name_str == LUAU_BLOCKED_TYPE {
        // 构造期接线的 arena 句柄，`add_type` 是单线程生成器独占的 arena 追加写。
        return self.arena.get_mut().add_type(BlockedType::default());
      }
    }

    let name_str = ref_.name.as_str_or_empty().to_string();
    // 读 `prefix`（Option<AstName>，按位拷出），不产生越过本行的借用。
    // （局部名取 `found`，避免遮蔽 arena_handle 的 `alias` 收口门面。）
    let found: Option<TypeFun> = if let Some(prefix_name) = ref_.prefix {
      let prefix_str = prefix_name.as_str_or_empty().to_string();
      scope.lookup_imported_type(&prefix_str, &name_str)
    } else {
      scope.lookup_type(&name_str)
    };

    if let Some(ref alias_ref) = found {
      if !alias_ref.type_params.is_empty()
        || !alias_ref.type_pack_params.is_empty()
        || ref_.has_parameter_list
      {
        let mut parameters: Vec<TypeId> = Vec::new();
        let mut pack_parameters: Vec<TypePackId> = Vec::new();

        for &param in ref_.parameters.as_slice() {
          match param {
            AstTypeOrPack::Type(ty) => {
              // `Type` 载荷按 `AstTypeOrPack` 构造契约指向 arena 存活 `AstType`。
              let param_ty = self.resolve_type_inner(scope, ty, true, false);
              parameters.push(param_ty);
            }
            AstTypeOrPack::Pack(pack) => {
              // `Pack` 载荷同上指向 arena 存活 `AstTypePack`。
              let tp =
                self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(scope, pack, true, false);

              // If we need more regular type_arguments, we can use single
              // element type packs to fill those in.
              if parameters.len() < alias_ref.type_params.len()
                // size 已 Rust 化：C++ 默认 nullptr 形参 → Option<&TxnLog> 的 None。
                && size(tp, None) == 1
                // finite 已 Option<&TxnLog> 化：None 即 C++ 默认 nullptr，只做无事务读。
                && finite(tp, None)
                && let Some(ty_val) = first(tp, false)
              {
                parameters.push(ty_val);
                continue;
              }
              pack_parameters.push(tp);
            }
            // cpp 两侧皆 null 时只走 `LUAU_ASSERT(false)`（ConstraintGenerator.cpp:4623）。
            AstTypeOrPack::Error => LUAU_ASSERT!(false),
          }
        }

        // `prefix`/`name` 为存活节点 `ref_` 的按位拷出字段（C++
        // `PendingExpansionType{ref->prefix, ref->name, ...}` 同样读取）。
        let pending = PendingExpansionType::pending_expansion_type_pending_expansion_type(
          ref_.prefix,
          ref_.name,
          parameters,
          pack_parameters,
        );
        result = self.arena.get_mut().add_type(pending);

        if !in_type_arguments {
          let constraint = TypeAliasExpansionConstraint { target: result };
          self.add_constraint_scope_ptr_location_constraint_v(
            scope,
            ty.base.location,
            ConstraintV::TypeAliasExpansion(constraint),
          );
        }
      } else {
        result = alias_ref.r#type();
      }
    } else {
      // 查不到别名时只读构造期注入的 builtin_types 句柄上的 `error_type` 槽
      // （对应 C++ `builtinTypes->errorType`，cpp:4656）。
      result = self.builtin_types.get().error_type;
      if replace_error_with_fresh {
        result = self.fresh_type(scope, Polarity::Mixed);
      }
    }

    let follow_result = follow_type::follow(result);
    // 对照 C++：`if (get<TypeFunctionInstanceType>(result))`
    if get_type::get::<TypeFunctionInstanceType>(follow_result).is_some() {
      let location = ty.base.location;
      self.report_error(
        location,
        TypeErrorData::UnappliedTypeFunction(UnappliedTypeFunction),
      );
      let reduce_constraint = ReduceConstraint { ty: result };
      self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        location,
        ConstraintV::Reduce(reduce_constraint),
      );
    }

    let follow_result = follow_type::follow(result);
    // 对照 C++：`if (auto gt = getMutable<GenericType>(follow(result))) gt->polarity = ...`
    if let Some(generic_type) = get_mutable::<GenericType>(follow_result) {
      let current_polarity = generic_type.polarity;
      generic_type.polarity = (current_polarity & Polarity::Mixed) | self.polarity;
    }

    result
  }
}
