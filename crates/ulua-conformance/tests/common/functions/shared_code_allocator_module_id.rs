use ulua_code_gen::type_aliases::module_id::ModuleId;

pub fn shared_code_allocator_module_id(first_byte: u8) -> ModuleId {
  let mut module_id = [0; 16];
  module_id[0] = first_byte;
  module_id
}
