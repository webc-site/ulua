//! cpp golden 分析套件（`cpp/tests/golden/`，运行器 `cpp/tools/golden`）的行为级移植。
//!
//! cpp 侧每个 `.luau` 是一个测试，在 `flags-on`/`flags-off` × `strict`/`nonstrict`
//! 矩阵下跑 `luau-analyze --mode=<mode> --solver=new`，期望输出来自相邻的
//! `<name>.flags-<on|off>.<mode>.output` 精确快照。Rust CLI 无 `--fflags` 全量开关，
//! 各用例的形态选择（on 全等 / off 形态 / `--!golden ok` 指令）见下方覆盖口径逐类说明。
//!
//! 快照中的文件路径是 CLI 实参回显（cpp 从仓库根传 `analysis/.../x.luau`）；
//! 本套件在工作区根传平铺文件名，故期望行的路径前缀相应为 `./<name>.luau`，
//! 行列号与消息正文保持 cpp 快照原文。
//!
//! 覆盖口径（w6 复核，替换 tw-6 误报旧口径）：`cpp/tests/golden/analysis` 共
//! 52 个 golden 条目。旧文档自述「按类别各取 1 例（6/52），其余由
//! `ulua-unit-test` 的 `type_infer_*` / `type_function*` 镜像按用例级覆盖」——
//! 实测两头不成立：上一提交时套件实有 16 例；对其余 36 条按「夹具源码 ==
//! cpp TEST_CASE 内嵌源码」机检，仅 `keyof_basic` 可对上
//! `type_function_keyof_type_function_works` 的部分用例级对位，其余 35 条无
//! 用例级镜像（`ulua-unit-test` 是 API 级用例族，源码多有改写，不构成 golden
//! 逐字节对位）。本批补齐 31 例 CLI 冒烟，套件至 47/52：
//! - 23 例四快照（flags-on/flags-off × strict/nonstrict）逐字节全等，单形态移植；
//! - 7 例按 cpp `flags-off` 快照形态移植（`index_unknown_refined_with_type`、
//!   `index_unknown_refined_with_typeof`、`disallow_less_specific_assign`、
//!   `error_handling_pcall`、`intersection_methods`、`optional_type`、
//!   `union_methods`）：Rust CLI 默认旗标复现 cpp flags-off 渲染；flags-on
//!   全开旗标后的新式紧凑子类型原因渲染（`LuauNewTypePathErrorMessages` 一族，
//!   未同步）不可在本套件表达，另见 `type_function_user.rs` 尾部缺口台账；
//! - 1 例（`type-states/initialize_optional_with_nil`）夹具仅带 `--!golden ok`
//!   指令、无快照：按 cpp golden 运行器语义（`expectations.py`：`ok` 要求全部
//!   命令 returncode 0）断言双模式退出码 0 + stderr 空。
//!
//! 其余 5 例未移植：`keyof_basic`、`keyof_metatable`、`keyof_union_common_keys`、
//! `rawkeyof_ignores_metatable`、`negation`——Rust CLI 默认输出与 cpp 现存任一
//! 快照形态（on/off）都逐字节不同（union 子类型失败原因的措辞代际差异；
//! `keyof_metatable` 连分量序号、`negation` 连被打印类型都不同），禁为凑绿
//! 放宽断言，登记为渲染/排序面缺口。
//!
//! `cpp/tests/golden/meta`（10 例）是 cpp golden 运行器自身的框架自检，非语言
//! 行为回归，不移植。

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

