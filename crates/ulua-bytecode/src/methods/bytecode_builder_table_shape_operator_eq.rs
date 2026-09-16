use crate::records::table_shape::TableShape;

impl TableShape {
  pub fn operator_eq(&self, other: &TableShape) -> bool {
    // Note: FFlag::LuauCompileDuptableConstantPack2 is assumed true in modern Luau bytecode logic
    // as the C++ source provides a branch for it.
    if self.length != other.length {
      return false;
    }

    let len = self.length as usize;
    if self.keys[..len] != other.keys[..len] {
      return false;
    }

    if self.has_constants != other.has_constants {
      return false;
    }

    if self.has_constants && self.constants[..len] != other.constants[..len] {
      return false;
    }

    true
  }
}
