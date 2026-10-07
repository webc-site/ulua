# r16-p28 锚定形探针终审：vm 侧窗口门面收 'a 入借用（判例入档）

- 席号：r16-p28（探针票），基线 `3bc30e00`（dev，r16-v26 判例已并入）。
- 假设：「安全 fn 返回未受约束 `&'a` 窗口」类（见 `.qoder/audit/safe-fn-deref-triage.md`）
  在 ulua-vm 侧可以走**锚定形**（出参寿命收进 `&mut LuaState` 借用，签名去 `'a` 形参），
  以纯安全 fn 关闭漏洞，strictly better than v26 屏障形。
- 判据：**成立，改动保留并入分支**。锚定形在 5 个门面 + access.rs 门面族全部落地，
  worktree 全仓 `cargo check --all-targets --all-features`、`cargo +nightly clippy -q
  --all-targets --all-features -- -D warnings -W clippy::absolute_paths`、
  `cargo +nightly fmt -p ulua-vm/-p ulua-require/-p ulua-repl-cli -- --check` 三门禁
  RC=0，零 `#[allow]`。

## 一、分类统计（唯一必需产出，按调用点计）

渐进锚定实测错误面：phase1（仅 lua_l_checkbuffer_ref）=1、phase2（+buffer_data_ref）=33、
phase4（+checklstring_ref、read_window_ref 体）=40、phase5（require 波及）=12、
phase6（require 修复迭代 E0308 自引入）=5、phase7（repl-cli）=3。

| 类 | 定义 | 点数 | 点位（文件） |
|---|---|---|---|
| A | 先读实参再取窗/长度快照先行/后置·二次派窗/(ptr,len) 裸量快照，≤3 行局部重排 | 15 | buffer_len, buffer_tostring, buffer_copy, buffer_fill, buffer_readstring, buffer_writestring⁺¹, buffer_readbits, buffer_writebits, buffer_writeinteger, buffer_writefp, buffer_writelong, buffer_fromstring⁺¹, str_pack, vector_index（快路）, repl-cli/lua_loadstring |
| B | 当场拷贝 owned 解耦（to_vec/into_owned） | 5 | lua_b_assert（错误路）, vector_index 错误臂并入上行注记, ulua-require 四点（cyclic_placeholder, lua_proxyrequire, lua_requirecont, lua_requireinternal，均 require 冷路径，cpp 同点位 std::string 拷贝背书） |
| C | 结构上需拆分/中间层 | 2 | buffer_window.rs 门面体：删 `buffer_at_ref`，增设借无参中间层 `buffer_data_len` + `buffer_range_checked(l,len,offset,size)->(usize,usize)`；access.rs 第二批：`opt_bytes` 需 `def` 与 `self` 借用共享 `'a` 的签名设计（票面预告特殊处理） |
| D | 与 @ref 宏臂/capi 透传壳/C-ABI 镜像冲突、须动出范围文件 | **0** | — |

A+B 主导（20/22），D 缺席 ⇒ 判「保留」。r16-v24（capi_shell! 增 refstate 臂）在本类
**未兑现**：零 D 冲突即无需其溶解，capi 根本不经这两个门面（C 垫片
`lua_l_checkbuffer`/`lua_l_checklstring` 签名未动）。

## 二、关键实测事实（裁决依据）

1. **裸指针侧重叠借用 rustc 不查**（最小复现 `/tmp/p28mini/a.rs`）：由 `*mut LuaState`
   重建的 `&mut *l` 重叠使用锚定门面不报错 ⇒ strlib 大部（str_format/str_find_aux/
   str_gsub/str_split/str_rep/codepoint/byteoffset/pack 壳…）**零改动过门**；真实
   `&mut LuaState` 值流（双传形参、`l.method()`）才报 E0499/E0502/E0621。错误面因此
   远小于 v17/v18 战役史预估的摩擦面。
