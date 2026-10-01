/// 对应 cpp `BytecodeCompiler` 侧的 `BytecodeEncoder` 抽象基类
/// (`virtual void encode(uint32_t* data, size_t count) = 0`), 在
/// `BytecodeBuilder::end_function` 里对单条函数指令流做一次原地变换。
pub(crate) trait BytecodeEncoder {
  fn encode(&mut self, data: &mut [u32]);
}

/// 无变换字节码编码器, 与 cpp `BytecodeEncoder` 基类一致 (原样输出)。
/// 全仓库各处 (`ulua` / `ulua-rt` / `ulua-config` / CLI) 的同名局部定义收敛于此。
#[derive(Default, Debug, Clone, Copy)]
pub struct NoopEncoder;

impl BytecodeEncoder for NoopEncoder {
  #[inline]
  fn encode(&mut self, _data: &mut [u32]) {}
}

/// cpp `BytecodeBuilder` 的 `BytecodeEncoder*` 裸指针在 Rust 端的封闭替身：
/// 指针为空对应 `BytecodeBuilder::encoder == None`，指向某编码器对应
/// `Some(Encoder::..)`。新增一种编码器即加一个变体。
///
/// 取代原先的 `Box<dyn BytecodeEncoder>`（全仓唯一实现是 [`NoopEncoder`]）：
/// 零堆分配、match 静态分发可内联，且让 `BytecodeBuilder` 重新可 `Clone`。
#[derive(Clone, Copy, Debug)]
pub enum Encoder {
  /// 原样输出，即 cpp 传入 `BytecodeEncoder` 基类默认实现的场景。
  Noop(NoopEncoder),
}

impl From<NoopEncoder> for Encoder {
  #[inline]
  fn from(encoder: NoopEncoder) -> Self {
    Self::Noop(encoder)
  }
}

impl Encoder {
  /// 同 [`BytecodeEncoder::encode`]，在封闭集合内静态分发。
  #[inline]
  pub(crate) fn encode(&mut self, data: &mut [u32]) {
    match self {
      Self::Noop(encoder) => encoder.encode(data),
    }
  }
}
