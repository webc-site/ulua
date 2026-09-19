use ulua_ast::records::location::Location;

use crate::enums::report_format::ReportFormat;

pub fn report(format: ReportFormat, name: &str, loc: &Location, r#type: &str, message: &str) {
  match format {
    ReportFormat::Default => {
      eprintln!(
        "{}({},{}): {}: {}",
        name,
        loc.begin.line + 1,
        loc.begin.column + 1,
        r#type,
        message
      );
    }
    ReportFormat::Luacheck => {
      // Note: luacheck's end column is inclusive but our end column is exclusive
      // In addition, luacheck doesn't support multi-line messages, so if the error is multiline we'll fake end column as 100 and hope for the best
      let column_end = if loc.begin.line == loc.end.line {
        loc.end.column
      } else {
        100
      };

      // Use stdout to match luacheck behavior
      println!(
        "{}:{}:{}-{}: (W0) {}: {}",
        name,
        loc.begin.line + 1,
        loc.begin.column + 1,
        column_end,
        r#type,
        message
      );
    }
    ReportFormat::Gnu => {
      // Note: GNU end column is inclusive but our end column is exclusive
      eprintln!(
        "{}:{}.{}-{}.{}: {}: {}",
        name,
        loc.begin.line + 1,
        loc.begin.column + 1,
        loc.end.line + 1,
        loc.end.column,
        r#type,
        message
      );
    }
  }
}
