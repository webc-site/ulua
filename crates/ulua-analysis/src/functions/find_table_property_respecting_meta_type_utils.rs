use ulua_ast::records::location::Location;

use crate::{
  enums::value_context::ValueContext,
  functions::{
    find_metatable_entry::find_metatable_entry, first::first, follow_type, follow_type_pack,
    get_table_type::get_table_type, get_type, to_string_to_string::to_string_type_id,
  },
  records::{
    any_type::AnyType, arena_handle::Handle, builtin_types::BuiltinTypes,
    function_type::FunctionType, generic_error::GenericError, type_error::TypeError,
  },
  type_aliases::{error_vec::ErrorVec, type_error_data::TypeErrorData, type_id::TypeId},
};

/// 对应 C++ `findTablePropertyRespectingMeta(NotNull<BuiltinTypes>, ErrorVec&, TypeId, const std::string&, ValueContext, Location, bool)`。
pub fn find_table_property_respecting_meta(
  builtin_types: Handle<BuiltinTypes>,
  errors: &mut ErrorVec,
  ty: TypeId,
  name: &str,
  context: ValueContext,
  location: Location,
  use_new_solver: bool,
) -> Option<TypeId> {
  // Safety: 本块与 C++ 函数体一一对应，其中真正的 unsafe 操作只有对 `builtin_types` 的两次
  // 解引用（读 `nil_type`/`any_type` 两个 `Copy` TypeId 句柄）。该指针的非空与长寿性由
  // 函数级契约（C++ `NotNull<BuiltinTypes>` 形参）给出：唯一入口是本模块内的安全包装，
  // 其实参为 `TypeChecker.builtin_types`（构造期接线的 NotNull 裸指针）。其余
  // `get_type::get::<T>(ty)`/`get_table_type(ty)`/`find_metatable_entry(..)` 都是安全函数，
  // 只要求 `ty`/`index` 为存活句柄——它们来自调用方的 arena 类型与 `follow_type_id` 的
  // 归一结果，且 `__index` 链最多迭代 100 次，不会对已释放结点反复跟踪。`errors` 以
  // `&mut` 独占借出，块内只追加诊断，不与其他借用共享。
  {
    let any_type = get_type::get::<AnyType>(ty);
    if any_type.is_some() {
      return Some(ty);
    }

    let table_type = get_table_type(ty);
    if let Some(tt) = table_type
      && let Some(prop) = tt.props.get(name)
    {
      match context {
        ValueContext::RValue => return prop.read_ty,
        ValueContext::LValue => return prop.write_ty,
      }
    }

    let mut mt_index = find_metatable_entry(builtin_types, errors, ty, "__index", location);
    let mut count = 0;

    while let Some(index) = mt_index {
      if count >= 100 {
        return None;
      }
      count += 1;

      let index = follow_type::follow(index);

      if let Some(itt) = get_table_type(index) {
        if let Some(fit) = itt.props.get(name) {
          if use_new_solver {
            match context {
              ValueContext::RValue => return fit.read_ty,
              ValueContext::LValue => return fit.write_ty,
            }
          } else {
            return fit.read_ty;
          }
        }
      } else if let Some(itf) = get_type::get::<FunctionType>(index) {
        let r = first(follow_type_pack::follow(itf.ret_types), false);
        if let Some(r) = r {
          return Some(r);
        } else {
          return Some(builtin_types.get().nil_type);
        }
      } else if get_type::get::<AnyType>(index).is_some() {
        return Some(builtin_types.get().any_type);
      } else {
        let type_str = to_string_type_id(index);
        errors.push(TypeError::type_error_location_type_error_data(
          location,
          TypeErrorData::GenericError(GenericError::new(format!(
            "__index should either be a function or table. Got {}",
            type_str
          ))),
        ));
      }

      mt_index = find_metatable_entry(
        builtin_types,
        errors,
        // 外层 `while let Some(index) = mt_index` 已把循环头的 mt_index 绑定为
        // Some，块内除本赋值点外无其它写点，故此处必为 Some。
        mt_index.expect("循环头 while-let 已确认 Some，赋值点前无写路径"),
        "__index",
        location,
      );
    }

    None
  }
}
