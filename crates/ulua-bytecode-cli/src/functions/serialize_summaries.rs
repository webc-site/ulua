use std::{
  fs::File,
  io::{self, BufWriter, Write},
};

use ulua_cli_lib::functions::write_json_entries::write_json_entries;
use ulua_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;

use crate::functions::serialize_script_summary::serialize_script_summary;

/// 序列化所有文件的字节码统计摘要为 JSON 文件
pub fn serialize_summaries(
  files: &[String],
  script_summaries: &[Vec<FunctionBytecodeSummary>],
  summary_file: &str,
) -> bool {
  // `--summary-file=` 的参数值不含 NUL，无需裁剪
  let Ok(file) = File::create(summary_file) else {
    eprintln!("Unable to open '{summary_file}'.");
    return false;
  };

  // 条目分隔符统一走 write_json_entries
  let write_entries = |writer: &mut BufWriter<File>| -> io::Result<()> {
    writeln!(writer, "{{")?;
    write_json_entries(
      writer,
      files.iter().zip(script_summaries.iter()),
      |writer, (path, summary)| serialize_script_summary(path, summary, writer),
    )?;
    write!(writer, "}}")
  };

  let mut writer = BufWriter::new(file);
  write_entries(&mut writer).is_ok() && writer.flush().is_ok()
}
