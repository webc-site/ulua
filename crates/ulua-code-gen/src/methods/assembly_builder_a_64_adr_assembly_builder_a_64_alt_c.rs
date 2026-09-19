use crate::{
  functions::writef_32::writef_32,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn adr_register_a_64_f32(&mut self, dst: RegisterA64, value: f32) {
    let pos = self.allocate_data(4, 4);
    let location = self.get_code_size();

    unsafe {
      let p = self.data.as_mut_ptr().add(pos);
      writef_32(p, value);
    }

    // cpp AssemblyBuilderA64.cpp:709-723 adr：FarRefs/ProtectData 下走 patchDataRef
    self.patch_data_ref(dst, location, pos);
  }
}
