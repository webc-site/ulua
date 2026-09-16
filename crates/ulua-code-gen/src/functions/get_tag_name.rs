const LUA_TNIL: u8 = 0;
const LUA_TBOOLEAN: u8 = 1;
const LUA_TLIGHTUSERDATA: u8 = 2;
const LUA_TNUMBER: u8 = 3;
const LUA_TINTEGER: u8 = 4;
const LUA_TVECTOR: u8 = 5;
const LUA_TSTRING: u8 = 6;
const LUA_TTABLE: u8 = 7;
const LUA_TFUNCTION: u8 = 8;
const LUA_TUSERDATA: u8 = 9;
const LUA_TTHREAD: u8 = 10;
const LUA_TBUFFER: u8 = 11;
const LUA_TCLASS: u8 = 12;
const LUA_TOBJECT: u8 = 13;
const LUA_TDEADKEY: u8 = 14;
const LUA_TPROTO: u8 = 15;
const LUA_TUPVAL: u8 = 16;

pub(crate) fn get_tag_name(tag: u8) -> &'static str {
  match tag {
    LUA_TNIL => "tnil",
    LUA_TBOOLEAN => "tboolean",
    LUA_TLIGHTUSERDATA => "tlightuserdata",
    LUA_TNUMBER => "tnumber",
    LUA_TVECTOR => "tvector",
    LUA_TSTRING => "tstring",
    LUA_TTABLE => "ttable",
    LUA_TFUNCTION => "tfunction",
    LUA_TUSERDATA => "tuserdata",
    LUA_TTHREAD => "tthread",
    LUA_TBUFFER => "tbuffer",
    LUA_TPROTO => "tproto",
    LUA_TUPVAL => "tupval",
    LUA_TDEADKEY => "tdeadkey",
    LUA_TCLASS => "tclass",
    LUA_TOBJECT => "tobject",
    LUA_TINTEGER => "tinteger",
    _ => unreachable!("Unknown type tag"),
  }
}
