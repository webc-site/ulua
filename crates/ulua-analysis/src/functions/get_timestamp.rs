pub fn get_timestamp() -> f64 {
  // `wasm32-unknown-unknown` 无时钟后端：原实现依赖的 `std::time::SystemTime::now`
  // 在该目标上会 panic（"time not implemented on this platform"）。此时间戳仅用于
  // 类型检查器的耗时统计（与 `TimeTrace` 的空实现同理），不参与任何正确性
  // 判定，故在 wasm 上返回固定值而不是 panic。
  #[cfg(target_arch = "wasm32")]
  {
    0.0
  }

  // 非 wasm：用 coarsetime 的粗粒度系统时钟取 Unix 纪元以来的秒数（f64），
  // 与原 `SystemTime::now().duration_since(UNIX_EPOCH)` 语义一致，但读取更快。
  #[cfg(not(target_arch = "wasm32"))]
  {
    coarsetime::Clock::now_since_epoch().as_f64()
  }
}
