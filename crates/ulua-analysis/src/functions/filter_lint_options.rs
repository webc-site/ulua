use ulua_ast::{enums::mode::Mode, records::hot_comment::HotComment};
use ulua_config::{
  enums::code::Code,
  records::{lint_options::LintOptions, lint_warning::LintWarning},
};

pub fn filter_lint_options(lint_options: &mut LintOptions, hotcomments: &[HotComment], mode: Mode) {
  let ignore_lints = LintWarning::parse_mask(hotcomments);

  lint_options.warning_mask &= !ignore_lints;

  if mode != Mode::NoCheck {
    lint_options.disable_warning(Code::UnknownGlobal);
  }

  if mode == Mode::Strict {
    lint_options.disable_warning(Code::ImplicitReturn);
  }
}
