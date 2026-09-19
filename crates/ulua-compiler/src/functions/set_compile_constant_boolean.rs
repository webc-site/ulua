use crate::{records::constant::Constant, type_aliases::compile_constant::CompileConstant};

pub fn set_compile_constant_boolean(constant: CompileConstant, b: bool) {
  // 单次整体写入，不产生"标签已更新、数据未更新"的中间态
  unsafe { *constant.cast::<Constant>() = Constant::Boolean(b) };
}
