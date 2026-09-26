//! Faithful port of Luau `Lexeme` (`Ast/include/Luau/Lexer.h`).
//!
//! The token payload is a C++ union (`const char* data`/`name`, `unsigned
//! codepoint`); a Rust `union` reproduces it. Unions can derive `Clone`/`Copy`
//! but not `Debug`, so `LexemeData` gets a hand-written `Debug` (it can't know
//! which arm is active) and `Lexeme` then derives `Debug` normally.

use alloc::{borrow::Cow, string::String};
use core::{
  fmt::{Debug, Display, Formatter, Result},
  ptr::null,
  slice::from_raw_parts,
};

use ulua_common::LUAU_ASSERT;

use crate::{
  enums::type_lexer::Type,
  functions::find_confusable::find_confusable,
  records::{ast_name::AstName, location::Location},
};

/// `Lexeme::QuoteStyle` (`Ast/include/Luau/Lexer.h`) — the delimiter of a quoted
/// string token, returned by `get_quote_style`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuoteStyle {
  Single,
  Double,
}

#[derive(Debug, Clone, Copy)]
pub struct Lexeme {
  pub r#type: Type,
  pub location: Location,
  pub(crate) length: u32,
  pub data: LexemeData,
}

impl Lexeme {
  /// 不带负载的词素（`length = 0`，负载臂取 [`LexemeData::EMPTY`]）。
  pub fn new(location: Location, r#type: Type) -> Lexeme {
    Lexeme {
      r#type,
      location,
      length: 0,
      data: LexemeData::EMPTY,
    }
  }

  /// A single-character token: `type = static_cast<Type>(static_cast<unsigned
  /// char>(character))`, so the token type is the raw byte value (`< 256`).
  pub fn from_char(location: Location, character: char) -> Lexeme {
    Lexeme {
      r#type: Type((character as u8) as i32),
      location,
      length: 0,
      data: LexemeData::EMPTY,
    }
  }

