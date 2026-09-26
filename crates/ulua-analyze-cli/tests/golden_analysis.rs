//! cpp golden 分析套件（`cpp/tests/golden/`，运行器 `cpp/tools/golden`）的行为级移植。
//!
//! cpp 侧每个 `.luau` 是一个测试，在 `flags-on`/`flags-off` × `strict`/`nonstrict`
//! 矩阵下跑 `luau-analyze --mode=<mode> --solver=new`，期望输出来自相邻的
//! `<name>.flags-<on|off>.<mode>.output` 精确快照。Rust CLI 无 `--fflags` 全量开关，
//! 但下列用例的 flags-on 与 flags-off 快照逐字节相同（已核对），单形态移植无损。
//!
//! 快照中的文件路径是 CLI 实参回显（cpp 从仓库根传 `analysis/.../x.luau`）；
//! 本套件在工作区根传平铺文件名，故期望行的路径前缀相应为 `./<name>.luau`，
//! 行列号与消息正文保持 cpp 快照原文。

use ulua_cli_lib::test_utils::{Workspace, code, stderr_of};

const BIN: &str = env!("CARGO_BIN_EXE_ulua-analyze");

/// 每个用例独占的工作目录（共享夹具，temp 目录以本 crate 名前缀隔离）
fn ws(name: &str) -> Workspace {
  Workspace::new(BIN, "ulua-analyze-cli", name)
}

/// 以 strict 模式跑一份 golden 源，返回 (退出码, stderr)。
/// `--solver=new` 与 cpp golden 矩阵的命令行一致。
fn analyze_strict(ws: &Workspace, file: &str) -> (i32, String) {
  let output = ws.run(&["--mode=strict", "--solver=new", file]);
  (code(&output), stderr_of(&output))
}

/// 以 nonstrict 模式跑一份 golden 源，返回 (退出码, stderr)。
fn analyze_nonstrict(ws: &Workspace, file: &str) -> (i32, String) {
  let output = ws.run(&["--mode=nonstrict", "--solver=new", file]);
  (code(&output), stderr_of(&output))
}

/// cpp `analysis/generics/generic_result_mismatch`：泛型函数实例化后返回值类型
/// 与注解不符，strict 报 TypeError（行列 5,23），nonstrict 放行。
#[test]
fn golden_generic_result_mismatch() {
  let ws = ws("generic-mismatch");
  ws.write(
    "generic_result_mismatch.luau",
    r#"local function identity<T>(value: T): T
    return value
end

local value: number = identity("wrong")
print(value)
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "generic_result_mismatch.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./generic_result_mismatch.luau(5,23): TypeError: Expected this to be 'number', but got 'string'\n"
  );

  let (code, stderr) = analyze_nonstrict(&ws, "generic_result_mismatch.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/intersection-types/no_matching_overload`：交集类型（重载集合）
/// 调用不匹配实参，strict 连报「无兼容重载」+「可用重载清单」两条 TypeError。
#[test]
fn golden_intersection_no_matching_overload() {
  let ws = ws("overload-miss");
  ws.write(
    "no_matching_overload.luau",
    r#"type Overloaded = ((number) -> number) & ((string) -> string)

local function _call(identity: Overloaded)
    return identity(true)
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "no_matching_overload.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./no_matching_overload.luau(4,12): TypeError: None of the overloads for function that accept 1 arguments are compatible.\n\
     ./no_matching_overload.luau(4,12): TypeError: Available overloads: (number) -> number; and (string) -> string\n"
  );

  let (code, stderr) = analyze_nonstrict(&ws, "no_matching_overload.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-functions/index_union_key`：内建类型函数 `index` 的键为
/// 联合单例时，任一键缺席即报 Property 不存在（键原样打印为 `"a" | "d"`）。
#[test]
fn golden_type_function_index_union_key() {
  let ws = ws("index-union-key");
  ws.write(
    "index_union_key.luau",
    r#"type MyObject = { a: string, b: number, c: boolean }
type Present = index<MyObject, "a" | "b">

local function _ok(value: Present): string | number
    return value
end

type PartlyMissing = index<MyObject, "a" | "d">
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "index_union_key.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./index_union_key.luau(8,22): TypeError: Property '\"a\" | \"d\"' does not exist on type 'MyObject'\n"
  );

  let (code, stderr) = analyze_nonstrict(&ws, "index_union_key.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-instantiations/too_many_provided`：显式类型实参个数超过
/// 泛型函数形参上限的报错文本与定位（5,15）。
#[test]
fn golden_type_instantiation_too_many_provided() {
  let ws = ws("too-many-params");
  ws.write(
    "too_many_provided.luau",
    r#"local function identity<T>(value: T): T
    return value
end

local value = identity<<number, string>>(1)
print(value)
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "too_many_provided.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./too_many_provided.luau(5,15): TypeError: Too many type parameters passed to 'identity', which is typed as <T>(T) -> T. Expected at most 1 type parameter, but 2 provided.\n"
  );

  let (code, stderr) = analyze_nonstrict(&ws, "too_many_provided.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/union-types/missing_property`：对「有键|缺键」联合取缺席属性，
/// 报 Key missing 并内联展开联合的两个成员类型。
#[test]
fn golden_union_missing_property() {
  let ws = ws("missing-property");
  ws.write(
    "missing_property.luau",
    r#"type Value = { common: number, name: string } | { common: number }

local function _name(value: Value)
    return value.name
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "missing_property.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./missing_property.luau(4,12): TypeError: Key 'name' is missing from '{ common: number }' in the type '{ common: number } | { common: number, name: string }'\n"
  );

  let (code, stderr) = analyze_nonstrict(&ws, "missing_property.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-states/annotated_assignment_checked`：注解局部变量被赋
/// 不兼容值，strict 报 TypeError（3,13）+ LocalUnused（2,11）；nonstrict 只剩
/// lint（LocalUnused 在两种模式下都独立于类型检查报告）。
#[test]
fn golden_type_state_annotated_assignment_checked() {
  let ws = ws("annotated-assign");
  ws.write(
    "annotated_assignment_checked.luau",
    r#"local function _assign()
    local value: number = 1
    value = "wrong"
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "annotated_assignment_checked.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./annotated_assignment_checked.luau(3,13): TypeError: Expected this to be 'number', but got 'string'\n\
     ./annotated_assignment_checked.luau(2,11): LocalUnused: Variable 'value' is never used; prefix with '_' to silence\n"
  );

  let (code, stderr) = analyze_nonstrict(&ws, "annotated_assignment_checked.luau");
  assert_eq!(code, 0);
  assert_eq!(
    stderr,
    "./annotated_assignment_checked.luau(2,11): LocalUnused: Variable 'value' is never used; prefix with '_' to silence\n"
  );
}
