use crate::{
  enums::kind::Kind,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  /// cpp CodeGen/src/AssemblyBuilderA64.cpp:1852 patchDataRef：
  /// 数据引用在 ±2^18 字内用 ADR，否则降级为 ADRP+ADD 页相对寻址。
  /// `location` 为 ADR 指令所在的指令字序号，`pos` 为数据在 data 段中的字节偏移。
  pub fn patch_data_ref(&mut self, dst: RegisterA64, location: u32, pos: usize) {
    let offset = -(location as i32) - (((self.data.len() - pos) / 4) as i32);

    if offset > -(1 << 18) && offset < (1 << 18) {
      self.place_adr_c_char_register_a_64_u8("adr", dst, 0b10000);
      self.patch_offset(location, offset, Kind::IMM19);
    } else {
      let page_offset = (location * 4) & 0xfff;
      let target_from_page = page_offset as i64 + (offset as i64) * 4;

      self.place_adrp(dst, (target_from_page >> 12) as i32);
      self.add_register_a_64_register_a_64_u16(dst, dst, (target_from_page & 0xfff) as u16);
    }
  }

  /// cpp CodeGen/src/AssemblyBuilderA64.cpp:1677 placeADRP：
  /// 21 位立即数拆分为 immLo（bits 30:29）与 immHi（bits 23:5）
  pub fn place_adrp(&mut self, dst: RegisterA64, page_offset: i32) {
    if self.log_text {
      self.log_c_char_register_a_64_i32_i32("adrp", dst, page_offset, -1);
    }

    if !(-(1 << 20)..(1 << 20)).contains(&page_offset) {
      self.overflowed = true;
      return;
    }

    let imm_lo = (page_offset as u32) & 0x3;
    let imm_hi = ((page_offset as u32) >> 2) & ((1 << 19) - 1);

    self
      .place(dst.index() as u32 | (imm_hi << 5) | (0b10000u32 << 24) | (imm_lo << 29) | (1 << 31));
    self.commit();
  }
}
