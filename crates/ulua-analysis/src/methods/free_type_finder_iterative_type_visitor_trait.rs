use crate::{
  records::{
    extern_type::ExternType,
    free_type::FreeType,
    free_type_finder::FreeTypeFinder,
    function_type::FunctionType,
    iterative_type_visitor::{IterativeTypeVisitor, IterativeTypeVisitorTrait},
    metatable_type::MetatableType,
    table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitorTrait for FreeTypeFinder {
  fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
    &mut self.base
  }

  fn visit_type_id_free_type(&mut self, ty: TypeId, ftv: &FreeType) -> bool {
    FreeTypeFinder::visit_type_id_free_type(self, ty, ftv)
  }

  fn visit_type_id_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
    FreeTypeFinder::visit_type_id_table_type(self, ty, ttv)
  }

  fn visit_type_id_metatable_type(&mut self, ty: TypeId, mtv: &MetatableType) -> bool {
    FreeTypeFinder::visit_type_id_metatable_type(self, ty, mtv)
  }

  fn visit_type_id_function_type(&mut self, ty: TypeId, ftv: &FunctionType) -> bool {
    FreeTypeFinder::visit_type_id_function_type(self, ty, ftv)
  }

  fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
    FreeTypeFinder::visit_type_id_extern_type(self, ty, etv)
  }
}
