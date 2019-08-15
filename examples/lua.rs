use std::env;
use std::ffi::{CStr, CString};
use std::ptr;

use luajit_sys as sys;

unsafe fn run_script(script_name: String, script_src: String) {
    let lua = sys::luaL_newstate();
    assert_ne!(lua, ptr::null_mut());
    sys::luaL_openlibs(lua);
    let script_data = script_src.as_bytes();
    let script_name = CString::new(script_name).unwrap();
    let mut error = sys::luaL_loadbuffer(
        lua,
        script_data.as_ptr() as _,
        script_data.len() as _,
        script_name.as_ptr() as _,
    );
    if error != 0 {
        eprintln!("luaL_loadbuffer failed");
    } else {
        error = sys::lua_pcall(lua, 0, 1, 0);
        if error != 0 {
            eprintln!("lua_pcall failed");
        }
    }
    let idx = sys::lua_gettop(lua);
    if sys::lua_isnoneornil(lua, idx) != 1 {
        let s = sys::lua_tostring(lua, idx);
        assert_ne!(s, ptr::null(), "lua_tostring returned null");
        let result = CStr::from_ptr(s).to_string_lossy().to_string();
        println!("script result: {}", result);
    }
    sys::lua_close(lua);
}

fn main() {
    if let Some(script_name) = env::args().skip(1).next() {
        let script_src = std::fs::read_to_string(&script_name)
            .unwrap_or_else(|e| panic!("failed to read file: '{}' {:?}", &script_name, e));
        unsafe {
            run_script(script_name, script_src);
        }
    } else {
        println!(
            "{} FILE",
            env::current_exe()
                .unwrap()
                .file_name()
                .unwrap()
                .to_string_lossy()
        );
    }
}
