use core::{ffi::c_void, mem::size_of, ptr::null_mut};

use ulua_ast::{
  enums::ast_table_access::AstTableAccess,
  records::{
    ast_array::AstArray, ast_name::AstName, ast_table_indexer::AstTableIndexer,
    ast_table_prop::AstTableProp, ast_type::AstType, ast_type_reference::AstTypeReference,
    ast_type_table::AstTypeTable, location::Location,
  },
};

use crate::{
  functions::allocate_string_type_attach::allocate_string_luau_allocator_string_view,
  records::{
    extern_type::ExternType, recursion_counter::RecursionCounter,
    type_rehydration_visitor::TypeRehydrationVisitor,
  },
};
impl TypeRehydrationVisitor {
  pub fn operator_call_4(&mut self, etv: &ExternType) -> *mut AstType {
    let _counter = unsafe { RecursionCounter::recursion_counter_i32(&mut self.count) };

    let name_ptr = {
      let allocator = unsafe { &mut *self.allocator };
      allocate_string_luau_allocator_string_view(allocator, &etv.name)
    };

    if !self.options.expand_extern_type_props
      || self.has_seen(etv as *const ExternType as *const c_void)
      || self.count > 1
    {
      let reference = AstTypeReference::new(
        Location::default(),
        None,
        AstName::ast_name_c_char(name_ptr),
        None,
        Location::default(),
        false,
        AstArray::default(),
      );
      return unsafe { (*self.allocator).alloc(reference) as *mut AstType };
    }

    let props_size = etv.props.len();
    let props_data = unsafe {
      (*self.allocator).allocate(size_of::<AstTableProp>() * props_size) as *mut AstTableProp
    };

    let mut idx = 0;
    for (prop_name, prop) in &etv.props {
      let name = {
        let allocator = unsafe { &mut *self.allocator };
        allocate_string_luau_allocator_string_view(allocator, prop_name)
      };

      if prop.is_shared() {
        let read_type_ptr = unsafe { self.visit_type(prop.read_ty.unwrap()) };
        unsafe {
          props_data.add(idx).write(AstTableProp {
            name: AstName::ast_name_c_char(name),
            location: Location::default(),
            r#type: read_type_ptr,
            access: AstTableAccess::ReadWrite,
            access_location: None,
          });
        }
        idx += 1;
      } else {
        if let Some(read_ty) = prop.read_ty {
          let read_type_ptr = unsafe { self.visit_type(read_ty) };
          unsafe {
            props_data.add(idx).write(AstTableProp {
              name: AstName::ast_name_c_char(name),
              location: Location::default(),
              r#type: read_type_ptr,
              access: AstTableAccess::Read,
              access_location: None,
            });
          }
          idx += 1;
        }

        if let Some(write_ty) = prop.write_ty {
          let write_type_ptr = unsafe { self.visit_type(write_ty) };
          unsafe {
            props_data.add(idx).write(AstTableProp {
              name: AstName::ast_name_c_char(name),
              location: Location::default(),
              r#type: write_type_ptr,
              access: AstTableAccess::Write,
              access_location: None,
            });
          }
          idx += 1;
        }
      }
    }

    let props = AstArray {
      data: props_data,
      size: idx,
    };

    let indexer = if let Some(ref indexer_data) = etv.indexer {
      let _inner_counter = unsafe { RecursionCounter::recursion_counter_i32(&mut self.count) };

      let index_type = unsafe { self.visit_type(indexer_data.index_type) };
      let result_type = unsafe { self.visit_type(indexer_data.index_result_type) };

      let allocator = unsafe { &mut *self.allocator };
      allocator.alloc(AstTableIndexer {
        index_type,
        result_type,
        location: Location::default(),
        access: AstTableAccess::ReadWrite,
        access_location: None,
      })
    } else {
      null_mut()
    };

    let table = AstTypeTable::new(Location::default(), props, indexer);
    let allocator = unsafe { &mut *self.allocator };
    allocator.alloc(table) as *mut AstType
  }
}
