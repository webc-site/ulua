use crate::{
  enums::kind::Kind,
  records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label, patch::Patch},
};

impl AssemblyBuilderA64 {
  pub fn patch_label(&mut self, label: &mut Label, kind: Kind) {
    let location = self.get_code_size().wrapping_sub(1);

    if label.location == !0u32 {
      if label.id == 0 {
        label.id = self.next_label;
        self.next_label = self.next_label.wrapping_add(1);
        self.label_locations.push(!0u32);
      }

      let mut patch = Patch {
        kind_and_label: 0,
        location,
      };
      patch.set_kind(kind);
      patch.set_label(label.id);
      self.pending_labels.push(patch);
    } else {
      let value = label.location as i32 - location as i32;
      self.patch_offset(location, value, kind);
    }
  }
}
