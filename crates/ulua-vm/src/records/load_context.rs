use crate::records::{proto::Proto, t_string::tstring, temp_buffer::TempBuffer};

#[derive(Debug)]
pub struct LoadContext<'a> {
  pub(crate) strings: TempBuffer<*mut tstring>,
  pub(crate) protos: TempBuffer<*mut Proto>,
  pub(crate) chunkname: &'a str,
  pub(crate) data: &'a [u8],
  pub(crate) env: i32,
  pub(crate) result: i32,
}
