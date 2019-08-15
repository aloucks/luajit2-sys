use luajit_sys as sys;

#[test]
fn run_script() {
    use std::ffi::CStr;
    use std::ptr;

    unsafe {
        let lua = sys::luaL_newstate();
        assert_ne!(lua, ptr::null_mut());
        sys::luaL_openlibs(lua);
        let script_data = b"return 1 + 2";
        let script_name = b"run_script\0";
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
        println!("lua_gettop = {}", idx);

        let s = sys::lua_tostring(lua, idx);
        assert_ne!(s, ptr::null(), "lua_tostring returned null");

        let result = CStr::from_ptr(s).to_string_lossy().to_string();
        sys::lua_close(lua);

        assert_eq!("3", result);
    }
}
