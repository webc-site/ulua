/// bit32 库的无符号字长：语义固定为 32 位（cpp `unsigned` 在全部受支持平台上
/// 为 32 位），用 u32 而非 c_uint 明确该宽度契约。
pub type BUint = u32;