2. **可靠性闭合**：锚定后纯安全代码无法再实例化 `'static` 窗（需 unsafe 造
   `&mut LuaState` 裸参）；残余裸窗源头收敛在 unsafe fn `lua_tobuffer_bytes_ref` /
   `lua_tolstring_ref` 边界，其调用点已有 `# Safety` 契约。锚定形与 v26 屏障形 soundness
   等价，但 vm 门面表层**零 unsafe**。
3. **抛错序保持**：cpp typeerror→checkinteger 等可观察序经 `check_type` 只判型不派窗
   前置守卫 + 快照先行重排逐位复现（buffer_copy/buffer_writestring 等）；二次派窗与原
   窗读同槽同值（两读之间无变更 `l` 状态的路径），观测等价。

## 三、unsafe 净差与体量

- unsafe 词频：5669 → 5669，**净 0**（`unsafe fn` 净 0、`unsafe extern` 净 0）。
- `unsafe {` 块：3071 → 3073，**净 +2**（buffer_writestring、buffer_fromstring 各 +1
  窄块，(ptr,len) 快照 → `from_raw_parts` 物化，锚定理想值 0 未达，代价记台账）。
- numstat：26 files, +250/−169（含注释/文档契约改写；`# Safety` 内存契约在 5 门面
  处降格为「调用序契约（正确性，非内存安全）」，持窗期间不得再动 state 的契约改由
  借用检查器背书）。
- 旧注释残引清理：luau_f_readinteger/luau_f_writeinteger/buffer_fill 三处
  `buffer_at_ref` 注释改指 `buffer_range_checked`（仅注释）。

## 四、判例

- **vm 侧（`&mut LuaState` 收参门面）走锚定形**：lua_l_checkbuffer_ref、
  lua_l_checklstring_ref、buffer_data_ref、buffer_read_window_ref、
  access.rs（to_bytes/to_str/check_bytes/check_str/opt_bytes）。
- **rt 侧维持 v26 屏障形**：StateView 为 Copy 手柄、无真实借用可锚，本案不通用。
- 后续新窗口门面优先锚定形；(ptr,len) 裸量快照形 soundness 等价但每点 +1 窄 unsafe
  块，能在 ≤3 行内换长度快照/二次派窗者不必上裸快照。

## 五、并入 dev 复验（r16-v33，主控）

探针基线 `3bc30e00` 上的 D=0 结论**只在当时成立**：与 dev@ace7e6a1（含 r16-v29 的
`lua_b_*` 首参收形）合并后，暴露 1 处**语义冲突**（文本零冲突，`git merge` 自动并入）。

- 冲突点：`crates/ulua-vm/src/functions/lua_b_tonumber.rs`。v29 把该核心降为安全 `fn`
  且首参成真实 `&mut LuaState` 后，p28 锚定形的 `l.check_bytes(1)` 出参与紧随其后的
  `l.arg_check(..)` 两个 `&mut` 借用重叠 → **E0499**。探针当年看不到此错，正因为
  该点位当时还是裸 `*mut LuaState`——即本文件 §二「裸 `&mut *l` 重叠 rustc 不查」的镜像
  后果：收形把 rustc 原本盲视的窗口重叠**变成编译错误**。
- 消解：按本判据 A 类**二次派窗**（非 (ptr,len) 裸快照，零新增 unsafe）——首次
  `l.check_bytes(1).len()` 仅作 cpp `luaL_checkstring` 的先位抛错件（"string expected"
  必先于 "base out of range"，可观察序逐位不变），借用止于取长；`arg_check` 落定后二次
  派窗直达解析。长度快照同时充当 `memchr` 无 NUL 时的 `unwrap_or` 全长，与旧
  `bytes.len()` 同值。
- 复验门禁（合并态，全在 worktree 只读跑）：clippy `-D warnings -W
  clippy::absolute_paths` RC=0、`cargo +nightly fmt --check` RC=0、`./test.sh`
  **6575 单测 + 111 conformance 全绿**（探针侧无 test.sh 权限，此为本判据首次全量实证）。
- 推论入档：**锚定形与「首参收形」互为放大**——收形越广，锚定形的借用冲突越早暴露；
  后续 vm 门面锚定票应与同族收形票同批复验，不可各自在旧基线上声称零冲突。

