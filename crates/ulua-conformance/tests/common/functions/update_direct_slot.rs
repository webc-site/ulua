use crate::common::functions::get_or_create_atom::direct_slot_for_atom;

pub(crate) fn update_direct_slot(atom: i32, cachedslot: *mut u16) {
  if let Some(slot) = direct_slot_for_atom(atom) {
    unsafe {
      *cachedslot = slot as u16;
    }
  }
}