/// 以 strict 模式 + `--fflags=true` 跑一份 golden 源，返回 (退出码, stderr)。
/// 对应 cpp golden 矩阵的 `flags-on` 快照生成命令行（`--fflags=true` 点亮全部
/// `Luau*` 旗标，含 `LuauNewTypePathErrorMessages` → `renderTypePath` 紧凑子类型
/// 原因渲染）。Rust 默认裸跑刻意保持该旗标关（见 `is_default_enabled_flag`），
/// 故 flags-on 形态必须显式传本开关。
fn analyze_strict_flags_on(ws: &Workspace, file: &str) -> (i32, String) {
  let output = ws.run(&[
    "--mode=strict",
    "--solver=new",
    "--fflags=true",
    file,
  ]);
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

/// cpp `analysis/generics/generic_identity`（flags-on/flags-off 快照全等，单形态移植）：泛型恒等函数分别以 number/string 实例化并与注解匹配，双模式零诊断。
#[test]
fn golden_generic_identity() {
  let ws = ws("generic-identity");
  ws.write(
    "generic_identity.luau",
    r#"local function identity<T>(value: T): T
    return value
end

local numberValue: number = identity(42)
local stringValue: string = identity("hello")
assert(numberValue == 42 and stringValue == "hello")
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "generic_identity.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "generic_identity.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/intersection-types/overload_resolution`（flags-on/flags-off 快照全等，单形态移植）：交类型 `(number) -> number & (string) -> string` 的重载解析各中其臂，双模式零诊断。
#[test]
fn golden_intersection_overload_resolution() {
  let ws = ws("overload-resolution");
  ws.write(
    "overload_resolution.luau",
    r#"type Overloaded = ((number) -> number) & ((string) -> string)
local identity = (function(value: any): any
    return value
end) :: Overloaded

local numberValue: number = identity(42)
local stringValue: string = identity("hello")
assert(numberValue == 42 and stringValue == "hello")
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "overload_resolution.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "overload_resolution.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/intersection-types/property_guaranteed_to_exist`（flags-on/flags-off 快照全等，单形态移植）：交表 `{ name: string } & { value: number }` 属性存在性保证，直接访问通过。
#[test]
fn golden_intersection_property_guaranteed_to_exist() {
  let ws = ws("property-guaranteed-to-exist");
  ws.write(
    "property_guaranteed_to_exist.luau",
    r#"type Named = { name: string } & { value: number }

local function describe(item: Named): string
    return item.name .. tostring(item.value)
end

assert(describe({ name = "item", value = 1 }) == "item1")
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "property_guaranteed_to_exist.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "property_guaranteed_to_exist.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/refinements/index_unknown_refined_with_type`（flags-off 形态移植：Rust CLI 默认旗标复现 cpp flags-off 渲染）：`unknown` 经 `type(object) ~= "table"` 细化后下标，strict 在（6,12）报 Expected type table, got 'table' instead。
#[test]
fn golden_refinement_index_unknown_refined_with_type() {
  let ws = ws("index-unknown-refined-with-type");
  ws.write(
    "index_unknown_refined_with_type.luau",
    r#"local function readFirst(object: unknown)
    if type(object) ~= "table" then
        return nil
    end

    return object[1]
end

return readFirst
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "index_unknown_refined_with_type.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./index_unknown_refined_with_type.luau(6,12): TypeError: Expected type table, got 'table' instead\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "index_unknown_refined_with_type.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/refinements/index_unknown_refined_with_typeof`（flags-off 形态移植：Rust CLI 默认旗标复现 cpp flags-off 渲染）：`unknown` 经 `typeof(object) ~= "table"` 细化后下标，strict 在（6,12）报 Expected type table, got 'table' instead。
#[test]
fn golden_refinement_index_unknown_refined_with_typeof() {
  let ws = ws("index-unknown-refined-with-typeof");
  ws.write(
    "index_unknown_refined_with_typeof.luau",
    r#"local function readFirst(object: unknown)
    if typeof(object) ~= "table" then
        return nil
    end

    return object[1]
end

return readFirst
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "index_unknown_refined_with_typeof.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./index_unknown_refined_with_typeof.luau(6,12): TypeError: Expected type table, got 'table' instead\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "index_unknown_refined_with_typeof.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-functions/getmetatable_protected`（flags-on/flags-off 快照全等，单形态移植）：`getmetatable` 遇 `__metatable = "protected"` 保护元表，返回保护值字面量类型，与 string 注解匹配。
#[test]
fn golden_type_function_getmetatable_protected() {
  let ws = ws("getmetatable-protected");
  ws.write(
    "getmetatable_protected.luau",
    r#"local metatable = { __metatable = "protected" }
local value = setmetatable({ x = 1 }, metatable)
type ProtectedMetatable = getmetatable<typeof(value)>

local function _ok(result: ProtectedMetatable): string
    return result
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "getmetatable_protected.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "getmetatable_protected.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-functions/index_array`（flags-on/flags-off 快照全等，单形态移植）：`index<typeof(values), number>` 取数组元素联合类型，与 string | number | boolean 注解匹配。
#[test]
fn golden_type_function_index_array() {
  let ws = ws("index-array");
  ws.write(
    "index_array.luau",
    r#"local values = { "hello", 1, true }
type Element = index<typeof(values), number>

local function _ok(value: Element): string | number | boolean
    return value
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "index_array.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "index_array.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-functions/index_invalid_key`（flags-on/flags-off 快照全等，单形态移植）：`index` 缺键 "d" 与非法键 boolean 各报一条 TypeError（2,16）/（3,16）。
#[test]
fn golden_type_function_index_invalid_key() {
  let ws = ws("index-invalid-key");
  ws.write(
    "index_invalid_key.luau",
    r#"type MyObject = { a: string, b: number, c: boolean }
type Missing = index<MyObject, "d">
type Invalid = index<MyObject, boolean>
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "index_invalid_key.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./index_invalid_key.luau(2,16): TypeError: Property '\"d\"' does not exist on type 'MyObject'\n\
     ./index_invalid_key.luau(3,16): TypeError: Property 'boolean' does not exist on type 'MyObject'\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "index_invalid_key.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-functions/index_metatable`（flags-on/flags-off 快照全等，单形态移植）：`index` 沿 `__index` 元表链读属性，缺键 "Car" 在（21,16）报 TypeError。
#[test]
fn golden_type_function_index_metatable() {
  let ws = ws("index-metatable");
  ws.write(
    "index_metatable.luau",
    r#"local base = { Foo = "text", Bar = true }
local first = setmetatable({ Foo = 8 }, { __index = base })
local second = setmetatable({ Bar = 5 }, { __index = base })

type FirstFoo = index<typeof(first), "Foo">
type SecondFoo = index<typeof(second), "Foo">
type SecondValues = index<typeof(second), "Foo" | "Bar">

local function _first(value: FirstFoo): number
    return value
end

local function _second(value: SecondFoo): string
    return value
end

local function _values(value: SecondValues): string | number
    return value
end

type Missing = index<typeof(first), "Car">
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "index_metatable.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./index_metatable.luau(21,16): TypeError: Property '\"Car\"' does not exist on type 'first'\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "index_metatable.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-functions/index_union_table`（flags-on/flags-off 快照全等，单形态移植）：联合表公共键 `index<First | Second, "a">` 可取，"b" 缺于 Second 在（9,26）报 TypeError。
#[test]
fn golden_type_function_index_union_table() {
  let ws = ws("index-union-table");
  ws.write(
    "index_union_table.luau",
    r#"type First = { a: string, b: number, c: boolean }
type Second = { a: number }
type A = index<First | Second, "a">

local function _ok(value: A): string | number
    return value
end

type MissingFromSecond = index<First | Second, "b">
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "index_union_table.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./index_union_table.luau(9,26): TypeError: Property '\"b\"' does not exist on type 'First | Second'\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "index_union_table.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-functions/rawget_missing_key`（flags-on/flags-off 快照全等，单形态移植）：`rawget` 缺失键 "b" 解析为 nil 类型（缺键不报错），返回值 nil 注解匹配，双模式零诊断。
#[test]
fn golden_type_function_rawget_missing_key() {
  let ws = ws("rawget-missing-key");
  ws.write(
    "rawget_missing_key.luau",
    r#"type MyObject = { a: string }
type Missing = rawget<MyObject, "b">

local function _ok(value: Missing): nil
    return value
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "rawget_missing_key.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "rawget_missing_key.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-functions/setmetatable_invalid_base`（flags-on/flags-off 快照全等，单形态移植）：`setmetatable<string, {}>` 基类型非表，（1,16）报类型函数实例不可栖居。
#[test]
fn golden_type_function_setmetatable_invalid_base() {
  let ws = ws("setmetatable-invalid-base");
  ws.write(
    "setmetatable_invalid_base.luau",
    r#"type Invalid = setmetatable<string, {}>
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "setmetatable_invalid_base.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./setmetatable_invalid_base.luau(1,16): TypeError: Type function instance setmetatable<string, {  }> is uninhabited\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "setmetatable_invalid_base.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-instantiations/as_expression_correct`（flags-on/flags-off 快照全等，单形态移植）：`make<<number>>() + 5` 显式实例化后算术通过，双模式零诊断。
#[test]
fn golden_type_instantiation_as_expression_correct() {
  let ws = ws("as-expression-correct");
  ws.write(
    "as_expression_correct.luau",
    r#"local function make<T>(): T
    return 0 :: any
end

local value: number = make<<number>>() + 5
assert(value == 5)
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "as_expression_correct.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "as_expression_correct.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-packs/infer_multi_return`（flags-on/flags-off 快照全等，单形态移植）：多返回 `values()` 推断类型包，解构注解 number/string 匹配，零诊断。
#[test]
fn golden_type_pack_infer_multi_return() {
  let ws = ws("infer-multi-return");
  ws.write(
    "infer_multi_return.luau",
    r#"local function values()
    return 42, "hello"
end

local count: number, message: string = values()
assert(count == 42 and message == "hello")
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "infer_multi_return.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "infer_multi_return.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-packs/variadic_pack_syntax`（flags-on/flags-off 快照全等，单形态移植）：`T...` 可变包转发 `forward(1, true)`，解构注解 number/boolean 匹配，零诊断。
#[test]
fn golden_type_pack_variadic_pack_syntax() {
  let ws = ws("variadic-pack-syntax");
  ws.write(
    "variadic_pack_syntax.luau",
    r#"local function forward<T...>(...: T...): T...
    return ...
end

local first: number, second: boolean = forward(1, true)
assert(first == 1 and second)
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "variadic_pack_syntax.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "variadic_pack_syntax.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-states/compound_assignment`（flags-on/flags-off 快照全等，单形态移植）：`value += 7` 复合赋值保持 number 类型态，零诊断。
#[test]
fn golden_type_state_compound_assignment() {
  let ws = ws("compound-assignment");
  ws.write(
    "compound_assignment.luau",
    r#"local value = 5
value += 7

assert(value == 12)
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "compound_assignment.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "compound_assignment.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/type-states/initialize_optional_with_nil`（`--!golden ok` 指令形态，无快照）：
/// fixture 首行为 golden 运行器指令（对分析器是未知注释指令）。cpp golden 树自带
/// `tests/golden/.luaurc` 关闭 `CommentDirective` lint，本例逐字节复刻该配置以对齐
/// oracle 环境；随后 `string?` 以 nil 初始化再赋字符串，双模式退出码 0 + stderr 空
/// （cpp `expectations.py`：`ok` 要求全部命令 returncode 0）。
#[test]
fn golden_type_state_initialize_optional_with_nil() {
  let ws = ws("initialize-optional-with-nil");
  // 逐字节复制自 `cpp/tests/golden/.luaurc`（golden 树环境夹具）
  ws.write(
    ".luaurc",
    r#"{
    "lint": {
        "CommentDirective": false
    }
}"#,
  );
  ws.write(
    "initialize_optional_with_nil.luau",
    r#"--!golden ok

local value: string? = nil
value = "ready"
assert(value == "ready")
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "initialize_optional_with_nil.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "initialize_optional_with_nil.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/union-types/disallow_less_specific_assign`：联合参数 `value: number | string`
/// 赋给 `number` 注解，strict（2,33）报联合分量不是 number 子类型。默认（无 `--fflags`）
/// 复现 cpp `flags-off` verbose 渲染（"the 2nd component of the union is ..."）；
/// `--fflags=true`（点亮 `LuauNewTypePathErrorMessages`）复现 cpp `flags-on` 紧凑渲染
/// （分量子路径 `renderTypePath` 出空前缀 ⇒ 落回 `baseReason`），二者逐字节对位。
#[test]
fn golden_union_disallow_less_specific_assign() {
  let ws = ws("disallow-less-specific-assign");
  ws.write(
    "disallow_less_specific_assign.luau",
    r#"local function _assign(value: number | string)
    local numberValue: number = value
    return numberValue
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "disallow_less_specific_assign.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./disallow_less_specific_assign.luau(2,33): TypeError: Expected this to be 'number', but got 'number | string'; \n\
     the 2nd component of the union is `string`, which is not a subtype of `number`\n",
  );

  let (code, stderr) = analyze_strict_flags_on(&ws, "disallow_less_specific_assign.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./disallow_less_specific_assign.luau(2,33): TypeError: Expected this to be 'number', but got 'number | string'; \n\
     `string` is not a subtype of `number`\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "disallow_less_specific_assign.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/user-defined-type-functions/calls_other_type_function`（flags-on/flags-off 快照全等，单形态移植）：UDTF `second` 内调 `first()` 得字面量 "hi"，（10,12）报与 nil 注解不匹配。
#[test]
fn golden_user_type_function_calls_other_type_function() {
  let ws = ws("calls-other-type-function");
  ws.write(
    "calls_other_type_function.luau",
    r#"type function first()
    return "hi"
end

type function second()
    return types.singleton(first())
end

local function _show(value: second<>): nil
    return value
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "calls_other_type_function.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./calls_other_type_function.luau(10,12): TypeError: Expected this to be 'nil', but got '\"hi\"'\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "calls_other_type_function.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/user-defined-type-functions/error_handling_pcall`（flags-off 形态移植：Rust CLI 默认旗标复现 cpp flags-off 渲染）：UDTF 体内 `pcall`/`xpcall` 不受支持：strict 报（10,16）/（11,16）unknown global、（17,14）type function 运行期 error、（17,32）never 分量不匹配。
#[test]
fn golden_user_type_function_error_handling_pcall() {
  let ws = ws("error-handling-pcall");
  ws.write(
    "error_handling_pcall.luau",
    r#"type function optionalize(value: type)
    if value:is("nil") then
        return value
    else
        error("oh no")
    end
end

type function recover(value)
    assert(not pcall(optionalize, value))
    assert(not xpcall(optionalize, function(problem)
        return problem
    end, value))
    return types.unionof(value, types.singleton(nil))
end

local value: recover<number> = 5
assert(value == 5)
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "error_handling_pcall.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./error_handling_pcall.luau(10,16): TypeError: Unknown global 'pcall'; consider assigning to it first\n\
     ./error_handling_pcall.luau(11,16): TypeError: Unknown global 'xpcall'; consider assigning to it first\n\
     ./error_handling_pcall.luau(17,14): TypeError: 'recover' type function errored at runtime: [string \"recover\"]:10: this function is not supported in type functions\n\
     ./error_handling_pcall.luau(17,32): TypeError: Expected this to be 'recover<number>', but got 'number'; \n\
     it reduces to `never`, and `number` is not a subtype of `never`\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "error_handling_pcall.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/user-defined-type-functions/function_methods`（flags-on/flags-off 快照全等，单形态移植）：`types.newfunction` 设参/返参后 `:parameters().head` 遍历重建函数类型，_show 注 never 报（19,12）unreachable。
#[test]
fn golden_user_type_function_function_methods() {
  let ws = ws("function-methods");
  ws.write(
    "function_methods.luau",
    r#"type function getfunction()
    local value = types.newfunction(nil, nil)
    value:setparameters({ types.string, types.number }, nil)
    value:setreturns(nil, types.boolean)
    if value:is("function") then
        local parameters: { type } = {}
        local head = value:parameters().head
        if head then
            for _, parameter in head do
                table.insert(parameters, parameter)
            end
        end
        return types.newfunction({ head = parameters }, value:returns())
    end
    return types.number
end

local function _show(value: getfunction<>): never
    return value
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "function_methods.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./function_methods.luau(19,12): TypeError: Expected this to be unreachable, but got\n\
     \t'(string, number) -> (...boolean)'\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "function_methods.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/user-defined-type-functions/intersection_methods`：`types.intersectionof`
/// 分量遍历重建交集，_show 注 never 报（22,12）unreachable。默认复现 cpp `flags-off`
/// verbose 渲染（"the Nth component of the intersection is ..."）；`--fflags=true`
/// 复现 cpp `flags-on` 紧凑渲染（交分量子路径出空前缀 ⇒ 各 `baseReason`）。交集分量
/// 枚举序在 on/off 两侧一致（区别于 union 排序代际缺口），故两形态皆逐字节对位。
#[test]
fn golden_user_type_function_intersection_methods() {
  let ws = ws("intersection-methods");
  ws.write(
    "intersection_methods.luau",
    r#"type function getintersection()
    local first = types.newtable(nil, nil, nil)
    first:setproperty(types.singleton("boolean"), types.boolean)
    first:setproperty(types.singleton("number"), types.number)

    local second = types.newtable(nil, nil, nil)
    second:setproperty(types.singleton("boolean"), types.boolean)
    second:setproperty(types.singleton("string"), types.string)

    local value = types.intersectionof(first, second)
    if value:is("intersection") then
        local components = {}
        for _, component in value:components() do
            table.insert(components, component)
        end
        return types.intersectionof(table.unpack(components))
    end
    return types.string
end

local function _show(value: getintersection<>): never
    return value
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "intersection_methods.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./intersection_methods.luau(22,12): TypeError: Expected this to be unreachable, but got\n\
     \t'{ boolean: boolean, number: number } & { boolean: boolean, string: string }'; \n\
     this is because \n\
     \t * the 1st component of the intersection is `{ boolean: boolean, number: number }`, which is not a subtype of `never`\n\
     \t * the 2nd component of the intersection is `{ boolean: boolean, string: string }`, which is not a subtype of `never`\n",
  );

  let (code, stderr) = analyze_strict_flags_on(&ws, "intersection_methods.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./intersection_methods.luau(22,12): TypeError: Expected this to be unreachable, but got\n\
     \t'{ boolean: boolean, number: number } & { boolean: boolean, string: string }'; \n\
     this is because \n\
     \t * `{ boolean: boolean, number: number }` is not a subtype of `never`\n\
     \t * `{ boolean: boolean, string: string }` is not a subtype of `never`\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "intersection_methods.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/user-defined-type-functions/issubtypeof_errors`（flags-on/flags-off 快照全等，单形态移植）：`issubtypeof` 为假时 UDTF `error("Not a subtype!")`：三处实例化 TypeError + 五个 LocalUnused 诊断。
#[test]
fn golden_user_type_function_issubtypeof_errors() {
  let ws = ws("issubtypeof-errors");
  ws.write(
    "issubtypeof_errors.luau",
    r#"type function checksubtype(subtype, supertype)
    if not subtype:issubtypeof(supertype) then
        error("Not a subtype!")
    end
    return subtype
end

local validnil: checksubtype<nil, string?>
local validstring: checksubtype<"Hello", string>
local invalidunion: checksubtype<string | vector | number, number>
local invalidboolean: checksubtype<boolean, number>
local invalidsingleton: checksubtype<false, nil>
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "issubtypeof_errors.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./issubtypeof_errors.luau(10,21): TypeError: 'checksubtype' type function errored at runtime: [string \"checksubtype\"]:3: Not a subtype!\n\
     ./issubtypeof_errors.luau(11,23): TypeError: 'checksubtype' type function errored at runtime: [string \"checksubtype\"]:3: Not a subtype!\n\
     ./issubtypeof_errors.luau(12,25): TypeError: 'checksubtype' type function errored at runtime: [string \"checksubtype\"]:3: Not a subtype!\n\
     ./issubtypeof_errors.luau(8,7): LocalUnused: Variable 'validnil' is never used; prefix with '_' to silence\n\
     ./issubtypeof_errors.luau(9,7): LocalUnused: Variable 'validstring' is never used; prefix with '_' to silence\n\
     ./issubtypeof_errors.luau(10,7): LocalUnused: Variable 'invalidunion' is never used; prefix with '_' to silence\n\
     ./issubtypeof_errors.luau(11,7): LocalUnused: Variable 'invalidboolean' is never used; prefix with '_' to silence\n\
     ./issubtypeof_errors.luau(12,7): LocalUnused: Variable 'invalidsingleton' is never used; prefix with '_' to silence\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "issubtypeof_errors.luau");
  assert_eq!(code, 0);
  assert_eq!(
    stderr,
    "./issubtypeof_errors.luau(8,7): LocalUnused: Variable 'validnil' is never used; prefix with '_' to silence\n\
     ./issubtypeof_errors.luau(9,7): LocalUnused: Variable 'validstring' is never used; prefix with '_' to silence\n\
     ./issubtypeof_errors.luau(10,7): LocalUnused: Variable 'invalidunion' is never used; prefix with '_' to silence\n\
     ./issubtypeof_errors.luau(11,7): LocalUnused: Variable 'invalidboolean' is never used; prefix with '_' to silence\n\
     ./issubtypeof_errors.luau(12,7): LocalUnused: Variable 'invalidsingleton' is never used; prefix with '_' to silence\n",
  );
}

/// cpp `analysis/user-defined-type-functions/issubtypeof_tables`（flags-on/flags-off 快照全等，单形态移植）：表协变与 `read` 属性的 `issubtypeof` 判定全部命中单例真值，双模式零诊断。
#[test]
fn golden_user_type_function_issubtypeof_tables() {
  let ws = ws("issubtypeof-tables");
  ws.write(
    "issubtypeof_tables.luau",
    r#"type function issub(subtype, supertype)
    return types.singleton(subtype:issubtypeof(supertype))
end

type Wide = { a: number, b: string }
type Narrow = { a: number }
type Different = { a: string }
type ReadNumber = { read a: number }
type ReadNumberOrString = { read a: number | string }

local wideIsNarrow: issub<Wide, Narrow> = true
local narrowIsWide: issub<Narrow, Wide> = false
local wideIsDifferent: issub<Wide, Different> = false
local readCovariance: issub<ReadNumber, ReadNumberOrString> = true

assert(wideIsNarrow and not narrowIsWide and not wideIsDifferent and readCovariance)
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "issubtypeof_tables.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "issubtypeof_tables.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/user-defined-type-functions/optional_type`（flags-off 形态移植：Rust CLI 默认旗标复现 cpp flags-off 渲染）：`types.optional(types.number)` 展开为 number?，_show 注 never 报（6,12）分量枚举 unreachable。
#[test]
fn golden_user_type_function_optional_type() {
  let ws = ws("optional-type");
  ws.write(
    "optional_type.luau",
    r#"type function optionalnumber()
    return types.optional(types.number)
end

local function _show(value: optionalnumber<>): never
    return value
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "optional_type.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./optional_type.luau(6,12): TypeError: Expected this to be unreachable, but got 'number?'; \n\
     this is because \n\
     \t * the 1st component of the union is `number`, which is not a subtype of `never`\n\
     \t * the 2nd component of the union is `nil`, which is not a subtype of `never`\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "optional_type.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/user-defined-type-functions/primitive_methods`（flags-on/flags-off 快照全等，单形态移植）：`singleton(nil)/number/boolean` 经 `:is()` 自检原样返回，双模式零诊断。
#[test]
fn golden_user_type_function_primitive_methods() {
  let ws = ws("primitive-methods");
  ws.write(
    "primitive_methods.luau",
    r#"type function getnil()
    local value = types.singleton(nil)
    if value:is("nil") then
        return value
    end
    return types.string
end

type function getnumber()
    local value = types.number
    if value:is("number") then
        return value
    end
    return types.string
end

type function getboolean()
    local value = types.boolean
    if value:is("boolean") then
        return value
    end
    return types.string
end

local function _nil(value: getnil<>): nil
    return value
end

local function _number(value: getnumber<>): number
    return value
end

local function _boolean(value: getboolean<>): boolean
    return value
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "primitive_methods.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "primitive_methods.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/user-defined-type-functions/table_methods`（flags-on/flags-off 快照全等，单形态移植）：`newtable` 索引器+属性读写遍历重建表类型，_show 注 never 报（25,12）unreachable。
#[test]
fn golden_user_type_function_table_methods() {
  let ws = ws("table-methods");
  ws.write(
    "table_methods.luau",
    r#"type function gettable()
    local indexer = {
        index = types.number,
        readresult = types.boolean,
        writeresult = types.boolean,
    }
    local value = types.newtable(nil, indexer, nil)
    value:setproperty(types.singleton("string"), types.number)
    value:setproperty(types.singleton("number"), types.string)
    value:setproperty(types.singleton("string"), nil)

    local result = types.newtable(nil, nil, nil)
    for key, property in value:properties() do
        result:setreadproperty(key, property.read)
        result:setwriteproperty(key, property.write)
    end
    if result:is("table") then
        result:setindexer(types.boolean, types.string)
        return result
    end
    return types.number
end

local function _show(value: gettable<>): never
    return value
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "table_methods.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./table_methods.luau(25,12): TypeError: Expected this to be unreachable, but got\n\
     \t'{ [boolean]: string, number: string }'\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "table_methods.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/user-defined-type-functions/type_alias_call`（flags-on/flags-off 快照全等，单形态移植）：UDTF 把泛型类型别名 `Optional(value)` 当值调用，两实例注解匹配，双模式零诊断。
#[test]
fn golden_user_type_function_type_alias_call() {
  let ws = ws("type-alias-call");
  ws.write(
    "type_alias_call.luau",
    r#"type Optional<T> = T?

type function makeoptional(value)
    return Optional(value)
end

local first: makeoptional<{ a: number }> = { a = 2 }
local second: makeoptional<{ b: number }> = { b = 2 }

assert(first ~= nil and second ~= nil)
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "type_alias_call.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "type_alias_call.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/user-defined-type-functions/type_alias_reduction`（flags-on/flags-off 快照全等，单形态移植）：UDTF 返回可归约别名实例 `rawget<T, "a">`，number/string 两实例匹配，双模式零诊断。
#[test]
fn golden_user_type_function_type_alias_reduction() {
  let ws = ws("type-alias-reduction");
  ws.write(
    "type_alias_reduction.luau",
    r#"type Property<T> = rawget<T, "a">

type function propertytype(value)
    return Property(value)
end

local first: propertytype<{ a: number }> = 2
local second: propertytype<{ a: string }> = "value"

assert(first == 2 and second == "value")
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "type_alias_reduction.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");

  let (code, stderr) = analyze_nonstrict(&ws, "type_alias_reduction.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/user-defined-type-functions/typecheck_failure`（flags-on/flags-off 快照全等，单形态移植）：`types.singleton({ 1 })` 对数组表造单例，（2,28）报与 '(boolean | string)?' 注解不匹配。
#[test]
fn golden_user_type_function_typecheck_failure() {
  let ws = ws("typecheck-failure");
  ws.write(
    "typecheck_failure.luau",
    r#"type function invalid()
    return types.singleton({ 1 })
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "typecheck_failure.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./typecheck_failure.luau(2,28): TypeError: Expected this to be '(boolean | string)?', but got '{number}'\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "typecheck_failure.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}

/// cpp `analysis/user-defined-type-functions/union_methods`（flags-off 形态移植：Rust CLI 默认旗标复现 cpp flags-off 渲染）：`types.unionof` 三分量联合经 `:components()` 遍历重建，_show 注 never 报（14,12）unreachable（联合分量枚举 verbose 渲染）。
#[test]
fn golden_user_type_function_union_methods() {
  let ws = ws("union-methods");
  ws.write(
    "union_methods.luau",
    r#"type function getunion()
    local value = types.unionof(types.string, types.number, types.boolean)
    if value:is("union") then
        local components = {}
        for _, component in value:components() do
            table.insert(components, component)
        end
        return types.unionof(table.unpack(components))
    end
    return types.number
end

local function _show(value: getunion<>): never
    return value
end
"#,
  );

  let (code, stderr) = analyze_strict(&ws, "union_methods.luau");
  assert_eq!(code, 1);
  assert_eq!(
    stderr,
    "./union_methods.luau(14,12): TypeError: Expected this to be unreachable, but got\n\
     \t'boolean | number | string'; \n\
     this is because \n\
     \t * the 1st component of the union is `string`, which is not a subtype of `never`\n\
     \t * the 2nd component of the union is `number`, which is not a subtype of `never`\n\
     \t * the 3rd component of the union is `boolean`, which is not a subtype of `never`\n",
  );

  let (code, stderr) = analyze_nonstrict(&ws, "union_methods.luau");
  assert_eq!(code, 0);
  assert_eq!(stderr, "");
}
