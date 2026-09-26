use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{r#type::Type, type_checker::TypeChecker},
  type_aliases::type_id::TypeId,
};

impl TypeChecker {
  pub fn add_type<T>(&mut self, tv: &T) -> TypeId
  where
    T: Clone + Into<Type> + 'static,
  {
    unsafe {
      let module = arc_as_mut(self.expect_current_module());
      (*module).internal_types.add_type(tv.clone())
    }
  }
}
