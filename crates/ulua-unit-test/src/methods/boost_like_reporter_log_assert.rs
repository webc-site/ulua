use crate::records::boost_like_reporter::BoostLikeReporter;
impl BoostLikeReporter {
  pub fn log_assert(
    &mut self,
    ad_failed: bool,
    ad_decomp: &str,
    ad_file: &str,
    ad_line: i32,
    ad_expr: &str,
  ) {
    if !ad_failed {
      return;
    }

    if !ad_decomp.is_empty() {
      eprintln!(
        "{}({}): ERROR: {} ({})",
        ad_file, ad_line, ad_expr, ad_decomp
      );
    } else {
      eprintln!("{}({}): ERROR: {}", ad_file, ad_line, ad_expr);
    }
  }
}
