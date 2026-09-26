use ulua_ast::records::location::Location;

use crate::{
  records::{
    code_too_complex::CodeTooComplex, constraint_generator::ConstraintGenerator,
    type_error::TypeError,
  },
  type_aliases::type_error_data::TypeErrorData,
};
impl ConstraintGenerator {
  pub fn report_code_too_complex(&mut self, location: Location) {
    unsafe {
      let err = TypeErrorData::CodeTooComplex(CodeTooComplex { _unused: None });
      self.errors.push(TypeError {
        location,
        // Safety: 构造期接线不变式（ConstraintGenerator::new 对 module 做 is_some 断言）。
        module_name: self
          .module
          .as_ref()
          .expect("ConstraintGenerator 构造期以 ModulePtr 接线并 LUAU_ASSERT(is_some)，恒为 Some")
          .name
          .clone(),
        data: err.clone(),
      });
      if !self.logger.is_null() {
        // Safety: 紧邻 push 之后取尾，必非空。
        (*self.logger).capture_generation_error(
          self
            .errors
            .last()
            .expect("紧邻上方 push 刚入队，last 必命中（cpp errors.back() 同位）"),
        );
      }
      self.recursion_limit_met = true;
    }
  }
}
