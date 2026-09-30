/// cpp `lstrlib.cpp`：`#define NB CHAR_BIT`——一个字节的位数。
///
/// 旧版误移植为 `i32::BITS`（32），零引用；按 cpp 原义修正为字节位宽。
pub const NB: i32 = u8::BITS as i32;
