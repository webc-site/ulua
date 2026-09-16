use ulua_ast::{enums::mode::Mode, records::hot_comment::HotComment};

pub fn parse_mode(hotcomments: &[HotComment]) -> Option<Mode> {
  for hc in hotcomments {
    if !hc.header {
      continue;
    }

    if hc.content == "nocheck" {
      return Some(Mode::NoCheck);
    }

    if hc.content == "nonstrict" {
      return Some(Mode::Nonstrict);
    }

    if hc.content == "strict" {
      return Some(Mode::Strict);
    }
  }

  None
}