  /// A token with a `data`/`length` payload (string, number, comment, ...).
  ///
  /// cpp 的 `(const char* data, size_t size)` 成对入参折为单一切片：指针与长度
  /// 不再可漂移。字节保真是硬约束（[`LexemeData`] 文档）——`data` 臂照 cpp 只存
  /// 区间起始地址、`length` 照存 `size`，不解释编码、不增删 NUL 终止语义；切片
  /// 源指向源缓冲，按 `records::lexer::Lexer` 的契约（源缓冲活过解析会话、词素
  /// 载荷同款 cpp `&buffer[startOffset]`）比本词素长寿。
  pub fn with_data(location: Location, r#type: Type, data: &[u8]) -> Lexeme {
    LUAU_ASSERT!(r#type.has_data_payload());

    Lexeme {
      r#type,
      location,
      length: data.len() as u32,
      // 位拷贝契约：联合体臂存地址值（cpp `data(data)` 同款），任何指针位模式
      // 合法；此处只透传 `as_ptr()`，不构造也不解引用引用。
      data: LexemeData {
        data: data.as_ptr(),
      },
    }
  }

  /// A name/attribute/reserved-word token: the `name` union arm, `length = 0`.
  ///
  /// 参数取 [`AstName`] 而非 `&[u8]`：cpp `Lexeme(location, type, const char*
  /// name)` 的 name 实参就是名表驻留串的指针身份（`read_name` 返回值、静态空名
  /// `""`、或 read_names=false 未命中时的 **null** 名——`Lexeme::name()` 与各
  /// parser 判定点依赖 null 与空串的区分），切片既无法表达 null、又会在
  /// 驻留串上白白重新 strlen；union 位拷贝契约（[`LexemeData`] 文档）钉死臂为
  /// 裸指针，本入口只透传地址，逐位 ⇔ cpp。
  pub fn with_name(location: Location, r#type: Type, name: AstName) -> Lexeme {
    LUAU_ASSERT!(
      r#type == Type::NAME
        || r#type == Type::ATTRIBUTE
        || (r#type >= Type::RESERVED_BEGIN && r#type < Type::RESERVED_END_TOKEN)
    );

    Lexeme {
      r#type,
      location,
      length: name.len,
      data: LexemeData { name: name.value },
    }
  }

  pub fn get_block_depth(&self) -> u32 {
    LUAU_ASSERT!(self.r#type == Type::RAW_STRING || self.r#type == Type::BLOCK_COMMENT);

    // Safety: 上方断言 r#type ∈ {RAW_STRING, BLOCK_COMMENT}，两类词法均将载荷写入
    // union 的 data 分支，读取当前活跃变体合法。
    let data_ptr = unsafe { self.data.data };
    let length = self.length as usize;

    // If we have a well-formed string, we are guaranteed to see 2 `]` characters after the end of the string contents
    // Safety: data 是 read_long_string 写入的源缓冲内载荷指针、length 止于内容末尾，
    // 形如 [==[...]==] 的闭串首 ] 紧跟 length 之后，读界内且只读。
    // ⇔ cpp Lexer.cpp:319/324 `*(data + length) == ']'`：char 与 u8 同字节域，比较同值
    LUAU_ASSERT!(unsafe { *data_ptr.add(length) } == b']');

    let mut depth: u32 = 0;
    loop {
      depth += 1;
      // Safety: 循环仅扫描 length 之后源缓冲内的连续 ] 区（闭串），读界内且只读。
      if unsafe { *data_ptr.add(length + depth as usize) } == b']' {
        break;
      }
    }

    depth - 1
  }

  pub fn get_length(&self) -> u32 {
    LUAU_ASSERT!(self.r#type.has_data_payload());

    self.length
  }

  pub fn get_quote_style(&self) -> QuoteStyle {
    LUAU_ASSERT!(self.r#type == Type::QUOTED_STRING);

    // If we have a well-formed string, we are guaranteed to see a closing delimiter after the string
    let data_ptr = unsafe {
      // Safety: union 臂读取本身恒安全（LexemeData 各臂均为指针、无有效式不变量）；类型前置为 QUOTED_STRING 时 data 臂是 read_quoted_string 写入的源缓冲内指针，非空由 with_data 构造路径保证（断言兜底）。
      self.data.data
    };
    LUAU_ASSERT!(!data_ptr.is_null());

    let quote = unsafe {
      // Safety: 形如 \x22content\x22 的词素 length 不含闭引号，data_ptr+length 落在源缓冲内的闭引号字节（read_quoted_string 消费闭引号后回退 length 语义），界内单字节读；u8 → char 逐位保真（cpp `*(data + length)` 按 char 比较同值）。
      *data_ptr.add(self.length as usize) as char
    };
    if quote == '\'' {
      return QuoteStyle::Single;
    } else if quote == '"' {
      return QuoteStyle::Double;
    }

    LUAU_ASSERT!(false);
    QuoteStyle::Double // unreachable, but required due to compiler warning
  }

  #[inline]
  pub fn name_is(&self, rhs: &str) -> bool {
    self.r#type == Type::NAME && self.name() == rhs
  }

  /// 返回 token 名称的字符串形态，对应 C++ `Lexeme::Type::toString()` 用法。
  /// 报告错误时以零位置构造一个不带负载的占位词素做显示。
  #[inline]
  pub fn type_display_name(t: Type) -> String {
    use alloc::string::ToString;
    Lexeme::new(Location::default(), t).to_string()
  }

  /// Returns the name payload as an `AstName`.
  #[inline]
  pub fn name(&self) -> AstName {
    AstName {
      value: unsafe {
        // Safety: 联合体位拷贝读（data/name 两臂同为 *const u8、同偏移，任意位模式合法，读本身不可能 UB）；与 cpp Lexeme::name() 同款直读，指针语义有效性由词素类型约定承载（NAME/RESERVED 词素由 lexer 以 with_name 写入 name 臂），各调用点（parser/lexer）均在类型判定后进入。
        self.data.name
      },
      len: self.length,
    }
  }

  /// STRING/NUMBER/COMMENT 族词素的字节负载（源缓冲中的原始切片，**不保证**
  /// UTF-8：`"\xFF"` 之类的字面量按 cpp 词法器就是逐字节存的）；空指针返回
  /// `None`（cpp 同款判空）。文本呈现口径收在本文件的
  /// `render_payload_bytes`（口径唯一）。
  ///
  /// 本函数是 `data` 臂「指针 + `length` 裸字节区间」→ 切片的**全仓唯一收口**
  /// （以切片为界的门面）：变体族判定内置（原 `pub(crate) unsafe fn` 的调用点
  /// 契约收编），非负载变体一律 `None`，联合体活跃成员证明不外溢。注意变体集
  /// 与 [`Lexeme::get_length`](Self::get_length) 的断言集不同
  /// （不含 BLOCK_COMMENT/BROKEN_INTERP_DOUBLE_BRACE，对齐 cpp `toString` 的
  /// `%.*s` 分支族），消费点（to_string/parser_next_lexeme/parse_number）均经
  /// 各自类型判定后进入。
  ///
  /// 与 [`Lexeme::name`] 同理，这里只能对联合体做位拷贝读：臂本体是裸指针
  /// （批 2 收口后为 `*const u8`），存活前提即 [`LexemeData::EMPTY`] 文档所记
  /// 契约——词法器成对写入的 `[ptr, ptr + length)` 指向比词素长寿的源缓冲
  /// （records/lexer.rs）。
  pub(crate) fn data_bytes(&self) -> Option<&[u8]> {
    // 联合体 `data` 仅在下列变体下以 `data` 指针为活跃成员（词法器成对写入）
    if !matches!(
      self.r#type,
      Type::RAW_STRING
        | Type::QUOTED_STRING
        | Type::INTERP_STRING_BEGIN
        | Type::INTERP_STRING_MID
        | Type::INTERP_STRING_END
        | Type::INTERP_STRING_SIMPLE
        | Type::NUMBER
        | Type::COMMENT
    ) {
      return None;
    }
    // Safety: 上方变体判定即原函数契约的内置兑现——此时 `data.data` 是活跃
    // 且已初始化的联合体成员，直读其值合法。
    let ptr = unsafe { self.data.data };
    if ptr.is_null() {
      return None;
    }
    // Safety: 非空指针与 `length` 均由词法器成对写入，指向源缓冲内
    // `[ptr, ptr + length)` 的存活字节；此处只建切片，不解释编码。
    Some(unsafe { from_raw_parts(ptr, self.length as usize) })
  }
}

/// 词法单元是否为 NAME 且名字等于 `rhs`，对应 C++ 反复出现的
/// `lexeme.type == Lexeme::Name && AstName(lexeme.data.name) == "x"`。
/// NAME 判型先行短路，`data.name` 联合体仅在 NAME 时读 active 臂。
/// 接受 `&Lexeme`：实时态传 `parser.lexer.current()`，快照态传局部拷贝。
#[inline]
pub fn lexeme_name_is(lexeme: &Lexeme, rhs: &str) -> bool {
  lexeme.name_is(rhs)
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union LexemeData {
  pub data: *const u8,
  pub name: *const u8,
  pub codepoint: u32,
}

impl LexemeData {
  /// 「无负载」词素（cpp `Lexeme` 默认构造下的 `nullptr` 臂）的唯一构造点。
  ///
  /// 为什么本联合体只能存裸指针：`data` 臂在 NUMBER/COMMENT/RAW_STRING 等词素上是
  /// 「源缓冲指针 + `Lexeme::length`」的裸字节区间，区间末没有 NUL 终止、内容可为任意
  /// 字节，故 `&CStr`/`&str` 这类自带不变量的类型不成立；`codepoint` 臂又与指针臂逐位
  /// 重叠，指针的 niche 会被合法码点值占用。有没有负载、读哪一臂都由 `Lexeme::r#type`
  /// 决定，`null` 只表示该词素不带负载。
  pub const EMPTY: Self = Self { data: null() };
}

impl Debug for LexemeData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    // The active arm is determined by `Lexeme::type`; print opaquely.
    f.write_str("LexemeData(..)")
  }
}

impl Default for LexemeData {
  fn default() -> Self {
    Self::EMPTY
  }
}

/// 词素字节负载 → 可写进 `Formatter` 的文本，全文件唯一口径入口。
#[inline]
fn render_payload_bytes(s: &[u8]) -> Cow<'_, str> {
  String::from_utf8_lossy(s)
}

const K_RESERVED: [&str; 21] = [
  "and", "break", "do", "else", "elseif", "end", "false", "for", "function", "if", "in", "local",
  "nil", "not", "or", "repeat", "return", "then", "true", "until", "while",
];

const _: () =
  assert!(K_RESERVED.len() == (Type::RESERVED_END_TOKEN.0 - Type::RESERVED_BEGIN.0) as usize);

const fn static_symbol(ty: Type) -> Option<&'static str> {
  match ty {
    Type::EOF => Some("<eof>"),
    Type::EQUAL => Some("'=='"),
    Type::LESS_EQUAL => Some("'<='"),
    Type::GREATER_EQUAL => Some("'>='"),
    Type::NOT_EQUAL => Some("'~='"),
    Type::DOT2 => Some("'..'"),
    Type::DOT3 => Some("'...'"),
    Type::SKINNY_ARROW => Some("'->'"),
    Type::DOUBLE_COLON => Some("'::'"),
    Type::FLOOR_DIV => Some("'//'"),
    Type::ADD_ASSIGN => Some("'+='"),
    Type::SUB_ASSIGN => Some("'-='"),
    Type::MUL_ASSIGN => Some("'*='"),
    Type::DIV_ASSIGN => Some("'/='"),
    Type::FLOOR_DIV_ASSIGN => Some("'//='"),
    Type::MOD_ASSIGN => Some("'%='"),
    Type::POW_ASSIGN => Some("'^='"),
    Type::CONCAT_ASSIGN => Some("'..='"),
    Type::COMMENT => Some("comment"),
    Type::ATTRIBUTE_OPEN => Some("'@['"),
    Type::BROKEN_STRING => Some("malformed string"),
    Type::BROKEN_COMMENT => Some("unfinished comment"),
    Type::BROKEN_INTERP_DOUBLE_BRACE => Some("'{{', which is invalid (did you mean '\\{{'?)"),
    _ => None,
  }
}

impl Display for Lexeme {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    if let Some(sym) = static_symbol(self.r#type) {
      return write!(f, "{sym}");
    }

    match self.r#type {
      Type::RAW_STRING | Type::QUOTED_STRING => match self.data_bytes() {
        Some(s) => write!(f, "\"{}\"", render_payload_bytes(s)),
        None => write!(f, "string"),
      },
      Type::INTERP_STRING_BEGIN => match self.data_bytes() {
        Some(s) => write!(f, "`{}{{", render_payload_bytes(s)),
        None => write!(f, "the beginning of an interpolated string"),
      },
      Type::INTERP_STRING_MID => match self.data_bytes() {
        Some(s) => write!(f, "}}{}{{", render_payload_bytes(s)),
        None => write!(f, "the middle of an interpolated string"),
      },
      Type::INTERP_STRING_END => match self.data_bytes() {
        Some(s) => write!(f, "}}{}`", render_payload_bytes(s)),
        None => write!(f, "the end of an interpolated string"),
      },
      Type::INTERP_STRING_SIMPLE => match self.data_bytes() {
        Some(s) => write!(f, "`{}`", render_payload_bytes(s)),
        None => write!(f, "interpolated string"),
      },
      Type::NUMBER => match self.data_bytes() {
        Some(s) => write!(f, "'{}'", render_payload_bytes(s)),
        None => write!(f, "number"),
      },

      Type::NAME | Type::ATTRIBUTE => {
        let name = self.name();
        if !name.is_null() {
          write!(f, "'{}'", name)
        } else if self.r#type == Type::NAME {
          write!(f, "identifier")
        } else {
          write!(f, "attribute")
        }
      }

      Type::BROKEN_UNICODE => {
        // Safety: 本分支 `r#type` 为 BROKEN_UNICODE，词法器在该变体下把码点写入联合体 `codepoint` 成员，
        // 故 `data.codepoint` 读取的是活跃且已初始化的成员，直读合法。
        let cp = unsafe { self.data.codepoint };
        if cp != 0 {
          if let Some(confusable) = find_confusable(cp) {
            write!(
              f,
              "Unicode character U+{:x} (did you mean '{}'?)",
              cp, confusable
            )
          } else {
            write!(f, "Unicode character U+{:x}", cp)
          }
        } else {
          write!(f, "invalid UTF-8 sequence")
        }
      }

      _ => {
        let type_val = self.r#type.0;
        if type_val < Type::CHAR_END.0 {
          write!(f, "'{}'", type_val as u8 as char)
        } else if (Type::RESERVED_BEGIN.0..Type::RESERVED_END_TOKEN.0).contains(&type_val) {
          let index = (type_val - Type::RESERVED_BEGIN.0) as usize;
          write!(f, "'{}'", K_RESERVED[index])
        } else {
          write!(f, "<unknown>")
        }
      }
    }
  }
}
