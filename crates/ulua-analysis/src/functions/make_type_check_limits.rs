use ulua_common::functions::get_clock::get_clock;

use crate::records::{frontend_options::FrontendOptions, type_check_limits::TypeCheckLimits};

pub fn make_type_check_limits(options: &FrontendOptions) -> TypeCheckLimits {
  let mut limits = TypeCheckLimits::default();

  if let Some(time_limit) = options.module_time_limit_sec {
    limits.finish_time = Some(get_clock() + time_limit);
  } else {
    limits.finish_time = None;
  }

  limits.cancellation_token = options.cancellation_token.clone();

  limits
}
