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
//!
//! 覆盖口径（tw-6 核实）：`cpp/tests/golden/analysis` 共 52 个 golden 条目，
//! 本套件按类别各取 1 例（6/52）做 CLI 形态冒烟；其余条目所对应的 C++
//! TEST_CASE（同源于 `TypeInfer*` / `TypeFunction` / `TypeFunction.user` 套件）
//! 已由 `ulua-unit-test` 的 `type_infer_*` / `type_function` / `type_function_user`
//! 镜像按用例级覆盖，不再重复移植。`cpp/tests/golden/meta`（10 例）是 cpp
//! golden 运行器自身的框架自检，非语言行为回归，不移植。

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

/// cpp `analysis/generics/generic_type_pack`：泛型类型包 `T...` 贯穿形参/返回，
/// 多值解构 `count: number, message: string = forward(42, "hello")` 完全推断、
/// strict 与 nonstrict 均零诊断（钉住「不产生伪报错」）。
#[test]
fn golden_generic_type_pack() {
  let ws = ws("generic-type-pack");
  ws.write(
    "generic_type_pack.luau",
    r#"local function forward<T...>(...: T...): T...
    return ...
end

local count: number, message: string = forward(42, "hello")
assert(count == 42 and message == "hello")
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "generic_type_pack.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "generic_type_pack.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-packs/pack_assignment_mismatch`：多返回包展开到注解局部，
/// 第二个分量注解 boolean 与实际 string 不符，strict 报（5,41）TypeError。
#[test]
fn golden_type_pack_pack_assignment_mismatch() {
  let ws = ws("pack-assign-mismatch");
  ws.write(
    "pack_assignment_mismatch.luau",
    r#"local function values(): (number, string)
    return 42, "hello"
end

local count: number, message: boolean = values()
assert(count == 42 and message)
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "pack_assignment_mismatch.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./pack_assignment_mismatch.luau(5,41): TypeError: Expected this to be 'boolean', but got 'string'\n"
  );

  let (code, stderr) = analyze_nonstrict(&ws, "pack_assignment_mismatch.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-functions/index_basic`：内建类型函数 `index`（含
/// `index<T, keyof<T>>` 展开）取属性类型，错误用例在（14,12）报返回值不匹配。
#[test]
fn golden_type_function_index_basic() {
  let ws = ws("index-basic");
  ws.write(
    "index_basic.luau",
    r#"type MyObject = { a: string, b: number, c: boolean }
type A = index<MyObject, "a">
type All = index<MyObject, keyof<MyObject>>

local function _ok(value: A): string
    return value
end

local function _all(value: All): string | number | boolean
    return value
end

local function _err(value: A): boolean
    return value
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "index_basic.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./index_basic.luau(14,12): TypeError: Expected this to be 'boolean', but got 'string'\n"
  );

  let (code, stderr) = analyze_nonstrict(&ws, "index_basic.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-functions/keyof_invalid_operand`：`keyof` 的操作数含
/// boolean（无键联合），在类型别名定义处（2,23）与函数签名使用处（4,1）
/// 各报一次「does not have keys」，消息内嵌原类型打印 `MyObject | boolean`。
#[test]
fn golden_type_function_keyof_invalid_operand() {
  let ws = ws("keyof-invalid-operand");
  ws.write(
    "keyof_invalid_operand.luau",
    r#"type MyObject = { x: number, y: number, z: number }
type KeysOfMyObject = keyof<MyObject | boolean>

local function _err(idx: KeysOfMyObject): "x" | "y" | "z"
    return idx
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "keyof_invalid_operand.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./keyof_invalid_operand.luau(2,23): TypeError: Type 'MyObject | boolean' does not have keys, so 'keyof<MyObject | boolean>' is invalid\n\
     ./keyof_invalid_operand.luau(4,1): TypeError: Type 'MyObject | boolean' does not have keys, so 'keyof<MyObject | boolean>' is invalid\n"
  );

  let (code, stderr) = analyze_nonstrict(&ws, "keyof_invalid_operand.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-functions/setmetatable_invalid_metatable`：`setmetatable`
/// 的 metatable 实参给了 string，类型函数实例不可栖居，报（1,16）并原样打印
/// 实例 `setmetatable<{  }, string>`（空表打印为 `{  }`，双空格）。
#[test]
fn golden_type_function_setmetatable_invalid_metatable() {
  let ws = ws("setmetatable-invalid-mt");
  ws.write(
    "setmetatable_invalid_metatable.luau",
    r#"type Invalid = setmetatable<{}, string>
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "setmetatable_invalid_metatable.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./setmetatable_invalid_metatable.luau(1,16): TypeError: Type function instance setmetatable<{  }, string> is uninhabited\n"
  );

  let (code, stderr) = analyze_nonstrict(&ws, "setmetatable_invalid_metatable.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-functions/rawget_basic`：内建类型函数 `rawget`（含
/// keyof 展开）取属性类型，错误用例在（14,12）报返回值不匹配（与 index_basic
/// 同构，钉住 rawget 与 index 在同一位置的分歧面为零）。
#[test]
fn golden_type_function_rawget_basic() {
  let ws = ws("rawget-basic");
  ws.write(
    "rawget_basic.luau",
    r#"type MyObject = { a: string, b: number, c: boolean }
type A = rawget<MyObject, "a">
type All = rawget<MyObject, keyof<MyObject>>

local function _ok(value: A): string
    return value
end

local function _all(value: All): string | number | boolean
    return value
end

local function _err(value: A): boolean
    return value
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "rawget_basic.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./rawget_basic.luau(14,12): TypeError: Expected this to be 'boolean', but got 'string'\n"
  );

  let (code, stderr) = analyze_nonstrict(&ws, "rawget_basic.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/user-defined-type-functions/optionify`：用户态 `type function`
/// 遍历属性做 `unionof(property.read, singleton(nil))`，结果表打印为
/// `{ age: number?, alive: boolean?, name: string? }`（可选化 + 按键排序），
/// 错误用例在（18,12）报返回 nil 不匹配。
#[test]
fn golden_user_type_function_optionify() {
  let ws = ws("optionify");
  ws.write(
    "optionify.luau",
    r#"type function optionify(tabletype)
    if not tabletype:is("table") then
        error("Argument is not a table")
    end
    for key, property in tabletype:properties() do
        tabletype:setproperty(key, types.unionof(property.read, types.singleton(nil)))
    end
    return tabletype
end

type Person = {
    name: string,
    age: number,
    alive: boolean,
}

local function _show(value: optionify<Person>): nil
    return value
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "optionify.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./optionify.luau(18,12): TypeError: Expected this to be 'nil', but got '{ age: number?, alive: boolean?, name: string? }'\n"
  );

  let (code, stderr) = analyze_nonstrict(&ws, "optionify.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/user-defined-type-functions/user_error`：用户态 type function
/// 体内 `error(...)` 的消息经 `[string "errors_if_string"]:3:` chunk 前缀回传，
/// 在签名（8,1）与实例化点（8,29）各报一条。
#[test]
fn golden_user_type_function_user_error() {
  let ws = ws("user-error");
  ws.write(
    "user_error.luau",
    r#"type function errors_if_string(argument)
    if argument:is("string") then
        error("We are in a math class! not english")
    end
    return argument
end

local function _show(value: errors_if_string<string>): nil
    return value
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "user_error.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./user_error.luau(8,1): TypeError: 'errors_if_string' type function errored at runtime: [string \"errors_if_string\"]:3: We are in a math class! not english\n\
     ./user_error.luau(8,29): TypeError: 'errors_if_string' type function errored at runtime: [string \"errors_if_string\"]:3: We are in a math class! not english\n"
  );

  let (code, stderr) = analyze_nonstrict(&ws, "user_error.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/union-types/allow_specific_assign`：联合注解局部接受任一成员
/// 赋值（number → string），strict 与 nonstrict 均零诊断（钉住不产生伪报错）。
#[test]
fn golden_union_allow_specific_assign() {
  let ws = ws("allow-specific-assign");
  ws.write(
    "allow_specific_assign.luau",
    r#"local value: number | string = 42
value = "ready"
assert(value == "ready")
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "allow_specific_assign.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "allow_specific_assign.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-instantiations/as_expression_incorrect`：显式类型实参
/// `make<<string>>()` 之后做 `+ 5`，strict 连报两条 __add 不可用（5,1/6,12）
/// 与一条建议（5,1）「Consider annotating the return with number」。
#[test]
fn golden_type_instantiation_as_expression_incorrect() {
  let ws = ws("as-expr-incorrect");
  ws.write(
    "as_expression_incorrect.luau",
    r#"local function make<T>(): T
    return nil :: any
end

local function _add()
    return make<<string>>() + 5
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "as_expression_incorrect.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./as_expression_incorrect.luau(5,1): TypeError: Operator '+' could not be applied to operands of types string and number; there is no corresponding overload for __add\n\
     ./as_expression_incorrect.luau(6,12): TypeError: Operator '+' could not be applied to operands of types string and number; there is no corresponding overload for __add\n\
     ./as_expression_incorrect.luau(5,1): TypeError: Consider annotating the return with number\n"
  );

  let (code, stderr) = analyze_nonstrict(&ws, "as_expression_incorrect.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}
