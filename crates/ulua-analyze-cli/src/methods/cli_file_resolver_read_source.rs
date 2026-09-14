use ulua_analysis::{
  enums::type_file_resolver::Type, records::source_code::SourceCode,
  type_aliases::module_name_type::ModuleName,
};
use ulua_cli_lib::functions::{read_file::read_file, read_stdin::read_stdin};

use crate::records::cli_file_resolver::CliFileResolver;
impl CliFileResolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn read_source(&mut self, name: &ModuleName) -> Option<SourceCode> {
    // '-' 模块名读 stdin, 其余读文件
    let (source, source_type) = if name == "-" {
      (read_stdin(), Type::Script)
    } else {
      (read_file(name), Type::Module)
    };

    source.map(|source_str| SourceCode {
      source: source_str,
      r#type: source_type,
    })
  }
}
