use alloc::string::String;

use crate::{
  functions::generate_name::{MAX_GENERATED_NAME_ATTEMPTS, generate_name},
  records::stringifier_state::StringifierState,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl StringifierState {
  /// C++ `std::string getName(TypeId ty)`.
  pub fn get_name_type_id(&mut self, ty: TypeId) -> String {
    unsafe {
      // Safety: self.opts 为构造期接线指向外部 ToStringOptions 的 *mut（并非 self 的字段、
      // 二者不重叠），比本次调用长寿且本线程独占；重建 &mut 供 name_map 写入，同时只读
      // self.used_names 属不同对象，无并存别名冲突。
      let opts = &mut *self.opts;
      let s = opts.name_map.types.size();
      // std::string& n = opts.nameMap.types[ty]; (default-constructs)
      {
        let n = opts.name_map.types.get_or_insert(ty);
        if !n.is_empty() {
          return n.clone();
        }
      }

      // 试名改迭代器链：map 产候选、find 短路命中，与原 early-return 循环逐位等价。
      if let Some(candidate) = (0..MAX_GENERATED_NAME_ATTEMPTS)
        .map(|count| generate_name(self.used_names.size() + count))
        .find(|candidate| !self.used_names.contains_str(candidate.as_str()))
      {
        self.used_names.insert(candidate.clone());
        *opts.name_map.types.get_or_insert(ty) = candidate.clone();
        return candidate;
      }

      generate_name(s)
    }
  }

  /// C++ `std::string getName(TypePackId ty)`.
  pub fn get_name_type_pack_id(&mut self, ty: TypePackId) -> String {
    unsafe {
      // Safety: 同上——self.opts 指向外部 ToStringOptions 的 *mut（非 self 字段、不重叠），
      // 本线程独占；重建 &mut 写 name_map.type_packs，与 self.previous_name_index/used_names
      // 的读写分属不同对象，无并存别名。
      let opts = &mut *self.opts;
      let s = opts.name_map.type_packs.size();
      {
        let n = opts.name_map.type_packs.get_or_insert(ty);
        if !n.is_empty() {
          return n.clone();
        }
      }

      // 试名改迭代器链；count 随候选一并带出，用于回写 previous_name_index 水位。
      let base = self.previous_name_index as usize;
      if let Some((count, candidate)) = (0..MAX_GENERATED_NAME_ATTEMPTS)
        .map(|count| (count, generate_name(base + count)))
        .find(|(_, candidate)| !self.used_names.contains_str(candidate.as_str()))
      {
        self.previous_name_index += count as i32;
        self.used_names.insert(candidate.clone());
        *opts.name_map.type_packs.get_or_insert(ty) = candidate.clone();
        return candidate;
      }

      generate_name(s)
    }
  }
}
