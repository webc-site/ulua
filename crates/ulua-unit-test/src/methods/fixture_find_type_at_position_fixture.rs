use ulua_analysis::{
  functions::find_type_at_position::find_type_at_position,
  records::module::Module,
  type_aliases::{module_name_type::ModuleName, type_id::TypeId},
};
use ulua_ast::records::position::Position;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn find_type_at_position_position(&mut self, position: Position) -> Option<TypeId> {
    let module: *mut Module = self.get_main_module(false);
    let source_module = self
      .get_main_source_module()
      .expect("findTypeAtPosition: main source module must exist after check");
    // Safety: module 为主模块注册产物（check 完成后 resolver 容器保有的存活
    // Module，&* 物化只读借用透传 cpp 同形查询，借用帧内结束）；
    // source_module 为 Handle 交付的共享只读借用（契约见 arena_handle 模块）。
    unsafe { find_type_at_position(&*module, source_module.get(), position) }
  }
}

impl Fixture {
  pub fn find_type_at_position_module_name_position(
    &mut self,
    module_name: &str,
    position: Position,
  ) -> Option<TypeId> {
    if module_name.is_empty() {
      return self.find_type_at_position_position(position);
    }

    let module_name = ModuleName::from(module_name);
    let frontend = self.get_frontend();
    let module = frontend.module_resolver.get_module(&module_name);
    let Some(source_module) = frontend.get_source_module(&module_name) else {
      panic!("findTypeAtPosition: No source module \"{}\"", module_name);
    };

    // module 为 resolver 返回的模块句柄，&module 直接借用（Arc/容器保活）；
    // source_module 为 Handle 交付的共享只读借用（契约见 arena_handle 模块），
    // find_type_at_position 只读查询，全程免 unsafe。
    find_type_at_position(&module, source_module.get(), position)
  }
}
