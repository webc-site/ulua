use ulua_common::{enums::luau_bytecode_type::LuauBytecodeType, macros::luau_assert::LUAU_ASSERT};

/// cpp `getBaseTypeString`：bytecode dump 的基础类型名。
/// 判别式与名字的单一真相在 [`LuauBytecodeType::base_name`]（历史上本文件
/// 手抄了一份 `INTEGER=3` 的漂移表，已随收敛一并消除）。
pub(crate) fn get_base_type_string(r#type: u8) -> &'static str {
  // Optional 位不属于类型判别式，剥掉后再查表
  let ty = LuauBytecodeType(r#type as u16 & !LuauBytecodeType::LBC_TYPE_OPTIONAL_BIT.0);
  match ty.base_name() {
    Some(name) => name,
    // 与原表一致：未知判别式断言报错，非断言构建下回退空串（dump 展示路径）
    None => {
      LUAU_ASSERT!(false);
      ""
    }
  }
}
