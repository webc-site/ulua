use core::ptr::NonNull;

use crate::{
  enums::polarity::Polarity,
  functions::get_mutable_type_pack::get_mutable_type_pack_id,
  records::{free_type_pack::FreeTypePack, scope::Scope, unifier_2::Unifier2},
  type_aliases::type_pack_id::TypePackId,
};

impl Unifier2 {
  pub fn fresh_type_pack(&mut self, scope: NonNull<Scope>, polarity: Polarity) -> TypePackId {
    let result = unsafe { (*self.arena.as_ptr()).fresh_type_pack(scope.as_ptr(), polarity) };

    // C++ Unifier2.cpp:958 LUAU_ASSERT(ftp)：result 刚由 arena freshTypePack 分配，必为 FreeTypePack
    if let Some(ftp) = get_mutable_type_pack_id::<FreeTypePack>(result) {
      ftp.polarity = polarity;
    }

    self.new_fresh_type_packs.push(result);
    result
  }
}
