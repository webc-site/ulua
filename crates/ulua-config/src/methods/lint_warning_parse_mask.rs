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

      // starts_with + 定长切片两步合一：strip_prefix 只在命中时给出后缀切片
      let Some(rest) = hc.content.strip_prefix(NOLINT) else {
        continue;
      };

      // "--!nolint" 之后的首个非空白字符偏移
      match rest.find(|c: char| c != ' ' && c != '\t') {
        // --!nolint 关闭全部警告
        None => return !0u64,
        // --!nolint 后必须跟空白字符，紧贴则视为无效指令
        Some(0) => continue,
        // --!nolint name 关闭指定 lint
        Some(offset) => {
          let code = Self::parse_name(&rest[offset..]);

          if code != Code::Unknown {
            result |= code.mask_bit();
          }
        }
      }
    }

    result
  }
}
