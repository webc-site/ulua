use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, label::Label};

impl AssemblyBuilderX64 {
  pub fn place_label(&mut self, label: &mut Label) {
    if label.location == !0u32 {
      if label.id == 0 {
        label.id = self.next_label;
        self.next_label = self.next_label.wrapping_add(1);
        // C++ `labelLocations.push_back(~0u)` — reserve this new label's
        // slot in `label_locations`. The original port wrongly pushed a
        // bogus Label into `pending_labels`, leaving `label_locations`
        // short (out-of-bounds in `set_label`) and corrupting fixups.
        self.label_locations.push(!0u32);
      }

      self.pending_labels.push(Label {
        id: label.id,
        location: self.get_code_size(),
      });
      self.place_imm_32(0);
    } else {
      self.place_imm_32((label.location.wrapping_sub(4 + self.get_code_size())) as i32);
    }
  }
}
