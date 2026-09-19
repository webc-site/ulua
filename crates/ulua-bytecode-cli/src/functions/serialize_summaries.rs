use std::{
  fs::File,
  io::{self, BufWriter, Write},
};

use ulua_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;

use crate::functions::serialize_script_summary::serialize_script_summary;

/// 序列化所有文件的字节码统计摘要为 JSON 文件
pub fn serialize_summaries(
  files: &[String],
  script_summaries: &[Vec<FunctionBytecodeSummary>],
  summary_file: &str,
) -> bool {
  let clean_path = summary_file.trim_matches('\0');
  let Ok(file) = File::create(clean_path) else {
    eprintln!("Unable to open '{clean_path}'.");
    return false;
  };

  // 逐项写条目；逗号/换行分隔符用 peekable 前瞻，免去下标比较
  let write_entries = |writer: &mut BufWriter<File>| -> io::Result<()> {
    writeln!(writer, "{{")?;
    let mut entries = files.iter().zip(script_summaries.iter()).peekable();
    while let Some((path, summary)) = entries.next() {
      serialize_script_summary(path, summary, writer)?;
      if entries.peek().is_some() {
        writeln!(writer, ",")?;
      } else {
        writeln!(writer)?;
      }
    }
    write!(writer, "}}")
  };

  let mut writer = BufWriter::new(file);
  write_entries(&mut writer).is_ok() && writer.flush().is_ok()
}
