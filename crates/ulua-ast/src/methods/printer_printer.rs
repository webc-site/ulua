use crate::{
  records::{printer::Printer, writer::Writer},
  type_aliases::cst_node_map::CstNodeMap,
};

impl<'a> Printer<'a> {
  pub fn new(writer: &'a mut dyn Writer, cst_node_map: CstNodeMap) -> Self {
    Self {
      write_types: false,
      writer,
      cst_node_map,
    }
  }
}

pub fn printer_printer<'a>(writer: &'a mut dyn Writer, cst_node_map: CstNodeMap) -> Printer<'a> {
  Printer::new(writer, cst_node_map)
}
