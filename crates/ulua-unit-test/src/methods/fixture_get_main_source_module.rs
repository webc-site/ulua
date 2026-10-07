use ulua_analysis::{
  records::{arena_handle::Handle, source_module::SourceModule},
  type_aliases::module_name_type::ModuleName,
};

use crate::records::fixture::Fixture;

const MAIN_MODULE_NAME: &str = "MainModule";

impl Fixture {
  /// 主模块 `SourceModule` 的别名句柄（复用 `Frontend::get_source_module_mut`
  /// 步⑤ 的 `Option<Handle>` 形态，取代原 `*mut SourceModule` + null 哨兵）；
  /// 主模块未注册时为 `None`（取代原裸解引用 UB），解引用契约集中在
  /// `records/arena_handle.rs`。
  pub fn get_main_source_module(&mut self) -> Option<Handle<SourceModule>> {
    let main_module_name = ModuleName::from(MAIN_MODULE_NAME);
    self.get_frontend().get_source_module_mut(&main_module_name)
  }

  /// 主模块 `SourceModule` 的共享只读引用：迁移留下的
  /// `get_main_source_module().expect(..).get()` 三步式的收敛门面
  /// （`Handle::get` 生命周期不受约束，物化引用不必依附句柄临时值）。
  pub fn main_source_module(&mut self) -> &SourceModule {
    self
      .get_main_source_module()
      .expect("main source module must exist after check")
      .get()
  }
}
