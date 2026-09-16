#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub(crate) struct CallContext {
  pub(crate) newsize: i32,
}
