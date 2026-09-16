//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:990-1074`
//!
//! `void deserializeChildren(TypeFunctionFunctionType* f2, FunctionType* f1)`.
//! Introduces the function's generic parameters into the deserializer scope
//! (resolving by name, rejecting packs-in-type-position and duplicates), then
//! shallow-deserializes the generics, generic packs, arg/ret packs and argNames.
use alloc::{
  collections::BTreeSet,
  format,
  string::{String, ToString},
};
use core::ptr::null_mut;

use ulua_ast::records::location::Location;
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::polarity::Polarity,
  functions::{
    get_type_function_runtime_alt_n::get_type_function_type_pack_id,
    get_type_function_runtime_alt_o::get_type_function_type_id,
  },
  records::{
    function_argument::FunctionArgument, function_type::FunctionType, generic_type::GenericType,
    generic_type_pack::GenericTypePack, serialized_function_scope::SerializedFunctionScope,
    serialized_generic::SerializedGeneric, r#type::Type,
    type_function_deserializer::TypeFunctionDeserializer,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack, type_level::TypeLevel,
    type_pack_var::TypePackVar,
  },
};
impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn deserialize_children_type_function_function_type_function_type(
    &mut self,
    f2: *mut TypeFunctionFunctionType,
    f1: *mut FunctionType,
  ) {
    unsafe {
      self.function_scopes.push(SerializedFunctionScope {
        old_queue_size: self.queue.len(),
        function: f2,
      });
      let mut generic_names: BTreeSet<(bool, String)> = BTreeSet::new();

      let arena = (*(*self.state).ctx).arena.as_ptr();
      let scope = (*(*self.state).ctx).scope.as_ptr();

      // Introduce generic function parameters into scope
      for &ty in &(*f2).generics {
        let gty = get_type_function_type_id::<TypeFunctionGenericType>(ty);
        if gty.is_null() || (*gty).is_pack() {
          self.push_runtime_error("Encountered unexpected generic".to_string());
          return;
        } else {
          LUAU_ASSERT!(!gty.is_null() && !(*gty).is_pack());
        }

        let name_key = ((*gty).is_named(), (*gty).name().to_string());

        // Duplicates are not allowed
        if generic_names.contains(&name_key) {
          self.push_runtime_error(format!("Duplicate type parameter '{}'", (*gty).name()));
          return;
        }

        generic_names.insert(name_key);

        let mapping = if (*gty).is_named() {
          (*arena).add_tv(Type::from(GenericType::generic_type_scope_name(
            scope,
            &(*gty).name().to_string(),
          )))
        } else {
          (*arena).add_tv(Type::from(GenericType::new()))
        };
        self.generic_types.push(SerializedGeneric {
          is_named: (*gty).is_named(),
          name: (*gty).name().to_string(),
          r#type: mapping,
        });
      }

      for &tp in &(*f2).generic_packs {
        let gtp = get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tp);
        if gtp.is_null() {
          self.push_runtime_error("Encountered unexpected generic type pack".to_string());
          return;
        } else {
          LUAU_ASSERT!(!gtp.is_null());
        }

        let name_key = ((*gtp).is_named(), (*gtp).name().to_string());

        // Duplicates are not allowed
        if generic_names.contains(&name_key) {
          self.push_runtime_error(format!("Duplicate type parameter '{}'", (*gtp).name()));
          return;
        }

        generic_names.insert(name_key);

        let mapping = if (*gtp).is_named() {
          let r#gen = GenericTypePack {
            index: 0,
            level: TypeLevel::default(),
            scope,
            name: (*gtp).name().to_string(),
            explicit_name: true,
            polarity: Polarity::Unknown,
          };
          (*arena).add_type_pack_t(TypePackVar::from(r#gen))
        } else {
          let mut r#gen = GenericTypePack {
            index: 0,
            level: TypeLevel::default(),
            scope: null_mut(),
            name: String::new(),
            explicit_name: false,
            polarity: Polarity::Unknown,
          };
          r#gen.generic_type_pack();
          (*arena).add_type_pack_t(TypePackVar::from(r#gen))
        };
        self.generic_packs.push(SerializedGeneric {
          is_named: (*gtp).is_named(),
          name: (*gtp).name().to_string(),
          r#type: mapping,
        });
      }

      (*f1).generics.reserve((*f2).generics.len());
      for &ty in &(*f2).generics {
        let g = self.shallow_deserialize_type_function_type_id(ty);
        (*f1).generics.push(g);
      }

      (*f1).generic_packs.reserve((*f2).generic_packs.len());
      for &tp in &(*f2).generic_packs {
        let g = self.shallow_deserialize_type_function_type_pack_id(tp);
        (*f1).generic_packs.push(g);
      }

      if !(*f2).arg_types.is_null() {
        (*f1).arg_types = self.shallow_deserialize_type_function_type_pack_id((*f2).arg_types);
      }

      if !(*f2).ret_types.is_null() {
        (*f1).ret_types = self.shallow_deserialize_type_function_type_pack_id((*f2).ret_types);
      }

      if FFlag::LuauTypeFunctionSerializeArgNames.get() {
        (*f1).arg_names.reserve((*f2).arg_names.len());
        for name in &(*f2).arg_names {
          if let Some(name) = name {
            (*f1).arg_names.push(Some(FunctionArgument {
              name: name.clone(),
              location: Location::default(),
            }));
          } else {
            (*f1).arg_names.push(None);
          }
        }
      }
    }
  }
}
