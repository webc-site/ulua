use crate::records::{
  type_function_instance_type::TypeFunctionInstanceType,
  type_function_reduction_guesser::TypeFunctionReductionGuesser,
};

impl TypeFunctionReductionGuesser {
  pub fn is_numeric_binop_function(&self, instance: &TypeFunctionInstanceType) -> bool {
    let func = unsafe { &*instance.function.as_ptr() };
    // 单值 matches!（字节 DFA），替代 7 次顺序字符串比较；null 名 as_bytes 为
    // 空切片，与旧比较链一致地落 false
    matches!(
      func.name.as_bytes(),
      b"add" | b"sub" | b"mul" | b"div" | b"idiv" | b"pow" | b"mod"
    )
  }
}
