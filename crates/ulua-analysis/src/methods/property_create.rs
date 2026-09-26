use crate::{records::property_type::Property, type_aliases::type_id::TypeId};

impl Property {
  pub fn create(read: Option<TypeId>, write: Option<TypeId>) -> Self {
    if let Some(read_ty) = read {
      if let Some(write_ty) = write {
        return Property::rw_type_id_type_id(read_ty, write_ty);
      } else {
        return Property::readonly(read_ty);
      }
    }

    if let Some(write_ty) = write {
      return Property::writeonly(write_ty);
    }

    // 紧邻 LUAU_ASSERT 的合取蕴含两侧均为 Some（不可达分支，保留 cpp 断言语义）。
    ulua_common::LUAU_ASSERT!(read.is_some() && write.is_some());
    Property::rw_type_id_type_id(
      read.expect("紧邻 LUAU_ASSERT 合取蕴含 read 为 Some"),
      write.expect("紧邻 LUAU_ASSERT 合取蕴含 write 为 Some"),
    )
  }
}
