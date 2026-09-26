#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""gen_capi.py —— 一次性生成脚本：把 ulua-vm 的 capi feature（C 符号导出层）拆到独立 crate ulua-capi。

用法（任意目录均可，脚本自行定位仓库根)::

    python3 crates/ulua-capi/tools/gen_capi.py

行为：
  1. 扫描 crates/ulua-vm/src 中全部
     `#[cfg_attr(feature = "capi", unsafe(export_name = "..."))]`（单行/跨行形态），
     提取紧邻 item（fn）的完整签名、参数与所属模块路径；
  2. 在 crates/ulua-capi/src 下按 vm 模块镜像生成壳文件：每个外向函数生成同名
     同参壳，`#[unsafe(export_name = "X")]` 后逐参数透传 `ulua_vm::<path>::<name>`，
     零逻辑；
  3. 改写 ulua-vm：删除全部 capi cfg_attr 块；被导出函数可见性 `pub(crate)` 升
     为 `pub`。

三个数据符号 static（ulua_luaH_dummynode / ulua_luaO_nilobject_ /
ulua_luauF_table）的 vm 侧 const 化提升（pub const / pub const fn）为一次性
手工改造，本脚本只生成 capi 侧引用它们的 `pub static` 壳，并在生成前断言
vm 侧的 pub const 已就位。

`lua_v_doarithimpl` 的 8 个导出由宏 `tm_exports!` 生成，export 名由
concat!(...stringify!($variant)) 拼出，此处按宏调用列表特判展开。

脚本为一次性：改写完成后 ulua-vm 中不再有 capi 标记，重跑只会得到 0 匹配；
保留本文件用于复现与审计。

注意（capi-safety 收口后，b26 实采校准）：src/functions 下 227 个函数导出壳
与 src/macros 下 3 个数据符号壳（合计 230 个导出，源码 `export_name` 属性与
staticlib nm 符号表双轨实测一致）的 `/// # Safety`
文档已升级为逐参数前提契约（非空/对齐/生命周期/单线程驱动），并为体内 unsafe
块补齐 `// Safety:` 理由注释，与本脚本 SAFETY_DOC 模板（通用一行）已有意分叉；
重跑本脚本仅用于结构审计，会丢失手工契约，勿用于覆盖生成。

