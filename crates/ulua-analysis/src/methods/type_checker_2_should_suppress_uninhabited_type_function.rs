use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{follow_type, get_type},
  records::{type_checker_2::TypeChecker2, type_function_instance_type::TypeFunctionInstanceType},
  type_aliases::type_id::TypeId,
};

impl TypeChecker2 {
  pub fn should_suppress_uninhabited_type_function_error(&mut self, ty: TypeId) -> bool {
    let ty = follow_type::follow(ty);
    let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(ty) else {
      return false;
    };

    // SAFETY: function 是 RefCell 风格 cell 指针，指向 arena 分配的
    // TypeFunctionWorkerName，与类型会话同寿（C++ 直接持有 std::string）。
    let function_name = unsafe { (*tfit.function.as_ptr()).name.as_str() };
    let is_numeric = matches!(
      function_name,
      "add" | "sub" | "mul" | "div" | "idiv" | "pow" | "mod"
    );

    if !is_numeric {
      return false;
    }

    for arg in &tfit.type_arguments {
      let arg = follow_type::follow(*arg);
      let Some(normalized) = self.normalizer.try_normalize(arg) else {
        continue;
      };

      if self
        .normalizer
        .is_inhabited_normalized_type(normalized.as_ref())
        == NormalizationResult::False
      {
        return true;
      }
    }

    false
  }
}
