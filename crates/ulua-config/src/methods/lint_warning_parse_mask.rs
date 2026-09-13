use ulua_ast::records::hot_comment::HotComment;

use crate::{enums::code::Code, records::lint_warning::LintWarning};

// "--!nolint" 指令前缀
const NOLINT: &str = "nolint";

impl LintWarning {
  pub fn parse_mask(hotcomments: &[HotComment]) -> u64 {
    let mut result: u64 = 0;

    for hc in hotcomments {
      if !hc.header {
        continue;
      }

      if !hc.content.starts_with(NOLINT) {
        continue;
      }

      // "--!nolint" 后首个非空白字符
      let name_start = hc.content[NOLINT.len()..].find(|c: char| c != ' ' && c != '\t');

      match name_start {
        // --!nolint 关闭全部警告
        None => return !0u64,
        Some(offset) => {
          // --!nolint 后必须跟空白字符
          if offset == 0 {
            continue;
          }

          // --!nolint name 关闭指定 lint
          let code = Self::parse_name(&hc.content[NOLINT.len() + offset..]);

          if code != Code::Unknown {
            result |= 1u64 << (code as i32);
          }
        }
      }
    }

    result
  }
}