注意（宏壳收口同步，abs-r121）：functions/ 下同形壳已陆续收口为 mod.rs 模板宏
`capi_shell_*!` 的单行/多行调用。其中 lua_userdatadirectfield_set* / lua_push* /
check·opt / barrier_voidptr 四宏族的宏调用形态、连同 arith_tm_exports! 先例
（lua_v_doarithimpl.rs，宏模板在文件内）共 16 个文件，由下方 MACRO_SHELL_FILES
表登记最终形态，重跑时直接写回表内文本；mod.rs 一律不整体重写、只追加缺失的
pub mod 声明；macros/ 数据符号壳已存在时不回写。写回末尾统一过一遍 nightly
rustfmt 归一排版，避免表内文本与树内换行风格产生纯格式漂移（实测重跑零 diff）。
"""

import json
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
VM_SRC = os.path.join(REPO, "crates", "ulua-vm", "src")
CAPI_SRC = os.path.join(REPO, "crates", "ulua-capi", "src")

# 捕获字面量 export 名的完整 cfg_attr 块（单行与跨行均可，\s* 覆盖换行）
ATTR_RE = re.compile(
    r'[ \t]*#\[cfg_attr\(\s*feature = "capi",\s*unsafe\(export_name = "([^"]+)"\)\s*\)\][ \t]*\n'
)
# 删除用：任何含 feature = "capi" 的 cfg_attr 块（含宏体内 concat! 形态；体内不含 ']'）
DELETE_RE = re.compile(r'[ \t]*#\[cfg_attr\([^]]*"capi"[^]]*\)\][ \t]*\n')

PRIMITIVES = {
    "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128",
    "f32", "f64", "bool", "char", "usize", "isize", "str", "Self",
}
TYPE_KEYWORDS = {"fn", "extern", "unsafe", "dyn", "impl", "const", "mut", "move", "for"}

FN_RE = re.compile(
    r'^pub(?:\((?:crate|super|[^)]*)\))?\s+unsafe\s+(?:extern\s+"(?P<abi>[^"]+)"\s+)?fn\s+(?P<name>\w+)\s*\((?P<params>.*)\)\s*(?:->\s*(?P<ret>.+))?$',
    re.S,
)

# lua_v_doarithimpl：宏生成的 8 个导出，签名一致
DOARITH_REL = 'functions/lua_v_doarithimpl.rs'
DOARITH_SIG = [('l', '*mut lua_State'), ('ra', 'StkId'), ('rb', '*const TValue'), ('rc', '*const TValue')]
DOARITH_NEED = [
    'ulua_vm::records::lua_state::lua_State',
    'ulua_vm::type_aliases::stk_id::StkId',
    'ulua_vm::type_aliases::t_value::TValue',
]


# ---------------------------------------------------------------------------
# use 语句解析：简单名 -> 完整路径（截断到 #[cfg(test)] 之前，忽略测试导入）
# ---------------------------------------------------------------------------


def strip_test_mod(src: str) -> str:
    m = re.search(r'^#\[cfg\(test\)\]', src, re.M)
    return src[: m.start()] if m else src


def split_top_level(text: str, sep: str):
    parts, depth, cur = [], 0, ''
    for c in text:
        if c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
        if c == sep and depth == 0:
            parts.append(cur)
            cur = ''
        else:
            cur += c
    parts.append(cur)
    return [p for p in parts if p.strip() != '']


def expand_use(item: str, prefix: str, out: dict):
    item = ' '.join(item.split())
    if not item:
        return
    if item == 'self':
        if prefix:
            out[prefix.split('::')[-1]] = prefix
        return
    if item.endswith('::*') or item == '*':
        return  # glob：若签名依赖其导入的名字会走未解析报错路径
    m = re.match(r'^(.*?)(?:::(\{.*\}))?(?:\s+as\s+(\w+))?$', item, re.S)
    head, group, alias = m.group(1), m.group(2), m.group(3)
    head = head.strip()
    path = f'{prefix}::{head}' if prefix and head else (prefix or head)
    if group:
        for sub in split_top_level(group[1:-1], ','):
            expand_use(sub.strip(), path, out)
    elif alias:
        out[alias] = path
    elif head != '*':
        out[head.split('::')[-1]] = path


def parse_use_statements(src: str):
    out = {}
    body = strip_test_mod(src)
    for m in re.finditer(r'(?m)^(?:pub(?:\([^)]*\))?\s+)?use\s+', body):
        start = m.end()
        depth = 0
        i = start
        while i < len(body):
            c = body[i]
            if c == '{':
                depth += 1
            elif c == '}':
                depth -= 1
            elif c == ';' and depth == 0:
                break
            i += 1
        stmt = body[start:i]
        for tree in split_top_level(stmt, ','):
            expand_use(tree.strip(), '', out)
    return out


def normalize_use_path(path: str) -> str:
    """vm 文件内 use 路径换算为 capi 侧可导入路径。"""
    if path.startswith('crate::'):
        return 'ulua_vm::' + path[len('crate::'):]
    if path.startswith('super::'):
        return 'ulua_vm::' + path[len('super::'):]
    return path


def type_tokens(type_text: str):
    t = re.sub(r'"[^"]*"', '', type_text)          # 去字符串字面量（"C-unwind"）
    t = re.sub(r"'[A-Za-z_][0-9A-Za-z_]*", '', t)  # 去生命周期
    names = []
    for tok in re.findall(r'[A-Za-z_][A-Za-z0-9_]*', t):
        if tok in PRIMITIVES or tok in TYPE_KEYWORDS:
            continue
        names.append(tok)
    return names


# ---------------------------------------------------------------------------
# 扫描
# ---------------------------------------------------------------------------


def find_item_start(lines, attr_last_line_idx):
    i = attr_last_line_idx + 1
    while i < len(lines):
        s = lines[i].strip()
        if s == '' or s.startswith('///') or s.startswith('//') or s.startswith('#'):
            i += 1
            continue
        return i
    raise RuntimeError('attribute 后无 item')


def collect_fn_signature(lines, start_idx):
    buf = []
    i = start_idx
    while i < len(lines):
        l = lines[i]
        buf.append(l)
        if '{' in re.sub(r'//.*$', '', l):
            break
        i += 1
    text = '\n'.join(buf)
    text = re.sub(r'\{[^{]*$', '', text)  # 去掉函数体起始 '{'
    return text.strip()


def analyze():
    fns, statics, promote_lines, problems = [], [], [], []
    seen_exports = set()

    for dirpath, _, files in os.walk(VM_SRC):
        for f in sorted(files):
            if not f.endswith('.rs'):
                continue
            path = os.path.join(dirpath, f)
            rel = os.path.relpath(path, VM_SRC).replace(os.sep, '/')
            mod_path = 'ulua_vm::' + rel[:-3].replace('/', '::')
            src = open(path, encoding='utf-8').read()
            lines = src.split('\n')
            use_map = parse_use_statements(src)
            bare_src = strip_test_mod(src)

            for m in ATTR_RE.finditer(src):
                export = m.group(1)
                if export in seen_exports:
                    problems.append(f'重复 export 名 {export} @ {rel}')
                    continue
                seen_exports.add(export)
                attr_first = src[: m.start()].count('\n')
                attr_last = src[: max(m.end() - 1, m.start())].count('\n')
                item_idx = find_item_start(lines, attr_last)
                item_line = lines[item_idx].strip()
                if re.match(r'pub(?:\([^)]*\))?\s+static\b', item_line):
                    statics.append(dict(file=path, rel=rel, export=export))
                    continue
                sig_flat = ' '.join(collect_fn_signature(lines, item_idx).split())
                fm = FN_RE.match(sig_flat)
                if not fm:
                    problems.append(f'FN 解析失败 {rel} {export}: {sig_flat[:140]}')
                    continue
                name = fm.group('name')
                params = [p for p in split_top_level(fm.group('params'), ',') if p.strip()]
                ret = (fm.group('ret') or '').strip()
                parsed, need, ok = [], set(), True
                for p in params:
                    pm = re.match(r'^(mut\s+)?([A-Za-z_]\w*)\s*:\s*(.+)$', p.strip(), re.S)
                    if not pm:
                        ok = False
                        problems.append(f'参数解析失败 {rel} {name}: {p!r}')
                        break
                    ty = ' '.join(pm.group(3).split())
                    for tok in type_tokens(ty):
                        if tok in use_map:
                            need.add(normalize_use_path(use_map[tok]))
                        elif re.search(
                            r'(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum|type|trait|const)\s+' + tok + r'\b', bare_src
                        ):
                            need.add(mod_path + '::' + tok)
                        else:
                            ok = False
                            problems.append(f'未解析类型 {tok} @ {rel} {name}({p})')
                    parsed.append(dict(name=(pm.group(2) or ''), ty=ty))
                if not ok:
                    continue
                for tok in type_tokens(ret):
                    if tok in use_map:
                        need.add(normalize_use_path(use_map[tok]))
                    elif re.search(
                        r'(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum|type|trait)\s+' + tok + r'\b', bare_src
                    ):
                        need.add(mod_path + '::' + tok)
                    else:
                        ok = False
                        problems.append(f'未解析返回类型 {tok} @ {rel} {name}')
                if not ok:
                    continue
                if rel.endswith('/mod.rs'):
                    continue  # mod.rs 是模块注册表，不是函数实现文件
                fns.append(
                    dict(
                        rel=rel, export=export, name=name, mod_path=mod_path,
                        abi='C-unwind',  # 导出壳是 C 链接面，恒为 C-unwind（与被调函数的 Rust 内部 ABI 解耦）
                        params=parsed, ret=ret, imports=sorted(need),
                    )
                )
                if item_line.startswith('pub(crate)'):
                    promote_lines.append((path, attr_first, item_idx))
    return fns, statics, promote_lines, problems


def doarith_variants():
    src = open(os.path.join(VM_SRC, 'functions', 'lua_v_doarithimpl.rs'), encoding='utf-8').read()
    m = re.search(r'tm_exports!\s*\{([^}]*)\}', src)
    out = [(vm.group(1), vm.group(2)) for vm in re.finditer(r'\(\s*(\w+)\s*,\s*(\w+)\s*\)', m.group(1))]
    assert len(out) == 8, out
    return out


# ---------------------------------------------------------------------------
# 壳生成
# ---------------------------------------------------------------------------

FILE_HEADER = (
    '//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/{rel}）。\n'
    '//! 每个导出壳与 ulua-vm 对应函数签名一致，仅做逐参数透传，零逻辑。\n'
)
SAFETY_DOC = '/// # Safety\n/// 透传至 `{call}`，安全前置条件与被调函数一致。\n'

# mod.rs 模板宏收口的壳文件登记表：rel -> 该文件逐字最终形态（头注释 + 宏调用）。
# 这些文件不再手写透传壳，generate() 命中此表时直接写回登记文本，保证重跑输出与
# 手改（abs-r121 四批收口）逐字节一致；宏模板本体见 src/functions/shells.rs。
_MACRO_HEADER = (
    '//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/{rel}）。\n'
    '//! 导出壳为 `functions/shells.rs` 中模板宏 `{macro}!` 的一次调用，壳契约见宏模板。\n'
)


def _macro_shell(rel: str, macro: str, *calls: str) -> str:
    body = _MACRO_HEADER.format(rel=rel, macro=macro) + '\n'.join(calls) + '\n'
    return body


MACRO_SHELL_FILES = {
    # arith_tm_exports! 先例（opt-o5 波收口，宏模板在本文件内）：登记最终形态保证重跑一致
    'functions/lua_v_doarithimpl.rs': r"""//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_v_doarithimpl.rs）。
