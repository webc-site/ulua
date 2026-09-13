use core::{ffi::c_void, slice::from_raw_parts};

use crate::{
  enums::kind::Kind,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn adr_data(&mut self, dst: RegisterA64, data: &[u8]) {
    let size = data.len();
    let pos = self.allocate_data(size, 4);
    let location = self.get_code_size();

    self.data[pos..pos + size].copy_from_slice(data);

    self.place_adr_c_char_register_a_64_u8("adr", dst, 0b10000);

    let data_size = self.data.len();
    let offset = -(location as i32) - (((data_size - pos) / 4) as i32);
    self.patch_offset(location, offset, Kind::IMM19);
  }

  pub fn adr_register_a_64_void_usize(
    &mut self,
    dst: RegisterA64,
    ptr: *const c_void,
    size: usize,
  ) {
    let slice = unsafe { from_raw_parts(ptr as *const u8, size) };
    self.adr_data(dst, slice);
  }
}
