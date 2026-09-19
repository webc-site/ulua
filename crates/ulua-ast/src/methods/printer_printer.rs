use crate::{
  records::{printer::Printer, writer::Writer},
  type_aliases::cst_node_map::CstNodeMap,
};

impl<'a, W: Writer> Printer<'a, W> {
  pub fn new(writer: &'a mut W, cst_node_map: &'a CstNodeMap) -> Self {
    Self {
      write_types: false,
      writer,
      cst_node_map,
    }
  }
}
