//! C++ 测试基建在注册内置 globals / 挂载测试类型前后的固定收尾
//! （`tests/Fixture.cpp` BuiltinsFixture::getFrontend 序列）：
//! `globals` 与 `globals_for_autocomplete` 必须成对冻结。
use ulua_analysis::{functions::freeze::freeze, records::frontend::Frontend};

pub fn freeze_globals_pair(frontend: &mut Frontend) {
  freeze(frontend.globals.global_types_mut());
  freeze(frontend.globals_for_autocomplete.global_types_mut());
}
