use core::ffi::c_char;

use ulua_ast::{
  records::{
    ast_array::AstArray, ast_generic_type::AstGenericType, ast_name::AstName, ast_node::AstNode,
    ast_stat_type_alias::AstStatTypeAlias, ast_type::AstType, ast_type_function::AstTypeFunction,
    ast_type_group::AstTypeGroup, ast_type_intersection::AstTypeIntersection,
    ast_type_optional::AstTypeOptional, ast_type_reference::AstTypeReference,
    ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
    ast_type_union::AstTypeUnion,
  },
  rtti::{ast_node_as, ast_node_is},
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{
  FFlag,
  enums::luau_bytecode_type::{
    LBC_TYPE_ANY, LBC_TYPE_BOOLEAN, LBC_TYPE_FUNCTION, LBC_TYPE_INVALID, LBC_TYPE_NIL,
    LBC_TYPE_OPTIONAL_BIT, LBC_TYPE_STRING, LBC_TYPE_TABLE, LBC_TYPE_TAGGED_USERDATA_BASE,
    LBC_TYPE_USERDATA, LBC_TYPE_VECTOR, LuauBytecodeType,
  },
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::functions::{get_primitive_type::get_primitive_type, is_generic::is_generic};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_type(
  ty: *const AstType,
  generics: AstArray<*mut AstGenericType>,
  type_aliases: &DenseHashMap<AstName, *mut AstStatTypeAlias>,
  resolve_aliases_deprecated: bool,
  host_vector_type: *const c_char,
  userdata_types: &DenseHashMap<AstName, u8>,
  bytecode: &mut BytecodeBuilder,
  seen_aliases: &mut DenseHashSet<AstName>,
) -> LuauBytecodeType {
  if ty.is_null() {
    return LBC_TYPE_ANY;
  }

  let node = ty as *mut AstNode;

  if let Some(ref_node) = unsafe { ast_node_as::<AstTypeReference>(node).as_ref() } {
    if ref_node.prefix.is_some() {
      return LBC_TYPE_ANY;
    }

    // C++ `if (alias && *alias)` — the entry may exist with a NULL value (a
    // block-scoped alias restored to its previous null binding on scope exit), in
    // which case it is NOT in scope and we must fall through to the generic/userdata
    // resolution. Only resolve when the stored pointer is non-null.
    if let Some(alias_ptr) = type_aliases
      .find(&ref_node.name)
      .copied()
      .filter(|p| !p.is_null())
    {
      let alias = unsafe { &*alias_ptr };
      if FFlag::LuauCompileTypeAliases.get() {
        if seen_aliases.contains(&alias.name) {
          seen_aliases.clear();
          return LBC_TYPE_ANY;
        } else {
          seen_aliases.insert(ref_node.name);
          return unsafe {
            get_type(
              alias.type_ptr,
              alias.generics,
              type_aliases,
              /* resolveAliases_DEPRECATED= */ false,
              host_vector_type,
              userdata_types,
              bytecode,
              seen_aliases,
            )
          };
        }
      } else {
        // note: we only resolve aliases to the depth of 1 to avoid dealing with recursive aliases
        if resolve_aliases_deprecated {
          return unsafe {
            get_type(
              alias.type_ptr,
              alias.generics,
              type_aliases,
              /* resolveAliases_DEPRECATED= */ false,
              host_vector_type,
              userdata_types,
              bytecode,
              seen_aliases,
            )
          };
        } else {
          return LBC_TYPE_ANY;
        }
      }
    }

    if is_generic(ref_node.name, &generics) {
      return LBC_TYPE_ANY;
    }

    if !host_vector_type.is_null() && unsafe { ref_node.name.operator_eq_raw(host_vector_type) } {
      return LBC_TYPE_VECTOR;
    }

    let prim = get_primitive_type(ref_node.name);
    if prim != LBC_TYPE_INVALID {
      return prim;
    }

    if let Some(userdata_index) = userdata_types.find(&ref_node.name) {
      bytecode.use_userdata_type(*userdata_index as u32);
      return LuauBytecodeType(LBC_TYPE_TAGGED_USERDATA_BASE.0 + *userdata_index as u16);
    }

    // not primitive or alias or generic => host-provided, we assume userdata for now
    return LBC_TYPE_USERDATA;
  } else if ast_node_is::<AstTypeTable>(node) {
    return LBC_TYPE_TABLE;
  } else if ast_node_is::<AstTypeFunction>(node) {
    return LBC_TYPE_FUNCTION;
  } else if let Some(un) = unsafe { ast_node_as::<AstTypeUnion>(node).as_ref() } {
    let mut optional = false;
    let mut r#type = LBC_TYPE_INVALID;

    for &ty in un.types.as_slice() {
      let et = unsafe {
        get_type(
          ty,
          generics,
          type_aliases,
          resolve_aliases_deprecated,
          host_vector_type,
          userdata_types,
          bytecode,
          seen_aliases,
        )
      };

      if et == LBC_TYPE_NIL {
        optional = true;
        continue;
      }

      if r#type == LBC_TYPE_INVALID {
        r#type = et;
        continue;
      }

      if r#type != et {
        return LBC_TYPE_ANY;
      }
    }

    if r#type == LBC_TYPE_INVALID {
      return LBC_TYPE_ANY;
    }

    return LuauBytecodeType(
      r#type.0
        | (if optional && (r#type != LBC_TYPE_ANY) {
          LBC_TYPE_OPTIONAL_BIT.0
        } else {
          0
        }),
    );
  } else if ast_node_is::<AstTypeIntersection>(node) {
    return LBC_TYPE_ANY;
  } else if let Some(group) = unsafe { ast_node_as::<AstTypeGroup>(node).as_ref() } {
    return unsafe {
      get_type(
        group.type_,
        generics,
        type_aliases,
        resolve_aliases_deprecated,
        host_vector_type,
        userdata_types,
        bytecode,
        seen_aliases,
      )
    };
  } else if ast_node_is::<AstTypeOptional>(node) {
    return LBC_TYPE_NIL;
  } else if ast_node_is::<AstTypeSingletonBool>(node) {
    return LBC_TYPE_BOOLEAN; // C++ returns LBC_TYPE_BOOLEAN for `true`/`false`
  } else if ast_node_is::<AstTypeSingletonString>(node) {
    return LBC_TYPE_STRING;
  }

  LBC_TYPE_ANY
}
