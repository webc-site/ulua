use crate::records::to_string_options::ToStringOptions;

pub fn dump_options() -> &'static mut ToStringOptions {
  static mut OPTIONS: Option<ToStringOptions> = None;

  unsafe {
    let ptr = &raw mut OPTIONS;
    if (*ptr).is_none() {
      let mut opts = ToStringOptions::new(true);
      opts.exhaustive = true;
      opts.function_type_arguments = true;
      opts.max_table_length = 0;
      opts.max_type_length = 0;
      *ptr = Some(opts);
    }
    (*ptr).as_mut().unwrap_unchecked()
  }
}