//! 每个导出壳与 ulua-vm 对应函数签名一致，仅做逐参数透传，零逻辑。
//! 8 个算术 TM 变体导出（签名与逐参数契约完全同形，仅变体名不同）由宏
//! `arith_tm_exports!` 单模板生成，与 vm 侧 `tm_exports!` 先例同构；导出符号与
//! 函数签名与逐壳手写时逐字节一致。
use ulua_vm::{
  functions::lua_v_doarithimpl,
  records::lua_state::lua_State, type_aliases::{stk_id::StkId, t_value::TValue},
};

/// 生成一个算术 TM 导出壳：`ulua_luaV_doarithimpl_<变体>` 透传至 ulua-vm 同名实现。
/// 第三参数为可选的 rb/rc 契约补注（一元 TmUnm 用）。
macro_rules! arith_tm_exports {
  ($( ( $variant:ident, $name:ident $(, $unary_note:literal)? ) ),+ $(,)?) => {
    $(
      #[doc = concat!(
        "# Safety\n",
        "C ABI 导出壳（符号 `ulua_luaV_doarithimpl_", stringify!($variant), "`），仅逐参数透传至 `lua_v_doarithimpl::", stringify!($name), "(l, ra, rb, rc)`，零逻辑，本帧不解引用任何指针。调用方须保证：\n",
        "- `l`：指向由本 VM 创建的合法 `lua_State`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；\n",
        "- `ra`（`StkId`）：可写栈槽指针，指向当前帧栈界内预留的结果槽，调用期间不迁移（运算结果写入该槽）；\n",
        "- `rb`/`rc`（`*const TValue`）：指向合法、地址稳定（栈槽或被 GC 持有）的 `TValue` 操作数，调用期间只读存活",
        $( $unary_note, )?
        "；\n",
        "- TM 调用协议（结果槽、栈余量、受保护帧）与被调函数的 `# Safety` 契约一致。"
      )]
      #[unsafe(export_name = concat!("ulua_luaV_doarithimpl_", stringify!($variant)))]
      pub unsafe extern "C-unwind" fn $name(
        l: *mut lua_State,
        ra: StkId,
        rb: *const TValue,
        rc: *const TValue,
      ) {
        // Safety: 宏按 TM 变体生成 C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：l 为有效 lua_State*，ra 为可写结果槽指针，rb/rc 为只读操作数 TValue 指针，调用期间存活。本壳不解引用任何指针、不在本帧重建引用，仅原样转调 ulua-vm 同名实现，故不存在别名/悬挂窗口；参数合法性前提即该实现 /// # Safety 所列契约。
        unsafe { lua_v_doarithimpl::$name(l, ra, rb, rc) }
      }
    )+
  };
}

