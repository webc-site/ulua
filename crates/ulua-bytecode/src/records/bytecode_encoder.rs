/// 对应 cpp `BytecodeCompiler` 侧的 `BytecodeEncoder` 抽象基类
/// (`virtual void encode(uint32_t* data, size_t count) = 0`), 在
/// `BytecodeBuilder::end_function` 里对单条函数指令流做一次原地变换。
pub trait BytecodeEncoder {
  fn encode(&mut self, data: &mut [u32]);
}

/// 无变换字节码编码器, 与 cpp `BytecodeEncoder` 基类一致 (原样输出)。
/// 全仓库各处 (`ulua` / `ulua-rt` / `ulua-config` / CLI) 的同名局部定义收敛于此。
#[derive(Default)]
pub struct NoopEncoder;

impl BytecodeEncoder for NoopEncoder {
  #[inline]
  fn encode(&mut self, _data: &mut [u32]) {}
}
