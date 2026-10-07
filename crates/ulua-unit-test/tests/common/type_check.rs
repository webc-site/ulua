// 类型推断/前端测试的样板前奏（fixture 构造 → 取前端 → 检查源码 → 交出结果）。
// 各 tests 文件里逐例重复的前奏语句在此收口为宏，展开后与原语句逐字一致，
// 行为不变。宏经 `#[macro_export]` 提升到各集成测试 crate 根，测试文件仅需 `mod common;`
// 即可按裸名调用（不必 `#[macro_use]`，避免未用宏告警）。宏体引用 `Fixture`/
// `BuiltinsFixture` 裸名——macro_rules! 在调用点展开，符号按调用方作用域解析，故各
// 测试文件需自行 use 引入这两个 fixture 类型（其原有构造语句本就在作用域内）。

// `Fixture::fixture_bool(false)` 起非严格 fixture，用默认前端选项检查一段源码。
// 返回 (fixture, result)，调用方以 `let (_fixture, result) = fx_check!(...);` 消费，
// `_fixture` 保活 fixture 至断言结束。仅适用于末尾选项为 None 的默认检查。
#[macro_export]
macro_rules! fx_check {
  ($src:expr) => {{
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options($src, None);
    (fixture, result)
  }};
}

// `BuiltinsFixture::default()` 起带内建类型的 fixture，先 get_frontend 再走 `.base` 检查。
// 语义与 fx_check! 同构，差别仅在 fixture 类型与 `.base` 取用。
#[macro_export]
macro_rules! bs_check {
  ($src:expr) => {{
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture
      .base
      .check_string_optional_frontend_options($src, None);
    (fixture, result)
  }};
}
