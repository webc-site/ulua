use ulua_ast::records::hot_comment::HotComment;
use ulua_config::{enums::code::Code, records::lint_warning::LintWarning};

use crate::{
  functions::{emit_warning::emit_warning, fuzzy_match::fuzzy_match},
  records::lint_context::LintContext,
};

pub fn lint_comments(context: &mut LintContext, hotcomments: &[HotComment]) {
  let mut seen_mode = false;

  for hc in hotcomments {
    // We reserve --!<space> for various informational (non-directive) comments
    if hc.content.is_empty()
      || hc.content.as_bytes().first() == Some(&b' ')
      || hc.content.as_bytes().first() == Some(&b'\t')
    {
      continue;
    }

    if !hc.header {
      emit_warning(
        context,
        Code::CommentDirective,
        hc.location,
        format_args!(
          "Comment directive is ignored because it is placed after the first non-comment token"
        ),
      );
    } else {
      let space_pos = hc.content.find([' ', '\t']);

      let first = if let Some(pos) = space_pos {
        &hc.content[..pos]
      } else {
        &hc.content[..]
      };

      if first == "nolint" {
        let notspace_pos = if let Some(pos) = space_pos {
          hc.content[pos..]
            .find(|c| c != ' ' && c != '\t')
            .map(|p| pos + p)
        } else {
          None
        };

        // notspace_pos 为 None 时（无参数或全空白），禁用所有 lints
        if let Some(nsp) = notspace_pos
          && LintWarning::parse_name(&hc.content[nsp..]) == Code::Unknown
        {
          let rule = &hc.content[nsp..];

          // skip Unknown
          if let Some(suggestion) = fuzzy_match(rule, &K_WARNING_NAMES) {
            emit_warning(
              context,
              Code::CommentDirective,
              hc.location,
              format_args!(
                "nolint directive refers to unknown lint rule '{}'; did you mean '{}'?",
                rule, suggestion
              ),
            );
          } else {
            emit_warning(
              context,
              Code::CommentDirective,
              hc.location,
              format_args!("nolint directive refers to unknown lint rule '{}'", rule),
            );
          }
        }
      } else if first == "nocheck" || first == "nonstrict" || first == "strict" {
        if space_pos.is_some() {
          emit_warning(
            context,
            Code::CommentDirective,
            hc.location,
            format_args!(
              "Comment directive with the type checking mode has extra symbols at the end of the line"
            ),
          );
        } else if seen_mode {
          emit_warning(
            context,
            Code::CommentDirective,
            hc.location,
            format_args!("Comment directive with the type checking mode has already been used"),
          );
        } else {
          seen_mode = true;
        }
      } else if first == "optimize" {
        let notspace_pos = if let Some(pos) = space_pos {
          hc.content[pos..]
            .find(|c| c != ' ' && c != '\t')
            .map(|p| pos + p)
        } else {
          None
        };

        if let Some(nsp) = notspace_pos {
          let level = &hc.content[nsp..];

          if level != "0" && level != "1" && level != "2" {
            emit_warning(
              context,
              Code::CommentDirective,
              hc.location,
              format_args!(
                "optimize directive uses unknown optimization level '{}', 0..2 expected",
                level
              ),
            );
          }
        } else {
          emit_warning(
            context,
            Code::CommentDirective,
            hc.location,
            format_args!("optimize directive requires an optimization level"),
          );
        }
      } else if first == "native" {
        if space_pos.is_some() {
          emit_warning(
            context,
            Code::CommentDirective,
            hc.location,
            format_args!("native directive has extra symbols at the end of the line"),
          );
        }
      } else {
        const K_HOT_COMMENTS: [&str; 6] = [
          "nolint",
          "nocheck",
          "nonstrict",
          "strict",
          "optimize",
          "native",
        ];

        if let Some(suggestion) = fuzzy_match(first, &K_HOT_COMMENTS) {
          emit_warning(
            context,
            Code::CommentDirective,
            hc.location,
            format_args!(
              "Unknown comment directive '{}'; did you mean '{}'?",
              first, suggestion
            ),
          );
        } else {
          emit_warning(
            context,
            Code::CommentDirective,
            hc.location,
            format_args!("Unknown comment directive '{}'", first),
          );
        }
      }
    }
  }
}

// kWarningNames array from LinterConfig.h, offset by 1 (skip "Unknown"),
// matching C++ `kWarningNames + 1` with size `Code__Count - 1`.
const K_WARNING_NAMES: [&str; 29] = [
  "UnknownGlobal",
  "DeprecatedGlobal",
  "GlobalUsedAsLocal",
  "LocalShadow",
  "SameLineStatement",
  "MultiLineStatement",
  "LocalUnused",
  "FunctionUnused",
  "ImportUnused",
  "BuiltinGlobalWrite",
  "PlaceholderRead",
  "UnreachableCode",
  "UnknownType",
  "ForRange",
  "UnbalancedAssignment",
  "ImplicitReturn",
  "DuplicateLocal",
  "FormatString",
  "TableLiteral",
  "UninitializedLocal",
  "DuplicateFunction",
  "DeprecatedApi",
  "TableOperations",
  "DuplicateCondition",
  "MisleadingAndOr",
  "CommentDirective",
  "IntegerParsing",
  "ComparisonPrecedence",
  "RedundantNativeAttribute",
];
