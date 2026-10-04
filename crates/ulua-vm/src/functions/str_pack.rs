//! Source: `VM/src/lstrlib.cpp:1486`

use core::{
  ffi::{c_double, c_float},
  mem::size_of,
};

use memchr::memchr;

use crate::{
  enums::k_option::KOption,
  functions::{
    copywithendian::copywithendian, getdetails::getdetails, getnum::FmtCursor,
    initheader::initheader, lua_l_addchar::lua_l_addchar, lua_l_addlstring::lua_l_addlstring,
    lua_l_buffinit::lua_l_buffinit, lua_l_checklstring::lua_l_checklstring_ref,
    lua_l_pushresult::lua_l_pushresult, packint::packint,
  },
  macros::{lua_lib_fn::lua_lib_fn, maxintsize::MAXINTSIZE},
  records::{ftypes::Ftypes, header::Header, lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

// review.md §7：本项无 crate 外消费，由 pub 收窄为 pub(crate)。
pub(crate) const LUAL_PACKPADBYTE: u8 = 0x00;

/// pack 落盘切片核（真实逻辑）：cpp `str_pack`（lstrlib.cpp:1486）的选项循环、
/// 对齐填充、溢出校验、错误抛出点与写入次序逐点对齐 oracle；`islittle`/截断/
/// 求值序零变更。
///
/// `fmt_bytes` 为 1 号实参格式串的 payload 切片（r12-w6e 入约：游标经
/// [`FmtCursor::from_slice`] 建，串尾读归一见 `getnum.rs`）；各串实参经
/// [`lua_l_checklstring_ref`] 取 payload 切片，借用自栈槽串体（Lua 串不可变且
/// 不被移动，本次调用内存活——同 ref 切片契约）。写侧全链经 strbuf 家族
/// `&mut` 形转发（w6c 收口），本核不读 `b.p` 游标，无跨扩容存活的旧窗。
///
/// w6e 诚实降级：形参已全为引用形（`l: &mut LuaState`/`fmt_bytes: &[u8]`），真实
/// 裸操作内核（`initheader`/`getdetails`/`packint`/strbuf 追加族，各自 `# Safety`
/// 契约）落逐句窄 `unsafe` 块；整块 `unsafe` 包裹消亡。
///
/// 调用序契约（正确性，非内存安全）：`l` 须为可抛错受保护帧内存活的 `LuaState`：
/// `initheader` 接线 `h.l` 后，`getdetails`/`arg_check` 报错内核经其解引用抛出并
/// unwind（不返回）；`lua_l_pushresult`/`packint` 的分配与 GC 义务由家族契约承载。
fn str_pack_ref(l: &mut LuaState, fmt_bytes: &[u8]) -> i32 {
  let mut b = LuaLStrbuf::new();
  let mut h = Header::default();
  let mut fmt = FmtCursor::from_slice(fmt_bytes); // format string
  let mut arg = 1; // current argument to pack
  let mut totalsize: usize = 0; // accumulate total size of result
  // SAFETY: `l` 由垫片契约保证存活独占驱动；`h` 为本地待初始化表头，`initheader`
  // 接线 `h.l` 至同一存活帧；`&mut h` 折裸形系就地转手（借用窗止于当句）
  unsafe { initheader(l, &mut h) };
  l.push_nil(); // mark to separate arguments from string buffer
  lua_l_buffinit(l, &mut b);
  while fmt.cur() != 0 {
    // SAFETY: `h` 已按 initheader 契约初始化、`fmt` 为本地游标、`totalsize` 为本轮
    // 累计偏移；报错内核经接线 `h.l` 解引用（上方调用序契约），发散不返回
    let (opt, size, ntoalign) = unsafe { getdetails(&mut h, totalsize, &mut fmt) };
    totalsize += (ntoalign + size) as usize;
    // 保留计数重复：ntoalign 是布局对齐算出的填充字节数，循环变量不参与取数，
    // 每轮仅向缓冲区追加同一个 pad 字节，无数据序列可迭代
    for _ in 0..ntoalign {
      // SAFETY: 缓冲 `b` 于 buffinit 后、pushresult 前仅经家族 &mut 形写读，
      // 游标不变式由家族维护（addchar 内部按需扩容）
      unsafe { lua_l_addchar(&mut b, LUAL_PACKPADBYTE) };
    }
    arg += 1;
    match opt {
      KOption::Kint | KOption::Kuint => {
        // 有/无符号双臂仅差溢出校验式与 packint 符号填充位
        let signed = opt == KOption::Kint;
        let n = l.check_number(arg) as i64;
        if size < size_of::<i64>() as i32 {
          // need overflow check?
          if signed {
            let lim = 1i64 << ((size * 8) - 1);
            l.arg_check(-lim <= n && n < lim, arg, "integer overflow");
          } else {
            l.arg_check((n as u64) < (1u64 << (size * 8)), arg, "unsigned overflow");
          }
        }
        // SAFETY: 家族契约——可分配、可 GC；`b` 处于有效态（同上不变式）
        unsafe {
          packint(
            &mut b,
            n as u64,
            h.islittle,
            size,
            if signed { (n < 0) as i32 } else { 0 },
          )
        };
      }
      KOption::Kfloat => {
        // floating-point options
        let mut u = Ftypes { n: 0.0 };
        let mut buff = [0u8; MAXINTSIZE as usize];
        let n = l.check_number(arg); // get argument
        // union 单写活跃变体——`size` 三选一确定写入字段（cpp 同形 union 用法，
        // LUAL_NUMSIZES 语义）；写侧为安全操作，仅下方 `u.buff` 读窗需窄块
        if size as usize == size_of::<c_float>() {
          u.f = n as c_float; // copy it into 'u'
        } else if size as usize == size_of::<c_double>() {
          u.d = n;
        } else {
          u.n = n;
        }
        // move 'u' to final result, correcting endianness if needed
        // SAFETY: `u.buff` 即上句单写活跃窗的字节视图（union 全体覆盖 buff，读回无歧义）
        let src = unsafe { &u.buff[..size as usize] };
        let (dst, src) = (&mut buff[..size as usize], src);
        copywithendian(dst, src, h.islittle);
        // SAFETY: 家族契约（同上）；dst 为本地界内窗
        unsafe { lua_l_addlstring(&mut b, dst) };
      }
      KOption::Kchar => {
        // fixed-size string（锚定形：先快照长度并过 argcheck，再二次派窗直达写入；
        // 1 号外的取参路径不改动 arg 槽，二次派窗取回同一串体，typeerror 重复即同
        // 消息同点位，观察序不变）
        let len = lua_l_checklstring_ref(l, arg).len();
        l.arg_check(len <= size as usize, arg, "string longer than given size");
        let s = lua_l_checklstring_ref(l, arg);
        // SAFETY: 家族契约（同上）；s 借用自栈槽串体，本次调用内界内存活
        unsafe { lua_l_addlstring(&mut b, s) }; // add string
        // 补零到定宽 size：循环变量不参与取数，纯计数重复
        //（原 `while len < size` 游走收为区间迭代；argcheck 已保证 len <= size）
        for _ in len..(size as usize) {
          // SAFETY: 家族契约（同 addchar 首点）
          unsafe { lua_l_addchar(&mut b, LUAL_PACKPADBYTE) };
        }
      }
      KOption::Kstring => {
        // strings with length count（同 Kchar：快照长度→argcheck→pack 长度→二次派窗）
        let len = lua_l_checklstring_ref(l, arg).len();
        l.arg_check(
          size >= size_of::<usize>() as i32 || len < (1usize << (size * 8)),
          arg,
          "string length does not fit in given size",
        );
        // SAFETY: 家族契约（同上）；len 为已校验的界内 usize
        unsafe { packint(&mut b, len as u64, h.islittle, size, 0) }; // pack length
        let s = lua_l_checklstring_ref(l, arg);
        // SAFETY: 家族契约（同上）；s 借用自栈槽串体
        unsafe { lua_l_addlstring(&mut b, s) };
        totalsize += len;
      }
      KOption::Kzstr => {
        // zero-terminated string（判定快照→argcheck→二次派窗写入）
        let s0 = lua_l_checklstring_ref(l, arg);
        let has_zero = memchr(0, s0).is_some();
        let len = s0.len();
        // 原 cstr_bytes strlen==len 的「无内嵌零」判定收为 memchr 单遍扫描
        l.arg_check(!has_zero, arg, "string contains zeros");
        let s = lua_l_checklstring_ref(l, arg);
        // SAFETY: 家族契约（同上）；s 已证无内嵌零、借用自栈槽串体
        unsafe {
          lua_l_addlstring(&mut b, s);
          // SAFETY: 家族契约——追加终止零字节（zstr 语义即 str + '\0'）
          lua_l_addchar(&mut b, 0); // add zero at the end
        }
        totalsize += len + 1;
      }
      KOption::Kpadding => {
        // SAFETY: 家族契约（同 addchar 首点）
        unsafe { lua_l_addchar(&mut b, LUAL_PACKPADBYTE) };
        arg -= 1; // undo increment
      }
      KOption::Kpaddalign | KOption::Knop => {
        arg -= 1; // undo increment
      }
    }
  }
  lua_l_pushresult(&mut b);
  1
}

/// 核心转发垫片（一行委托 [`str_pack_ref`]）：本面消费点仅 `lua_lib_fn!` 生成的
/// 注册臂 `str_pack_arm`（luaopen_string.rs:6 导入、:32 STRLIB "pack" 注册），
/// 全仓实测无其余消费面 ⇒ 保一行形（r12-w5s `byteoffset` 垫片先例）。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载，r16-v38 收形后
/// 本面唯一点名转手、无裸操作，降为安全 `fn`）：`l` 须处于可抛错受保护帧，栈槽 #1 为格式串
/// 实参（非串经 `check_bytes` 抛 "string expected"），其余义务单源 [`str_pack_ref`]。
/// 格式串窗口须与同句的核心调用（该核亦经 `&mut l` 压栈/报错）共存，p28 锚定形与 `&mut`
/// 接收者不可共存 ⇒ 按 r16-v29 桥接判例在块内一次就地转手裸句柄，借用窗止于本块。
/// cpp `lstrlib.cpp:1486`。
pub(crate) fn str_pack(l: &mut LuaState) -> i32 {
  // SAFETY: `l` 由 `&mut` 保证有效且独占，转手后的 `lp` 即同一存活帧；payload 切片借用自
  // 栈槽 #1 串体（不可变、不搬移），本次调用内有效
  unsafe {
    let lp = l.as_mut_ptr();
    str_pack_ref(&mut *lp, (*lp).check_bytes(1))
  }
}

lua_lib_fn!(pub(crate) fn str_pack @ref, str_pack_arm);
