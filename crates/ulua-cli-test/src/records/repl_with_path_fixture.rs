extern crate alloc;

use alloc::string::String;
use core::ffi::c_void;

use ulua_vm::functions::{
  lua_close::lua_close, lua_l_newstate::lua_l_newstate, lua_l_sandboxthread::lua_l_sandboxthread,
};

use crate::functions::{run_code::run_code, setup_state::setup_state};

pub struct ReplWithPathFixture {
  pub lua_state: *mut c_void,
  pub l: *mut c_void,
  pub pretty_print_source: String,
}

impl Drop for ReplWithPathFixture {
  fn drop(&mut self) {
    if !self.l.is_null() {
      unsafe {
        lua_close(self.l as *mut _);
      }
    }
  }
}

const PRETTY_PRINT_SOURCE: &str = r#"
-- Accumulate pretty printer output in `capturedoutput`
capturedoutput = ""

function arraytostring(arr)
    local strings = {}
    table.foreachi(arr, function(k,v) table.insert(strings, pptostring(v)) end )
    return "{" .. table.concat(strings, ", ") .. "}"
end

function pptostring(x)
    if type(x) == "table" then
        -- Just assume array-like tables for now.
        return arraytostring(x)
    elseif type(x) == "string" then
        return '"' .. x .. '"'
    else
        return tostring(x)
    end
end

-- Note: Instead of calling print, the pretty printer just stores the output
-- in `capturedoutput` so we can check for the correct results.
function _PRETTYPRINT(...)
    local args = table.pack(...)
    local strings = {}
    for i=1, args.n do
        local item = args[i]
        local str = pptostring(item, customoptions)
        if i == 1 then
            capturedoutput = capturedoutput .. str
        else
            capturedoutput = capturedoutput .. "\t" .. str
        end
    end
end
"#;

impl ReplWithPathFixture {
  pub fn new() -> Self {
    let l_state = lua_l_newstate();
    let l = l_state as *mut c_void;

    setup_state(l_state);
    unsafe {
      lua_l_sandboxthread(l_state);
    }

    let pretty_print_source = String::from(PRETTY_PRINT_SOURCE);
    unsafe {
      run_code(l_state, &pretty_print_source);
    }

    Self {
      lua_state: l,
      l,
      pretty_print_source,
    }
  }
}

impl Default for ReplWithPathFixture {
  fn default() -> Self {
    Self::new()
  }
}
