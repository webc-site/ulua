//! `instance_collector_2` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::{
  functions::{follow_type, get_type},
  records::{
    extern_type::ExternType, instance_collector_2::InstanceCollector2,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl InstanceCollector2 {
  pub fn cycle(&mut self, ty: TypeId) {
    let t = follow_type::follow(ty);
    let it = get_type::get::<TypeFunctionInstanceType>(t);
    if it.is_some() {
      self.cyclic_instance.insert(t);
    }
  }
}

impl InstanceCollector2 {
  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    it: &TypeFunctionInstanceType,
  ) -> bool {
    self.tys.push_front(ty);
    for t in &it.type_arguments {
      let followed = follow_type::follow(*t);
      self.instance_arguments.insert(followed);
    }
    true
  }

  pub fn visit_type_id_extern_type(&mut self, ty: TypeId, _extern: &ExternType) -> bool {
    let _ = ty;
    false
  }

  pub fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    tp: TypePackId,
    _it: &TypeFunctionInstanceTypePack,
  ) -> bool {
    self.tps.push_front(tp);
    true
  }
}
