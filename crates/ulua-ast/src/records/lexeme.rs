//! Faithful port of Luau `Lexeme` (`Ast/include/Luau/Lexer.h`).
//!
//! The token payload is a C++ union (`const char* data`/`name`, `unsigned
//! codepoint`). In Rust the `data`/`name` arms were bit-identical pointers, so
//! the payload is a plain struct of two named fields ([`LexemeData`]); only
//! the hand-written `Debug` remains (the field a reader takes is decided by
//! `Lexeme::r#type`, which `Debug` cannot see), and `Lexeme` derives `Debug`
//! normally.

use alloc::{borrow::Cow, string::String};
use core::{
  fmt::{Debug, Display, Formatter, Result},
  ptr::{NonNull, null},
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
  /// 不带负载的词素（`length = 0`，负载字段取 [`LexemeData::EMPTY`]）。
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
  /// 不再可漂移。字节保真是硬约束（[`LexemeData`] 文档）——`data` 字段照 cpp 只存
  /// 区间起始地址、`length` 照存 `size`，不解释编码、不增删 NUL 终止语义；切片
  /// 源指向源缓冲，按 `records::lexer::Lexer` 的契约（源缓冲活过解析会话、词素
  /// 载荷同款 cpp `&buffer[startOffset]`）比本词素长寿。
  pub fn with_data(location: Location, r#type: Type, data: &[u8]) -> Lexeme {
    LUAU_ASSERT!(r#type.has_data_payload());

    Lexeme {
      r#type,
      location,
      length: data.len() as u32,
      // 指针透传契约：`data` 字段存地址值（cpp `data(data)` 同款），引用切片
      // 恒非空指针（空切片亦为对齐的悬挂址），`NonNull::new` 只做非空性折叠、
      // 不构造也不解引用引用。
      data: LexemeData {
        data: NonNull::new(data.as_ptr().cast_mut()),
        codepoint: 0,
      },
    }
  }

  /// A name/attribute/reserved-word token: the `data` pointer field (cpp 的
  /// `name` 臂与之同槽同型), `length` 照存 `name.len`.
  ///
  /// 参数取 [`AstName`] 而非 `&[u8]`：cpp `Lexeme(location, type, const char*
  /// name)` 的 name 实参就是名表驻留串的指针身份（`read_name` 返回值、静态空名
  /// `""`、或 read_names=false 未命中时的 **null** 名——`Lexeme::name()` 与各
  /// parser 判定点依赖 null 与空串的区分），切片既无法表达 null、又会在
  /// 驻留串上白白重新 strlen；[`LexemeData`] 的指针字段契约钉死该字段为
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
      data: LexemeData {
        data: name.as_opt_ptr(),
        codepoint: 0,
      },
    }
  }

  /// 载荷区间后第 `offset` 字节的单点读取（本文件 `data` 指针字段的**唯一**
  /// 越出 `[ptr, ptr + length)` 的解引用面）。
  ///
  /// 消费形态：闭合定界符不落进内容区间——`[==[...]==]` 的首 `]` 在 `length`
  /// 处、连续 `]` 尾区在其后，`"..."` 的闭引号在 `length` 处（`read_quoted_string`
  /// 消费闭引号后回退 length 语义）。
  ///
  /// 契约（safe fn + 内部 unsafe，先例 `data_bytes`）：调用点均为闭合形态词素
  /// （`RAW_STRING`/`BLOCK_COMMENT`/`QUOTED_STRING`）且 `offset` 落在源缓冲内
  /// （定界符确实存在）；null 负载折叠为 `None`，形外输入不再外泄 UB。
  #[inline]
  fn byte_after_payload(&self, offset: usize) -> Option<u8> {
    // 缺席负载（`None`，cpp 判 `nullptr` 后不解引用的形态）直接短路为 `None`。
    let ptr = self.data.data?;
    // Safety: 上方契约——指针出自词法器成对写入、指向比词素长寿的源缓冲
    // （records/lexer.rs），读界内且只读；u8 与 cpp `char` 同字节域，
    // 与 `*(data + n)` 的比较逐位同值。
    Some(unsafe { *ptr.as_ptr().add(offset) })
  }

  pub fn get_block_depth(&self) -> u32 {
    LUAU_ASSERT!(self.r#type == Type::RAW_STRING || self.r#type == Type::BLOCK_COMMENT);

    // 上方断言 r#type ∈ {RAW_STRING, BLOCK_COMMENT}，两类词法均将载荷指针写入
    // LexemeData::data 字段。
    let length = self.length as usize;

    // If we have a well-formed string, we are guaranteed to see 2 `]` characters after the end of the string contents
    // ⇔ cpp Lexer.cpp:319/324 `*(data + length) == ']'`：闭串首 ] 紧跟 length 之后。
    LUAU_ASSERT!(self.byte_after_payload(length) == Some(b']'));

    let mut depth: u32 = 0;
    loop {
      depth += 1;
      // 循环仅扫描 length 之后源缓冲内的连续 ] 区（闭串），界内只读；
      // `None`（形外防御，cpp 该形态为解引用 UB）即截断扫描。
      match self.byte_after_payload(length + depth as usize) {
        Some(b']') => break,
        Some(_) => continue,
        None => {
          LUAU_ASSERT!(false);
          break;
        }
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
    // （QUOTED_STRING 的 data 字段是 read_quoted_string 写入的源缓冲内指针，
    // 非空性由 byte_after_payload 的判空折叠兜底）。

    // 闭引号在 length 处（界内单字节读，见 byte_after_payload 契约）：
    // 与 `*(data + length)` 的 `'`/`"` 比较逐位同值；`None`（null 负载，cpp
    // 该形态为解引用 UB）并入形外字节分支。
    match self.byte_after_payload(self.length as usize) {
      Some(b'\'') => QuoteStyle::Single,
      Some(b'"') => QuoteStyle::Double,
      // 形外字节即非法词素：cpp 以 `LUAU_ASSERT(false)` 拦下后仍返回 Double。
      _ => {
        LUAU_ASSERT!(false);
        QuoteStyle::Double
      }
    }
  }

  #[inline]
  fn name_is(&self, rhs: &str) -> bool {
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
    // 指针臂直读（cpp `data`/`name` 同槽同型，收口为 `data` 一字段）；有效性
    // 由词素类型约定承载（NAME/RESERVED 词素由 lexer 以 with_name 写入），
    // 各调用点（parser/lexer）均在类型判定后进入。
    AstName::from_opt_ptr(self.data.data, self.length)
  }

  /// STRING/NUMBER/COMMENT 族词素的字节负载（源缓冲中的原始切片，**不保证**
  /// UTF-8：`"\xFF"` 之类的字面量按 cpp 词法器就是逐字节存的）；空指针返回
  /// `None`（cpp 同款判空）。文本呈现口径收在本文件的
  /// `render_payload_bytes`（口径唯一）。
  ///
  /// 本函数是 `data` 指针字段「指针 + `length` 裸字节区间」→ 切片的**全仓唯一
  /// 收口**（以切片为界的门面）：变体族判定内置（原 `pub(crate) unsafe fn` 的
  /// 调用点契约收编），非负载变体一律 `None`，负载字段的有效性证明不外溢。注意
  /// 变体集与 [`Lexeme::get_length`](Self::get_length) 的断言集不同
  /// （不含 BLOCK_COMMENT/BROKEN_INTERP_DOUBLE_BRACE，对齐 cpp `toString` 的
  /// `%.*s` 分支族），消费点（to_string/parser_next_lexeme/parse_number）均经
  /// 各自类型判定后进入。
  ///
  /// 与 [`Lexeme::name`] 同理，这里直读 [`LexemeData`] 的裸指针字段：存活前提即
  /// [`LexemeData::EMPTY`] 文档所记契约——词法器成对写入的
  /// `[ptr, ptr + length)` 指向比词素长寿的源缓冲（records/lexer.rs）。
  ///
  /// `pub`：跨 crate 的负载读取也必须走这一收口（如 `ulua-config` 取 QUOTED_STRING
  /// 串），调用点不得自行 `data.as_ptr()` + 长度重拼裸区间再 `from_raw_parts`——
  /// 那等于把本函数内置的变体判定与存活契约外溢回业务侧（review.md §2）。
  pub fn data_bytes(&self) -> Option<&[u8]> {
    // `data` 指针字段仅在下列变体下由词法器成对写入为有效载荷区间
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
    // 缺席负载（cpp 同款判空）短路为 `None`。
    let ptr = self.data.data?;
    // Safety: 非空指针与 `length` 均由词法器成对写入，指向源缓冲内
    // `[ptr, ptr + length)` 的存活字节；此处只建切片，不解释编码。
    Some(unsafe { from_raw_parts(ptr.as_ptr().cast_const(), self.length as usize) })
  }
}

/// 词法单元是否为 NAME 且名字等于 `rhs`，对应 C++ 反复出现的
/// `lexeme.type == Lexeme::Name && AstName(lexeme.data.name) == "x"`。
/// NAME 判型先行短路，`data` 指针字段仅在 NAME 时按名字读取。
/// 接受 `&Lexeme`：实时态传 `parser.lexer.current()`，快照态传局部拷贝。
#[inline]
pub(crate) fn lexeme_name_is(lexeme: &Lexeme, rhs: &str) -> bool {
  lexeme.name_is(rhs)
}

/// 词素负载：cpp `Lexeme` 匿名 union（`const char* data`/`name` + `unsigned
/// codepoint`）的具名字段化形态。
///
/// cpp 里 `data` 与 `name` 本就是同一槽位的同型指针（union 别名），故收口为
/// 单一 `data` 字段；真正与指针复用的只有 `codepoint`（BROKEN_UNICODE 码点），
/// 改为独立具名字段后即无位复用——读哪一字段仍由 `Lexeme::r#type` 决定，
/// 各构造入口只写自己语义对应的字段（另一字段恒为 0/缺席占位）。
#[derive(Clone, Copy)]
pub struct LexemeData {
  /// 载荷/名字指针（cpp `data`/`name` 两臂的合并）；`None` 即 cpp `nullptr`
  /// 「本词素不带负载」态，缺席由类型表达、无 null 哨兵。
  pub data: Option<NonNull<u8>>,
  /// BROKEN_UNICODE 词素的码点（cpp `codepoint` 臂）。
  pub codepoint: u32,
}

impl LexemeData {
  /// 「无负载」词素（cpp `Lexeme` 默认构造下的 `nullptr` 臂）的唯一构造点。
  ///
  /// 为什么 `data` 字段只能存指针型而非引用：它在 NUMBER/COMMENT/RAW_STRING 等
  /// 词素上是「源缓冲指针 + `Lexeme::length`」的裸字节区间，区间末没有 NUL
  /// 终止、内容可为任意字节，故带内容不变量的引用型不成立。有没有负载、读哪一
  /// 字段都由 `Lexeme::r#type` 决定，`None` 只表示该词素不带负载。
  pub const EMPTY: Self = Self {
    data: None,
    codepoint: 0,
  };

  /// 指针身份桥（同 [`AstName::as_ptr`] 先例）：`None` 折回 `null`，供既有
  /// `c_slice`/extern 裸指针消费点透传地址值，不构成解引用许可。
  #[inline]
  pub fn as_ptr(&self) -> *const u8 {
    self.data.map_or(null(), |p| p.as_ptr().cast_const())
  }
}

impl Debug for LexemeData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    // The active field is determined by `Lexeme::type`; print opaquely.
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

/// 负载族词素的显示样板单点收口（cpp `toString` 的 `%.*s` 分支族逐条手抄 6 遍）：
/// 有字节负载则以 `prefix`/`suffix` 包裹 lossy 文本，无负载（词法器未写区间的
/// 防御形态，cpp 打空 `%.*s`）则落 `fallback` 描述串。
fn write_payload(
  f: &mut Formatter<'_>,
  data: Option<&[u8]>,
  prefix: &str,
  suffix: &str,
  fallback: &str,
) -> Result {
  match data {
    Some(s) => write!(f, "{prefix}{}{suffix}", render_payload_bytes(s)),
    None => f.write_str(fallback),
  }
}

/// 21 个保留字：全 crate 单源关键字表（下标即 `Type::RESERVED_BEGIN + i`）。
/// 消费方两处——本文件 [`Display`] 的词素名打印、`AstNameTable::new` 的保留字
/// 登记（`&'static str` 直取 `as_bytes()`，无需第二份字节字面量）。
pub(crate) const K_RESERVED: [&str; 21] = [
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
      Type::RAW_STRING | Type::QUOTED_STRING => {
        write_payload(f, self.data_bytes(), "\"", "\"", "string")
      }
      Type::INTERP_STRING_BEGIN => write_payload(
        f,
        self.data_bytes(),
        "`",
        "{",
        "the beginning of an interpolated string",
      ),
      Type::INTERP_STRING_MID => write_payload(
        f,
        self.data_bytes(),
        "}",
        "{",
        "the middle of an interpolated string",
      ),
      Type::INTERP_STRING_END => write_payload(
        f,
        self.data_bytes(),
        "}",
        "`",
        "the end of an interpolated string",
      ),
      Type::INTERP_STRING_SIMPLE => {
        write_payload(f, self.data_bytes(), "`", "`", "interpolated string")
      }
      Type::NUMBER => write_payload(f, self.data_bytes(), "'", "'", "number"),

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
        // 本分支 `r#type` 为 BROKEN_UNICODE，词法器在该变体下把码点写入
        // LexemeData::codepoint 字段。
        let cp = self.data.codepoint;
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
