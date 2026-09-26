//! JSON 整数 `WriteJson` 实现的生成器。
//!
//! `AstJsonEncoder`（AstJsonEncoder.cpp 的 `write` 重载族）与 `JsonEmitter`
//! （JsonEmitter.h 的同名族）两侧各有「一批整数类型 → 原样十进制文本」的同形
//! 实现，原先两文件各抄一份 `write_json_int!`，收口于此。两侧接收者都有
//! `write_raw_string_view`，整数 `to_string` 输出与 `format!("{}", ..)` 逐字节
//! 一致，行为等价。
//!
//! NB: 两调用点都不含 i8/u8——C++ `char` 写成单字符 STRING（AstJsonEncoder.cpp:156），
//! c_char 按目标机解析为 i8 或 u8，故二者不得成为 JSON 整数。

/// 为 `$emitter` 侧的 `WriteJson` trait 批量实现整数类型。
macro_rules! impl_write_json_int {
  ($trait_path:path, $emitter:ty, $($t:ty),+ $(,)?) => {$(
    impl $trait_path for $t {
      fn write_json(&self, enc: &mut $emitter) {
        enc.write_raw_string_view(&self.to_string());
      }
    }
  )+};
}

pub(crate) use impl_write_json_int;
