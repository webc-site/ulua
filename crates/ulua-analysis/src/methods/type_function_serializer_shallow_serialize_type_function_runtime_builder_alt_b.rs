//! Faithful port of `TypeFunctionSerializer::shallowSerialize(TypePackId tp)`
//! (Analysis/src/TypeFunctionRuntimeBuilder.cpp:257-292).
use alloc::{format, string::String, vec::Vec};
use core::ptr::null;

use ulua_ast::records::location::Location;
use ulua_common::FFlag;

use crate::{
  functions::{
    follow_type_pack::follow_type_pack_id, get_type_pack::get_type_pack_id,
    to_string_to_string_alt_d::to_string_type_pack_id,
  },
  records::{
    generic_type_pack::GenericTypePack, type_function_error::TypeFunctionError,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_serializer::TypeFunctionSerializer,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_type_pack_var::TypeFunctionTypePackVar,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack, type_pack::TypePack,
    unsupported_type_pack::UnsupportedTypePack, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    type_function_error_data::TypeFunctionErrorData, type_function_kind::TypeFunctionKind,
    type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariant, type_or_pack::TypeOrPack,
    type_pack_id::TypePackId,
  },
};
impl TypeFunctionSerializer {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn shallow_serialize_type_pack_id(
    &mut self,
    tp: TypePackId,
  ) -> TypeFunctionTypePackId {
    let tp = unsafe { follow_type_pack_id(tp) };

    // if (auto it = find(tp)) return *it;
    if let Some(it) = self.packs.get(&tp).copied() {
      return it;
    }

    let state = self.state;
    let runtime = self.type_function_runtime;

    // Create a shallow serialization
    let mut target: TypeFunctionTypePackId = null();

    unsafe {
      if !get_type_pack_id::<TypePack>(tp).is_none() {
        target = (*runtime)
          .type_pack_arena
          .allocate(TypeFunctionTypePackVar::new(
            TypeFunctionTypePackVariant::V0(TypeFunctionTypePack {
              head: Vec::new(),
              tail: None,
            }),
          ));
      } else if !get_type_pack_id::<VariadicTypePack>(tp).is_none() {
        target = (*runtime)
          .type_pack_arena
          .allocate(TypeFunctionTypePackVar::new(
            TypeFunctionTypePackVariant::V1(TypeFunctionVariadicTypePack { type_id: null() }),
          ));
      } else if let Some(g_pack) = get_type_pack_id::<GenericTypePack>(tp).as_ref() {
        let mut name = g_pack.name.clone();

        if !g_pack.explicit_name {
          name = format!("g{}", g_pack.index);
        }

        target = (*runtime)
          .type_pack_arena
          .allocate(TypeFunctionTypePackVar::new(
            TypeFunctionTypePackVariant::V2(TypeFunctionGenericTypePack {
              is_named: g_pack.explicit_name,
              name,
            }),
          ));
      } else {
        if FFlag::LuauTypeFunctionStructuredErrors.get() {
          (*state).errors.push(TypeFunctionError {
            location: Location::default(),
            module_name: String::new(),
            data: TypeFunctionErrorData::V1(UnsupportedTypePack { pack: tp }),
          });
        } else {
          (*state).errors_deprecated.push(format!(
            "Argument of type pack {} is not currently serializable by type functions",
            to_string_type_pack_id(tp)
          ));
        }
      }
    }

    // packs[tp] = target;
    *self.packs.get_or_insert(tp) = target;
    // queue.emplace_back(tp, target);
    self
      .queue
      .push((TypeOrPack::V1(tp), TypeFunctionKind::V1(target)));
    target
  }
}
