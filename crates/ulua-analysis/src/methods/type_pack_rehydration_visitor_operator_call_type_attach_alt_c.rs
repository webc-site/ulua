use core::{mem::size_of, ptr::null_mut};

use ulua_ast::records::{
  ast_array::AstArray, ast_type::AstType, ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
  ast_type_pack_explicit::AstTypePackExplicit, location::Location,
};

use crate::records::{
  type_pack::TypePack, type_pack_rehydration_visitor::TypePackRehydrationVisitor,
};
impl TypePackRehydrationVisitor {
  #[inline]
  pub fn operator_call_7(&self, tp: &TypePack) -> *mut AstTypePack {
    let allocator = unsafe { &mut *self.allocator };

    let head_size = tp.head.len();
    let head_data = allocator.allocate(size_of::<*mut AstType>() * head_size) as *mut *mut AstType;
    for (i, &type_id) in tp.head.iter().enumerate() {
      // C++ `head.data[i] = Luau::visit(*typeVisitor, tp.head[i]->ty);`
      let type_visitor = unsafe { &mut *self.type_visitor };
      let ast_type = unsafe { type_visitor.visit_type(type_id) };
      unsafe { *head_data.add(i) = ast_type };
    }

    let head = AstArray {
      data: head_data,
      size: head_size,
    };

    // C++ `if (tp.tail) tail = Luau::visit(*this, (*tp.tail)->ty);`
    let tail = if let Some(tail_tp_id) = tp.tail {
      unsafe { self.visit_type_pack(tail_tp_id) }
    } else {
      null_mut()
    };

    let type_list = AstTypeList {
      types: head,
      tail_type: tail,
    };

    let node = AstTypePackExplicit::new(Location::default(), type_list);
    allocator.alloc(node) as *mut AstTypePack
  }
}
