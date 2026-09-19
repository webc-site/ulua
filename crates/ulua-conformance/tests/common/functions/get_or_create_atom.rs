use std::{
  collections::HashMap,
  sync::{Mutex, OnceLock},
};

use crate::common::enums::direct_slot::DirectSlot;

struct DirectAtomState {
  name_to_atom: HashMap<String, i16>,
  atom_to_direct_slot: HashMap<i16, DirectSlot>,
  next_atom_id: i16,
}

impl DirectAtomState {
  fn new() -> Self {
    Self {
      name_to_atom: HashMap::new(),
      atom_to_direct_slot: HashMap::new(),
      next_atom_id: 1,
    }
  }
}

fn name_to_direct_slot(name: &str) -> Option<DirectSlot> {
  match name {
    "X" => Some(DirectSlot::X),
    "Y" => Some(DirectSlot::Y),
    "Magnitude" => Some(DirectSlot::Magnitude),
    "Unit" => Some(DirectSlot::Unit),
    "Dot" => Some(DirectSlot::Dot),
    "Min" => Some(DirectSlot::Min),
    "Clone" => Some(DirectSlot::Clone),
    "Reenter" => Some(DirectSlot::Reenter),
    "pos" => Some(DirectSlot::Pos),
    "normal" => Some(DirectSlot::Normal),
    "uv" => Some(DirectSlot::UV),
    "sizeof" => Some(DirectSlot::Sizeof),
    _ => None,
  }
}

fn direct_atom_state() -> &'static Mutex<DirectAtomState> {
  static STATE: OnceLock<Mutex<DirectAtomState>> = OnceLock::new();
  STATE.get_or_init(|| Mutex::new(DirectAtomState::new()))
}

pub fn reset_direct_atom_state() {
  let mut state = direct_atom_state()
    .lock()
    .expect("direct atom state poisoned");
  *state = DirectAtomState::new();
}

pub fn get_or_create_atom(name: &str) -> i16 {
  let mut state = direct_atom_state()
    .lock()
    .expect("direct atom state poisoned");

  if let Some(atom) = state.name_to_atom.get(name) {
    return *atom;
  }

  let Some(slot) = name_to_direct_slot(name) else {
    return -1;
  };

  let atom = state.next_atom_id;
  state.next_atom_id += 1;
  state.name_to_atom.insert(name.to_owned(), atom);
  state.atom_to_direct_slot.insert(atom, slot);
  atom
}

pub fn direct_slot_for_atom(atom: i32) -> Option<DirectSlot> {
  let state = direct_atom_state()
    .lock()
    .expect("direct atom state poisoned");
  state.atom_to_direct_slot.get(&(atom as i16)).copied()
}