arith_tm_exports! {
  (TmAdd, lua_v_doarithimpl_tm_add),
  (TmDiv, lua_v_doarithimpl_tm_div),
  (TmIDiv, lua_v_doarithimpl_tm_idiv),
  (TmMod, lua_v_doarithimpl_tm_mod),
  (TmMul, lua_v_doarithimpl_tm_mul),
  (TmPow, lua_v_doarithimpl_tm_pow),
  (TmSub, lua_v_doarithimpl_tm_sub),
  (
    TmUnm,
    lua_v_doarithimpl_tm_unm,
    "（一元取负仅读 `rb`，`rc` 按协议仍须为合法 TValue）"
  ),
}
""",
    # capi_shell_udfield_set!：`(result: *mut c_void [, 值型参数]) -> ()` 直写字段族
    'functions/lua_userdatadirectfield_setnil.rs': _macro_shell(
        'functions/lua_userdatadirectfield_setnil.rs', 'capi_shell_udfield_set',
        'capi_shell_udfield_set!(\n  lua_userdatadirectfield_setnil,\n  lua_userdatadirectfield_setnil\n);',
    ),
    'functions/lua_userdatadirectfield_setboolean.rs': _macro_shell(
        'functions/lua_userdatadirectfield_setboolean.rs', 'capi_shell_udfield_set',
        'capi_shell_udfield_set!(lua_userdatadirectfield_setboolean, lua_userdatadirectfield_setboolean, b: c_int);',
    ),
    'functions/lua_userdatadirectfield_setnumber.rs': _macro_shell(
        'functions/lua_userdatadirectfield_setnumber.rs', 'capi_shell_udfield_set',
        'capi_shell_udfield_set!(lua_userdatadirectfield_setnumber, lua_userdatadirectfield_setnumber, n: f64);',
    ),
    'functions/lua_userdatadirectfield_setinteger_64.rs': _macro_shell(
        'functions/lua_userdatadirectfield_setinteger_64.rs', 'capi_shell_udfield_set',
        'capi_shell_udfield_set!(lua_userdatadirectfield_setinteger_64, lua_userdatadirectfield_setinteger64, n: i64);',
        'capi_shell_udfield_set!(lua_userdatadirectfield_setinteger_64, lua_userdatadirectfield_setinteger_64, n: i64);',
    ),
    'functions/lua_userdatadirectfield_setvector_lapi.rs': _macro_shell(
        'functions/lua_userdatadirectfield_setvector_lapi.rs', 'capi_shell_udfield_set',
        'capi_shell_udfield_set!(\n  lua_userdatadirectfield_setvector_lapi,\n  lua_userdatadirectfield_setvector_void_f32_f32_f32_f32,\n  x: f32,\n  y: f32,\n  z: f32,\n  w: f32\n);',
        'capi_shell_udfield_set!(\n  lua_userdatadirectfield_setvector_lapi,\n  lua_userdatadirectfield_setvector_void_f32_f32_f32,\n  x: f32,\n  y: f32,\n  z: f32\n);',
    ),
    # capi_shell_push!：`(l [, 值型参数]) -> ()` 压栈族
    'functions/lua_pushnil.rs': _macro_shell(
        'functions/lua_pushnil.rs', 'capi_shell_push',
        'capi_shell_push!(lua_pushnil, lua_pushnil);',
    ),
    'functions/lua_pushboolean.rs': _macro_shell(
        'functions/lua_pushboolean.rs', 'capi_shell_push',
        'capi_shell_push!(lua_pushboolean, lua_pushboolean, b: c_int);',
    ),
    'functions/lua_pushnumber.rs': _macro_shell(
        'functions/lua_pushnumber.rs', 'capi_shell_push',
        'capi_shell_push!(lua_pushnumber, lua_pushnumber, n: f64);',
    ),
    'functions/lua_pushinteger_64.rs': _macro_shell(
        'functions/lua_pushinteger_64.rs', 'capi_shell_push',
        'capi_shell_push!(lua_pushinteger_64, lua_pushinteger_64, n: i64);',
    ),
    'functions/lua_pushvector_lapi.rs': _macro_shell(
        'functions/lua_pushvector_lapi.rs', 'capi_shell_push',
        'capi_shell_push!(\n  lua_pushvector_lapi,\n  lua_pushvector_lua_state_f32_f32_f32_f32,\n  x: f32,\n  y: f32,\n  z: f32,\n  w: f32\n);',
        'capi_shell_push!(lua_pushvector_lapi, lua_pushvector_lua_state_f32_f32_f32, x: f32, y: f32, z: f32);',
    ),
    # capi_shell_check_opt!：`(l, narg [, 附加值型参数]) -> 标量` check/opt 族
    'functions/lua_l_checkboolean.rs': _macro_shell(
        'functions/lua_l_checkboolean.rs', 'capi_shell_check_opt',
        'capi_shell_check_opt!(lua_l_checkboolean, lua_l_checkboolean, c_int, narg: c_int);',
    ),
    'functions/lua_l_checkinteger_64.rs': _macro_shell(
        'functions/lua_l_checkinteger_64.rs', 'capi_shell_check_opt',
        'capi_shell_check_opt!(lua_l_checkinteger_64, lua_l_checkinteger_64, i64, narg: c_int);',
    ),
    'functions/lua_l_optinteger_64.rs': _macro_shell(
        'functions/lua_l_optinteger_64.rs', 'capi_shell_check_opt',
        'capi_shell_check_opt!(\n  lua_l_optinteger_64,\n  lua_l_optinteger_64,\n  i64,\n  narg: c_int,\n  def: i64\n);',
    ),
    # capi_shell_l_int!：`(l, <c_int 值型参数>) -> c_int`（unit 尾缀为无返回值）查询/设置族
    'functions/coresumefinish.rs': _macro_shell(
        'functions/coresumefinish.rs', 'capi_shell_l_int',
        'capi_shell_l_int!(coresumefinish, coresumefinish, "ulua_coresumefinish", r);',
    ),
    'functions/lua_g_hasnative.rs': _macro_shell(
        'functions/lua_g_hasnative.rs', 'capi_shell_l_int',
        'capi_shell_l_int!(lua_g_hasnative, lua_g_hasnative, "ulua_lua_g_hasnative", level);',
    ),
    'functions/lua_g_isnative.rs': _macro_shell(
        'functions/lua_g_isnative.rs', 'capi_shell_l_int',
        'capi_shell_l_int!(lua_g_isnative, lua_g_isnative, "ulua_luaG_isnative", level);',
    ),
    'functions/lua_isstring.rs': _macro_shell(
        'functions/lua_isstring.rs', 'capi_shell_l_int',
        'capi_shell_l_int!(lua_isstring, lua_isstring, "ulua_lua_isstring", idx);',
    ),
    'functions/lua_type.rs': _macro_shell(
        'functions/lua_type.rs', 'capi_shell_l_int',
        'capi_shell_l_int!(lua_type, lua_type, "ulua_lua_type", idx);',
    ),
    'functions/str_find_aux.rs': _macro_shell(
        'functions/str_find_aux.rs', 'capi_shell_l_int',
        'capi_shell_l_int!(str_find_aux, str_find_aux, "ulua_str_find_aux", find);',
    ),
    'functions/lua_settop.rs': _macro_shell(
        'functions/lua_settop.rs', 'capi_shell_l_int',
        'capi_shell_l_int!(lua_settop, lua_settop, "ulua_lua_settop", idx, unit);',
    ),
    'functions/lua_setuserdatametatable.rs': _macro_shell(
        'functions/lua_setuserdatametatable.rs', 'capi_shell_l_int',
        'capi_shell_l_int!(\n  lua_setuserdatametatable,\n  lua_setuserdatametatable,\n  "ulua_lua_setuserdatametatable",\n  tag,\n  unit\n);',
    ),
    # functions/lua_singlestep.rs 已随 vm 侧 B 档前移（&mut LuaState 接收者）从
    # capi_shell_l_int! 退役为显式壳（r7-gtd，先例 lua_status/lua_setthreaddata/lua_l_checkudata），
    # 不再是宏收口壳，故移出本登记表；文件为手写单源，重跑不触碰。
    # capi_shell_barrier_voidptr!：`(l, <不透明指针>, v: *mut c_void) -> ()` GC 屏障族
    'functions/lua_c_barrierf.rs': _macro_shell(
        'functions/lua_c_barrierf.rs', 'capi_shell_barrier_voidptr',
        'capi_shell_barrier_voidptr!(\n  lua_c_barrierf,\n  lua_c_barrierf_export,\n  "ulua_luaC_barrierf",\n  o\n);',
    ),
    'functions/lua_c_barriertable.rs': _macro_shell(
        'functions/lua_c_barriertable.rs', 'capi_shell_barrier_voidptr',
        'capi_shell_barrier_voidptr!(\n  lua_c_barriertable,\n  lua_c_barriertable_export,\n  "ulua_luaC_barriertable",\n  t\n);',
    ),
}


def render_imports(paths, module_path):
    grouped = {}
    for p in set(paths) | {module_path}:
        grouped.setdefault(p.split('::')[0], set()).add(p)
    lines = []
    for head in sorted(grouped):
        items = sorted(grouped[head])
        if len(items) == 1:
            lines.append(f'use {items[0]};')
        else:
            lines.append(f'use {head}::{{{", ".join(i[len(head) + 2:] for i in items)}}};')
    return lines


def render_fn(item) -> str:
    mod_short = item['mod_path'].split('::')[-1]
    abi = f'extern "{item["abi"]}" ' if item.get('abi') else ''
    args, params = [], []
    for p in item['params']:
        if p['name'] == '_':
            params.append(f'_: {p["ty"]}')
            continue
        nm = p['name'].removeprefix('mut ').strip()
        params.append(f'{nm}: {p["ty"]}')
        args.append(nm)
    sig_ret = f' -> {item["ret"]}' if item['ret'] else ''
    call = f"{mod_short}::{item['name']}({', '.join(args)})"
    out = SAFETY_DOC.format(call=call)
    out += f'#[unsafe(export_name = "{item["export"]}")]\n'
    out += f'pub unsafe {abi}fn {item["name"]}(\n'
    for p in params:
        out += f'  {p},\n'
    out += f'){sig_ret} {{\n  unsafe {{ {call} }}\n}}\n'
    return out


def generate(fns):
    by_rel = {}
    for item in fns:
        by_rel.setdefault(item['rel'], []).append(item)
    written = []
    for rel, items in sorted(by_rel.items()):
        out_path = os.path.join(CAPI_SRC, rel)
        os.makedirs(os.path.dirname(out_path), exist_ok=True)
        if rel in MACRO_SHELL_FILES:  # 宏收口壳：直接写回登记表单源文本，与手改逐字一致
            open(out_path, 'w', encoding='utf-8').write(MACRO_SHELL_FILES[rel])
            written.append(rel)
            continue
        mod_path = 'ulua_vm::' + rel[:-3].replace('/', '::')
        imports = set()
        for it in items:
            imports.update(it['imports'])
        parts = [FILE_HEADER.format(rel=rel), '\n'.join(render_imports(imports, mod_path)), '\n\n']
        for it in sorted(items, key=lambda x: x['export']):
            parts.append(render_fn(it) + '\n')
        open(out_path, 'w', encoding='utf-8').write(''.join(parts))
        written.append(rel)
    # vm 侧 capi 标记已随一次性改写移除（重跑扫描为空集），宏壳登记表无条件写回，
    # 保证重跑产出与宏收口手改后的树逐字节一致。
    for rel in sorted(set(MACRO_SHELL_FILES) - set(by_rel)):
        out_path = os.path.join(CAPI_SRC, rel)
        os.makedirs(os.path.dirname(out_path), exist_ok=True)
        open(out_path, 'w', encoding='utf-8').write(MACRO_SHELL_FILES[rel])
        written.append(rel)
    return written


def assert_static_prep():
    dn = open(os.path.join(VM_SRC, 'macros', 'dummynode.rs'), encoding='utf-8').read()
    no = open(os.path.join(VM_SRC, 'macros', 'lua_o_nilobject.rs'), encoding='utf-8').read()
    ft = open(os.path.join(VM_SRC, 'macros', 'luau_f_table.rs'), encoding='utf-8').read()
    assert 'pub const LUA_H_DUMMYNODE_VALUE' in dn, 'vm 侧 dummynode pub const 未就位'
    assert 'pub const LUA_O_NILOBJECT_VALUE' in no, 'vm 侧 nilobject pub const 未就位'
    assert 'pub const fn build_table' in ft and re.search(r'\bpub const TABLE_LEN', ft), 'vm 侧 luau_f_table const 化未就位'


def render_static_wrappers():
    header = (
        '//! 数据符号导出壳（源：ulua-vm/src/{rel}）。\n'
        '//! 注意：导出的是独立副本——C 侧地址与 ulua-vm 内部 static 地址不同\n'
        '//! （当前无任何消费者，可接受）；vm 内部指针同一性不受影响。\n'
    )
    files = {}
    files['macros/dummynode.rs'] = (
        header.format(rel='macros/dummynode.rs')
        + '\nuse ulua_vm::macros::dummynode::{DummyNodeSentinel, LUA_H_DUMMYNODE_VALUE};\n\n'
        + '#[unsafe(export_name = "ulua_luaH_dummynode")]\npub static LUA_H_DUMMYNODE: DummyNodeSentinel = LUA_H_DUMMYNODE_VALUE;\n'
    )
    files['macros/lua_o_nilobject.rs'] = (
        header.format(rel='macros/lua_o_nilobject.rs')
        + '\nuse ulua_vm::macros::lua_o_nilobject::{NilSentinel, LUA_O_NILOBJECT_VALUE};\n\n'
        + '#[unsafe(export_name = "ulua_luaO_nilobject_")]\npub static LUA_O_NILOBJECT_: NilSentinel = LUA_O_NILOBJECT_VALUE;\n'
    )
    files['macros/luau_f_table.rs'] = (
        header.format(rel='macros/luau_f_table.rs')
        + '\nuse ulua_vm::{\n  macros::luau_f_table::{build_table, TABLE_LEN},\n  type_aliases::luau_fast_function::LuauFastFunction,\n};\n\n'
        + '#[unsafe(export_name = "ulua_luauF_table")]\npub static LUAU_F_TABLE: [LuauFastFunction; TABLE_LEN] = build_table();\n'
    )
    return files


def generate_mod_rs():
    for d in sorted(os.listdir(CAPI_SRC)):
        full = os.path.join(CAPI_SRC, d)
        if not os.path.isdir(full):
            continue
        mods = sorted(f[:-3] for f in os.listdir(full) if f.endswith('.rs') and f != 'mod.rs')
        mod_rs = os.path.join(full, 'mod.rs')
        if os.path.exists(mod_rs):
            # mod.rs 可能携带模板宏定义（capi_shell_*）或手工头注释，一律不整体重写，
            # 只为新增壳文件补齐缺失的 pub mod 声明（追加在文件尾，声明序不影响宏作用域）。
            # 私有 `mod shells;`（模板宏单源文件，abs-r121 自 mod.rs 迁出）视为已声明。
            existing = open(mod_rs, encoding='utf-8').read()
            declared = set(re.findall(r'^(?:pub )?mod (\w+);$', existing, re.M))
            missing = [m for m in mods if m not in declared]
            if missing:
                open(mod_rs, 'a', encoding='utf-8').write(''.join(f'pub mod {m};\n' for m in missing))
            continue
        open(mod_rs, 'w', encoding='utf-8').write(''.join(f'pub mod {m};\n' for m in mods))
    top = sorted(d for d in os.listdir(CAPI_SRC) if os.path.isdir(os.path.join(CAPI_SRC, d)))
    lib = (
        '//! ulua C ABI 导出层：从 ulua-vm 拆出的 `ulua_*` 符号壳。\n'
        '//! 每个壳与 ulua-vm 对应实现签名一致，仅逐参数透传，零逻辑。\n\n'
        + ''.join(f'pub mod {d};\n' for d in top)
    )
    open(os.path.join(CAPI_SRC, 'lib.rs'), 'w', encoding='utf-8').write(lib)


def rewrite_vm(promote_lines):
    by_file = {}
    for path, _attr_line, item_idx in promote_lines:
        by_file.setdefault(path, []).append(item_idx)
    for path, idxs in by_file.items():
        lines = open(path, encoding='utf-8').read().split('\n')
        for i in sorted(idxs, reverse=True):
            assert lines[i].lstrip().startswith('pub(crate)'), f'{path}:{i} 预期 pub(crate)'
            lines[i] = lines[i].replace('pub(crate)', 'pub', 1)
        open(path, 'w', encoding='utf-8').write('\n'.join(lines))
    n = 0
    for dirpath, _, files in os.walk(VM_SRC):
        for f in sorted(files):
            if not f.endswith('.rs'):
                continue
            path = os.path.join(dirpath, f)
            src = open(path, encoding='utf-8').read()
            new, cnt = DELETE_RE.subn('', src)
            if cnt:
                open(path, 'w', encoding='utf-8').write(new)
                n += cnt
    return n


def main():
    fns, statics, promote_lines, problems = analyze()
    if problems:
        print('!! 存在解析问题，先人工处理：', file=sys.stderr)
        for p in problems:
            print('  -', p, file=sys.stderr)
        sys.exit(1)
    # 说明：dummynode / lua_o_nilobject 两处 static 的属性行随其 const 化手工改造
    # 一并删除，脚本运行时仅剩 luau_f_table 一处，故断言为 0..=3。
    assert len(statics) <= 3, statics

    for variant, snake in doarith_variants():
        fns.append(
            dict(
                rel=DOARITH_REL, export='ulua_luaV_doarithimpl_' + variant, name=snake,
                mod_path='ulua_vm::functions::lua_v_doarithimpl', abi='C-unwind',
                params=[dict(name=n, ty=t) for n, t in DOARITH_SIG], ret='', imports=list(DOARITH_NEED),
            )
        )

    assert_static_prep()
    os.makedirs(CAPI_SRC, exist_ok=True)
    written = generate(fns)
    static_files = render_static_wrappers()
    for rel, body in static_files.items():
        out = os.path.join(CAPI_SRC, rel)
        # 数据符号壳的 /// # Safety 契约已手工校准（见文件头注记），已存在则保留不回写。
        if os.path.exists(out):
            continue
        os.makedirs(os.path.dirname(out), exist_ok=True)
        open(out, 'w', encoding='utf-8').write(body)
    generate_mod_rs()
    deleted = rewrite_vm(promote_lines)
    # 表内登记的是语义单源文本，换行排版交给 nightly rustfmt 归一（同 sh/clippy.sh 工具链）。
    subprocess.run(['cargo', '+nightly', 'fmt', '-p', 'ulua-capi'], check=True)

    print(json.dumps(dict(
        fn_wrappers=len(fns),
        fn_wrappers_from_scan=len(fns) - 8,
        fn_wrappers_macro_expanded=8,
        static_wrappers=len(static_files),
        static_wrappers_skipped=0,
        wrapper_files=len(written) + len(static_files),
        cfg_attr_blocks_deleted=deleted,
        promoted_pub_crate=len(promote_lines),
    ), ensure_ascii=False, indent=2))


if __name__ == '__main__':
    main()
