use core::{
  ffi::c_void,
  mem::size_of,
  ptr::{null_mut, write},
};

use ulua_ast::{
  enums::ast_table_access::AstTableAccess,
  records::{
    ast_array::AstArray, ast_name::AstName, ast_table_indexer::AstTableIndexer,
    ast_table_prop::AstTableProp, ast_type::AstType, ast_type_or_pack::AstTypeOrPack,
    ast_type_reference::AstTypeReference, ast_type_table::AstTypeTable, location::Location,
  },
};

use crate::{
  functions::allocate_string_type_attach::allocate_string_luau_allocator_string_view,
  records::{
    recursion_counter::RecursionCounter, table_type::TableType,
    type_rehydration_visitor::TypeRehydrationVisitor,
  },
};
impl TypeRehydrationVisitor {
  pub fn operator_call_17(&mut self, ttv: &TableType) -> *mut AstType {
    let _counter = unsafe { RecursionCounter::recursion_counter_i32(&mut self.count) };

    if let Some(ref name) = ttv.name
      && !self.options.banned_names.contains(name)
    {
      let params_size =
        ttv.instantiated_type_params.len() + ttv.instantiated_type_pack_params.len();
      let params_data = unsafe {
        (*self.allocator).allocate(size_of::<AstTypeOrPack>() * params_size) as *mut AstTypeOrPack
      };

      let mut idx = 0;
      for &ty_param in &ttv.instantiated_type_params {
        let rehydrated = unsafe { self.visit_type(ty_param) };
        unsafe {
          write(
            params_data.add(idx),
            AstTypeOrPack {
              r#type: rehydrated,
              type_pack: null_mut(),
            },
          );
        }
        idx += 1;
      }

      for &tp_param in &ttv.instantiated_type_pack_params {
        let rehydrated = self.rehydrate(tp_param);
        unsafe {
          write(
            params_data.add(idx),
            AstTypeOrPack {
              r#type: null_mut(),
              type_pack: rehydrated,
            },
          );
        }
        idx += 1;
      }

      let allocator = unsafe { &mut *self.allocator };
      let name_cstr = allocate_string_luau_allocator_string_view(allocator, name);
      let name_ast = AstName::ast_name_c_char(name_cstr);

      let parameters = AstArray {
        data: params_data,
        size: params_size,
      };

      let ref_node = AstTypeReference::new(
        Location::default(),
        None,
        name_ast,
        None,
        Location::default(),
        params_size != 0,
        parameters,
      );

      return allocator.alloc(ref_node) as *mut AstType;
    }

    if self.has_seen(ttv as *const TableType as *const c_void) {
      let allocator = unsafe { &mut *self.allocator };
      let name_cstr = if let Some(ref name) = ttv.name {
        allocate_string_luau_allocator_string_view(allocator, name)
      } else {
        allocate_string_luau_allocator_string_view(allocator, "<Cycle>")
      };
      let name_ast = AstName::ast_name_c_char(name_cstr);

      let ref_node = AstTypeReference::new(
        Location::default(),
        None,
        name_ast,
        None,
        Location::default(),
        false,
        AstArray {
          data: null_mut(),
          size: 0,
        },
      );

      return allocator.alloc(ref_node) as *mut AstType;
    }

    let props_size = ttv.props.len();
    let props_data = unsafe {
      (*self.allocator).allocate(size_of::<AstTableProp>() * props_size) as *mut AstTableProp
    };

    let mut idx = 0;
    for (prop_name, prop) in &ttv.props {
      let _counter_inner = unsafe { RecursionCounter::recursion_counter_i32(&mut self.count) };

      let name_cstr = {
        let allocator = unsafe { &mut *self.allocator };
        allocate_string_luau_allocator_string_view(allocator, prop_name)
      };
      let name_ast = AstName::ast_name_c_char(name_cstr);

      if prop.is_shared() {
        let read_ty_rehydrated = unsafe { self.visit_type(prop.read_ty.unwrap()) };
        unsafe {
          write(
            props_data.add(idx),
            AstTableProp {
              name: name_ast,
              location: Location::default(),
              r#type: read_ty_rehydrated,
              access: AstTableAccess::ReadWrite,
              access_location: None,
            },
          );
        }
        idx += 1;
      } else {
        if let Some(read_ty) = prop.read_ty {
          let read_ty_rehydrated = unsafe { self.visit_type(read_ty) };
          unsafe {
            write(
              props_data.add(idx),
              AstTableProp {
                name: name_ast,
                location: Location::default(),
                r#type: read_ty_rehydrated,
                access: AstTableAccess::Read,
                access_location: None,
              },
            );
          }
          idx += 1;
        }

        if let Some(write_ty) = prop.write_ty {
          let write_ty_rehydrated = unsafe { self.visit_type(write_ty) };
          unsafe {
            write(
              props_data.add(idx),
              AstTableProp {
                name: name_ast,
                location: Location::default(),
                r#type: write_ty_rehydrated,
                access: AstTableAccess::Write,
                access_location: None,
              },
            );
          }
          idx += 1;
        }
      }
    }

    let indexer = if let Some(ref indexer_ref) = ttv.indexer {
      let _counter_indexer = unsafe { RecursionCounter::recursion_counter_i32(&mut self.count) };

      let index_type = unsafe { self.visit_type(indexer_ref.index_type) };
      let result_type = unsafe { self.visit_type(indexer_ref.index_result_type) };

      let indexer_node = AstTableIndexer {
        index_type,
        result_type,
        location: Location::default(),
        access: AstTableAccess::ReadWrite,
        access_location: None,
      };

      let allocator = unsafe { &mut *self.allocator };
      allocator.alloc(indexer_node)
    } else {
      null_mut()
    };

    let props_array = AstArray {
      data: props_data,
      size: props_size,
    };

    let table_node = AstTypeTable::new(Location::default(), props_array, indexer);

    let allocator = unsafe { &mut *self.allocator };
    allocator.alloc(table_node) as *mut AstType
  }
}
