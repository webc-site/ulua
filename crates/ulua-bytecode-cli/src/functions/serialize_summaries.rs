use std::{
  fs::File,
  io::{BufWriter, Write},
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
  let file = match File::create(clean_path) {
    Ok(f) => f,
    Err(_) => {
      eprintln!("Unable to open '{clean_path}'.");
      return false;
    }
  };
  let mut writer = BufWriter::new(file);
  let file_count = files.len();

  if writeln!(writer, "{{").is_err() {
    return false;
  }

  for (i, (path, summary)) in files.iter().zip(script_summaries.iter()).enumerate() {
    if serialize_script_summary(path, summary, &mut writer).is_err() {
      return false;
    }
    if i < file_count - 1 {
      if writeln!(writer, ",").is_err() {
        return false;
      }
    } else if writeln!(writer).is_err() {
      return false;
    }
  }

  write!(writer, "}}").is_ok() && writer.flush().is_ok()
}
