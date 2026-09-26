use alloc::{string::String, vec::Vec};
use core::ptr::{NonNull, null_mut};

use ulua_ast::records::{
  ast_type::AstType, ast_type_or_pack::AstTypeOrPack, ast_type_reference::AstTypeReference,
};
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::polarity::Polarity,
  functions::{
    arc_as_mut::arc_as_mut, finite::finite, first::first, follow_type,
    get_mutable_type::get_mutable, get_type, size_type_pack::size,
  },
  records::{
    blocked_type::BlockedType, constraint_generator::ConstraintGenerator,
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
  /// # Safety
  /// `ty` 与 `ref_` 须非空、对齐，并指向**同一处**本模块 parse arena 中整个 check
  /// 会话存活的类型 AST 节点：唯一调用方 [`ConstraintGenerator::resolve_type_inner`]
  /// 先对 `ty` 做 `ast_node_try_as::<AstTypeReference>` 下转、再 `NonNull::from` 取指针
  /// 传入，故二者指向同一 place；`(*ref_).parameters` 的 data/size 由 arena 成对写入，
  /// 各元素为 `AstTypeOrPack` 的 `Type`/`Pack`/`Error` 三态（parser 保证；`Error`
  /// 落入体内 `LUAU_ASSERT!(false)`）；`scope` 为存活作用域对象的共享引用。对应 C++
  /// `resolveReferenceType(const ScopePtr&, AstType*, AstTypeReference*, bool, bool)`
  /// （ConstraintGenerator.cpp:4566）的引用形参契约。
  pub unsafe fn resolve_reference_type(
    &mut self,
    scope: &ScopePtr,
    ty: *mut AstType,
    ref_: *mut AstTypeReference,
    in_type_arguments: bool,
    replace_error_with_fresh: bool,
  ) -> TypeId {
    let scope_ptr = arc_as_mut(scope);
    let mut result: TypeId;

    if fflag::DebugLuauMagicTypes.get() {
      // Safety: `ref_` 按本函数契约指向存活 `AstTypeReference`，此处仅读 `name` 槽。
      let ref_name_str = unsafe { (*ref_).name.as_str_or_empty() };

      if ref_name_str == "_luau_ice" {
        // Safety: `ty` 同为契约存活的 AST 节点，此处仅读 location。
        let location = unsafe { (*ty).base.location };
        // Safety: `self.ice.as_ptr()` 是构造期 `NonNull<InternalErrorReporter>` 注入的
        // 句柄，`ice_string_location` 取 `&self` 只上报内部错误
        // （C++ `ice->ice(msg, ty->location)`）。
        self
          .ice
          .get()
          .ice_string_location("_luau_ice encountered", &location);
      } else if ref_name_str == "_luau_print" {
        // Safety: 解引用契约存活的 `ty` 仅读 location。
        let location = unsafe { (*ty).base.location };
        // Safety: `ref_` 按本函数契约指向存活 `AstTypeReference`，只读借用其
        // `parameters`——仅取其 `size`/`data`，data/size 成对由 arena 保证。
        let params = unsafe { &(*ref_).parameters };
        // 对应 cpp `params.size != 1 || !params.data[0].type`：`as_slice().first()`
        // 仅在 `size == 1` 时读取首元素（data/size 成对由 arena 保证），`as_type`
        // 把「非 `Type` 形态（含错误态）」折叠为 `None`，与 cpp 读 null 同值。
        let print_arg: Option<&'static AstType> = if params.size == 1 {
          params.as_slice().first().and_then(AstTypeOrPack::as_type)
        } else {
          None
        };
        if let Some(arg_ty) = print_arg {
          // Safety: `Type` 变体载荷按 `AstTypeOrPack` 构造契约指向 arena 存活
          // `AstType` 节点，`NonNull::as_ptr` 只还原同一地址（等价 cpp `data[0].type`
          // 槽值），满足 `resolve_type_inner` 对其裸指针形参的契约。
          let param_ty = NonNull::from(arg_ty).as_ptr();
          return self.resolve_type_inner(scope_ptr, param_ty, in_type_arguments, false);
        } else {
          let err = GenericError::new(String::from("_luau_print requires one generic parameter"));
          self.report_error(location, TypeErrorData::GenericError(err));
          // Safety: `self.module` 由构造断言为 `Some`、Arc 被 generator 持有至会话末，
          // `arc_as_mut` 句柄按 C++ const_cast 惯用法写 `module->astResolvedTypes`
          // （单线程独占写）；`ty` 是契约存活节点、作 map 键稳定；`self.builtin_types.as_ptr()`
          // 为构造期 `NonNull<BuiltinTypes>` 句柄，此处只读 `error_type` 槽。
          unsafe {
            let module_ptr = arc_as_mut(
              self
                .module
                .as_ref()
                .expect("构造断言 module 为 Some 且 generator 持有 Arc 至会话末"),
            );
            *(*module_ptr)
              .ast_resolved_types
              .get_or_insert(ty as *const _) = self.builtin_types.get().error_type;
            return self.builtin_types.get().error_type;
          }
        }
      } else if ref_name_str == "_luau_blocked_type" {
        // Safety: `self.arena.as_ptr()` 为构造期从 `NonNull<Normalizer>` 解出的 arena 句柄，
        // 与生成会话同寿；`add_type` 是单线程生成器独占的 arena 追加写。
        return self.arena.get_mut().add_type(BlockedType::default());
      }
    }

    // Safety: `ref_` 按本函数契约存活可读，此处仅取 `name` 槽（与上方魔法名读取同源）。
    let name_str = unsafe { (*ref_).name.as_str_or_empty().to_string() };
    // Safety: 同上，读 `prefix`（Option<AstName>，按位拷出），不产生越过本行的借用。
    let alias: Option<TypeFun> = if let Some(prefix_name) = unsafe { (*ref_).prefix } {
      let prefix_str = prefix_name.as_str_or_empty().to_string();
      scope.lookup_imported_type(&prefix_str, &name_str)
    } else {
      scope.lookup_type(&name_str)
    };

    if let Some(ref alias_ref) = alias {
      if !alias_ref.type_params.is_empty()
        || !alias_ref.type_pack_params.is_empty()
        // Safety: `||` 短路走到此处才读 `(*ref_).has_parameter_list`；`ref_` 按
        // 本函数契约存活，读布尔位无副作用。
        || unsafe { (*ref_).has_parameter_list }
      {
        let mut parameters: Vec<TypeId> = Vec::new();
        let mut pack_parameters: Vec<TypePackId> = Vec::new();

        // Safety: 对存活节点 `ref_` 的 `parameters` 数组做只读借用，data/size 成对
        // 由 arena 写入，迭代所得 `&AstTypeOrPack` 生命周期锚定本块。
        let params_array = unsafe { &(*ref_).parameters };
        for &param in params_array.as_slice() {
          match param {
            AstTypeOrPack::Type(ty) => {
              // `Type` 载荷按 `AstTypeOrPack` 构造契约指向 arena 存活 `AstType`，
              // `as_ptr` 只还原同一地址（等价 cpp `param.type` 槽值）。
              let param_ty =
                self.resolve_type_inner(scope_ptr, NonNull::from(ty).as_ptr(), true, false);
              parameters.push(param_ty);
            }
            AstTypeOrPack::Pack(pack) => {
              // Safety: `Pack` 载荷同上指向 arena 存活 `AstTypePack`；`scope_ptr`
              // 契约同上，整次调用内有效。
              let tp = unsafe {
                self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
                  scope_ptr,
                  NonNull::from(pack).as_ptr(),
                  true,
                  false,
                )
              };

              // If we need more regular type_arguments, we can use single
              // element type packs to fill those in.
              if parameters.len() < alias_ref.type_params.len()
                // size 已 Rust 化：C++ 默认 nullptr 形参 → Option<&TxnLog> 的 None。
                && size(tp, None) == 1
                // Safety: 同上——`tp` 存活、log=null 合法，finite 只做无事务读。
                && unsafe { finite(tp, null_mut()) }
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

        // Safety: `prefix`/`name` 为存活节点 `ref_` 的按位拷出字段（C++
        // `PendingExpansionType{ref->prefix, ref->name, ...}` 同样读取），不产生借用。
        let pending = PendingExpansionType::pending_expansion_type_pending_expansion_type(
          unsafe { (*ref_).prefix },
          unsafe { (*ref_).name },
          parameters,
          pack_parameters,
        );
        // Safety: 构造期注入的 normalizer arena 句柄，`add_type` 独占追加。
        result = self.arena.get_mut().add_type(pending);

        if !in_type_arguments {
          // Safety: 契约存活节点 `ty` 读 location（C++ `addConstraint(scope,
          // ty->location, TypeAliasExpansionConstraint{result})`）。
          let location = unsafe { (*ty).base.location };
          let constraint = TypeAliasExpansionConstraint { target: result };
          self.add_constraint_scope_ptr_location_constraint_v(
            scope,
            location,
            ConstraintV::TypeAliasExpansion(constraint),
          );
        }
      } else {
        result = alias_ref.r#type();
      }
    } else {
      // Safety: 查不到别名时只读构造期注入的 `self.builtin_types.as_ptr()` 句柄上的
      // `error_type` 槽（对应 C++ `builtinTypes->errorType`，cpp:4656），无写操作。
      result = self.builtin_types.get().error_type;
      if replace_error_with_fresh {
        result = self.fresh_type(scope, Polarity::Mixed);
      }
    }

    let follow_result = follow_type::follow(result);
    // 对照 C++：`if (get<TypeFunctionInstanceType>(result))`
    if get_type::get::<TypeFunctionInstanceType>(follow_result).is_some() {
      // Safety: 收尾处再读契约存活节点 `ty` 的 location（C++ `reportError(ty->location,
      // UnappliedTypeFunction{})`）。
      let location = unsafe { (*ty).base.location };
      self.report_error(
        location,
        TypeErrorData::UnappliedTypeFunction(UnappliedTypeFunction::default()),
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
