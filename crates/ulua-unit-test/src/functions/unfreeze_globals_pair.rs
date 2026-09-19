//! C++ 测试基建在写入 globals / globals_for_autocomplete 之前的固定准备
//! （`tests/Fixture.cpp` BuiltinsFixture::getFrontend 序列）：成对解冻。
use ulua_analysis::{functions::unfreeze::unfreeze, records::frontend::Frontend};

pub fn unfreeze_globals_pair(frontend: &mut Frontend) {
  unfreeze(frontend.globals.global_types_mut());
  unfreeze(frontend.globals_for_autocomplete.global_types_mut());
}
