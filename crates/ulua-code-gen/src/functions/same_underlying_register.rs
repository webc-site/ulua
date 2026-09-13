use crate::{enums::size_x_64::SizeX64, records::register_x_64::RegisterX64};

pub fn same_underlying_register(a: RegisterX64, b: RegisterX64) -> bool {
  let underlying_size_a = if a.size() == SizeX64::Xmmword {
    SizeX64::Xmmword
  } else {
    SizeX64::Qword
  };

  let underlying_size_b = if b.size() == SizeX64::Xmmword {
    SizeX64::Xmmword
  } else {
    SizeX64::Qword
  };

  underlying_size_a == underlying_size_b && a.index() == b.index()
}
