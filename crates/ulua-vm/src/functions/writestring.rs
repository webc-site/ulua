use std::io::{self, Write};

/// 将字节串写到 stdout；写失败静默忽略（cpp `lbaselib.cpp:18-21` 的 `writestring`
/// 即 `fwrite(s, 1, l, stdout)`，**不**逐次 flush，靠 stdio 缓冲）。
///
/// 与之对齐地走 `io::stdout()` 的行缓冲：`print` 的每一句都以 `b"\n"` 收尾，
/// `LineWriter` 见到换行即刷出整行，因此既不留尾数据、也不在每一段（`"\t"`、
/// 各实参串）上多付一次 `write(2)`。原先的显式 `flush()` 使一次 `print(a, b, c)`
/// 产生 5 次系统调用，属 §9「关键路径不得比 cpp 慢」的反例。
///
/// review.md §5 的 `log` 在此**不引入**（有意例外，故写明理由）：本函数是 Lua
/// `print` 的**语言级输出**，文本内容与换行/制表分隔都是脚本可见的语义，必须与
/// cpp 逐字节相同地直达 stdout；`log` 的分级、目标可重定向、格式器可插拔只会
/// 把「程序自己的输出」降级成「宿主日志的一条记录」，还会引入 cpp 没有的级别
/// 过滤。分析侧的诊断走 `Frontend::internalErrorReporter` 与 `set_print_line`
/// 用户回调（同样是显式回调而非全局 logger），全仓没有分级日志的位点。
pub(crate) fn writestring(buf: &[u8]) {
  let mut stdout = io::stdout().lock();
  let _ = stdout.write_all(buf);
}
