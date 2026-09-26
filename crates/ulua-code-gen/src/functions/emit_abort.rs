use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label};

pub fn emit_abort(build: &mut AssemblyBuilderA64, abort: &mut Label) {
  let mut skip = Label::default();
  build.b_label(&mut skip);
  build.set_label_label(abort);
  build.udf();
  build.set_label_label(&mut skip);
}
