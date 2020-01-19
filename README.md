# Rust LuaJIT 2 Bindings

[![crates.io](https://img.shields.io/crates/v/luajit2-sys.svg)](https://crates.io/crates/luajit2-sys)
[![docs.rs](https://docs.rs/luajit2-sys/badge.svg)](https://docs.rs/luajit2-sys)
[![build](https://dev.azure.com/aloucks/aloucks/_apis/build/status/aloucks.luajit2-sys?branchName=master)](https://dev.azure.com/aloucks/aloucks/_build/latest?definitionId=3&branchName=master)

```toml
[dependencies]
luajit2-sys = "0.0.1"
```

## Exported Cargo Environment Variables

|||
| -- | -- |
| `DEP_LUAJIT_INCLUDE`  | Path to the LuaJIT source and headers |
| `DEP_LUAJIT_LIB_NAME` | Platform specfic lib name (`lua51` on Windows and `luajit` everywhere else) |

## Example

```rust
use luajit2_sys as sys;
use std::ffi::CStr;

fn main() {
    unsafe {
        let lua = sys::luaL_newstate();
        sys::luaL_openlibs(lua);
        let script_data = b"return 1 + 2";
        let script_name = b"run_script\0";
        sys::luaL_loadbuffer(
            lua,
            script_data.as_ptr() as _,
            script_data.len() as _,
            script_name.as_ptr() as _,
        );
        sys::lua_pcall(lua, 0, 1, 0);
        let idx = sys::lua_gettop(lua);
        let s = sys::lua_tostring(lua, idx);
        let result = CStr::from_ptr(s).to_string_lossy().to_string();
        sys::lua_close(lua);

        println!("result: {}", result);
    }
}
```

