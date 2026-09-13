#[macro_export]
macro_rules! CHECK_REACHING_DEF {
  ($pos:expr, $expected:expr) => {
    CHECK!(get_definition_at_pos(&*cfg, $pos).versioned_name() == $expected);
  };
}
