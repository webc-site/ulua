/// cpp 各 CLI 带值文件名选项（`--summary-file=` / `--stats-file=`）缺省值参数时
/// 共用的一行错误文案：消息原文以 `\n\n` 结尾（cpp `printf` 语义），`eprintln!`
/// 自带一个换行，故字面量只再补一个 `\n`。选项名由调用方给出，收口此一处以免
/// bytecode/compile 两 CLI 各抄同一字符串（review.md §3「雷同字符串提为常量/单点」）。
pub fn report_missing_filename_option(option: &str) {
  eprintln!("Error: filename missing for '{option}'.\n");
}
