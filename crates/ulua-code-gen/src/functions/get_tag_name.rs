use ulua_vm::enums::lua_type::LuaType;

const LUA_TNIL: u8 = LuaType::Nil as u8;
const LUA_TBOOLEAN: u8 = LuaType::Boolean as u8;
const LUA_TLIGHTUSERDATA: u8 = LuaType::LightUserData as u8;
const LUA_TNUMBER: u8 = LuaType::Number as u8;
const LUA_TINTEGER: u8 = LuaType::Integer as u8;
const LUA_TVECTOR: u8 = LuaType::Vector as u8;
const LUA_TSTRING: u8 = LuaType::String as u8;
const LUA_TTABLE: u8 = LuaType::Table as u8;
const LUA_TFUNCTION: u8 = LuaType::Function as u8;
const LUA_TUSERDATA: u8 = LuaType::UserData as u8;
const LUA_TTHREAD: u8 = LuaType::Thread as u8;
const LUA_TBUFFER: u8 = LuaType::Buffer as u8;
const LUA_TCLASS: u8 = LuaType::Class as u8;
const LUA_TOBJECT: u8 = LuaType::Object as u8;
const LUA_TDEADKEY: u8 = LuaType::DeadKey as u8;
const LUA_TPROTO: u8 = LuaType::Proto as u8;
const LUA_TUPVAL: u8 = LuaType::Upval as u8;

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
