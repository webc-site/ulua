use alloc::string::String;

pub fn rep(s: &str, n: usize) -> String {
  // 标准库 repeat 取代手写计数循环（内部即一次性按 s.len()*n 预留并复制）
  s.repeat(n)
}
