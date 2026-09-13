use core::ffi::c_char;
use std::ffi::CString;

use ulua_analysis::{
  records::primitive_type::{PrimitiveType, Type as PrimitiveKind},
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
use ulua_common::{FFlag::LuauIntegerType2, LUAU_ASSERT};
use ulua_vm::{
  functions::{lua_pushstring::lua_pushstring, lua_setfield::lua_setfield},
  macros::lua_newtable::lua_newtable,
  records::lua_state::lua_State,
};

unsafe fn push_literal(l: *mut lua_State, value: &'static [u8]) {
  unsafe {
    lua_pushstring(l, value.as_ptr() as *const c_char);
  }
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn populate_rtti(l: *mut lua_State, ty: TypeId) {
  unsafe {
    LUAU_ASSERT!(!ty.is_null());

    match &(*ty).ty {
      TypeVariant::Primitive(PrimitiveType { r#type, .. }) => match r#type {
        PrimitiveKind::Boolean => push_literal(l, b"boolean\0"),
        PrimitiveKind::NilType => push_literal(l, b"nil\0"),
        PrimitiveKind::Number => push_literal(l, b"number\0"),
        PrimitiveKind::Integer => {
          if LuauIntegerType2.get() {
            push_literal(l, b"integer\0");
          }
        }
        PrimitiveKind::String => push_literal(l, b"string\0"),
        PrimitiveKind::Thread => push_literal(l, b"thread\0"),
        PrimitiveKind::Buffer => push_literal(l, b"Buffer\0"),
        _ => LUAU_ASSERT!(false, "Unknown primitive type"),
      },
      TypeVariant::Table(table) => {
        lua_newtable(l);

        for (name, prop) in &table.props {
          if let Some(read_ty) = prop.read_ty {
            populate_rtti(l, read_ty);
          } else if let Some(write_ty) = prop.write_ty {
            populate_rtti(l, write_ty);
          } else {
            continue;
          }

          let field = CString::new(name.as_str()).expect("type property name contains nul");
          lua_setfield(l, -2, field.as_ptr());
        }
      }
      TypeVariant::Function(_) => push_literal(l, b"function\0"),
      TypeVariant::Any(_) => push_literal(l, b"any\0"),
      TypeVariant::Intersection(intersection) => {
        for part in &intersection.parts {
          LUAU_ASSERT!(matches!((*(*part)).ty, TypeVariant::Function(_)));
        }

        push_literal(l, b"function\0");
      }
      TypeVariant::Extern(extern_ty) => {
        let name = CString::new(extern_ty.name.as_str()).expect("extern type name contains nul");
        lua_pushstring(l, name.as_ptr());
      }
      _ => LUAU_ASSERT!(false, "Unknown type"),
    }
  }
}
