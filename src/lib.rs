#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! # LuaJIT 2.1
//!
//! <http://luajit.org>
//!
//! <http://www.lua.org/manual/5.1/manual.html>
//!
//! ## Performance considerations
//!
//! The _Not Yet Implemented_ guide documents which language features will be JIT compiled
//! into native machine code.
//!
//! <http://wiki.luajit.org/NYI>

mod ffi;
pub use ffi::*;

use core::ptr;

// These are defined as macros

/// <https://www.lua.org/manual/5.1/manual.html#lua_pop>
#[inline]
pub unsafe fn lua_pop(L: *mut lua_State, idx: libc::c_int) {
    lua_settop(L, -(idx) - 1)
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_newtable>
#[inline]
pub unsafe fn lua_newtable(L: *mut lua_State) {
    lua_createtable(L, 0, 0)
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_register>
#[inline]
pub unsafe fn lua_register(L: *mut lua_State, name: *const libc::c_char, f: lua_CFunction) {
    lua_pushcfunction(L, f);
    lua_setglobal(L, name);
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_pushcfunction>
#[inline]
pub unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(L, f, 0);
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_strlen>
#[inline]
pub unsafe fn lua_strlen(L: *mut lua_State, idx: libc::c_int) -> usize {
    lua_objlen(L, idx)
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_isfunction>
#[inline]
pub unsafe fn lua_isfunction(L: *mut lua_State, idx: libc::c_int) -> libc::c_int {
    (lua_type(L, idx) == LUA_TFUNCTION as i32) as i32
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_istable>
#[inline]
pub unsafe fn lua_istable(L: *mut lua_State, idx: libc::c_int) -> libc::c_int {
    (lua_type(L, idx) == LUA_TTABLE as i32) as i32
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_islightuserdata>
#[inline]
pub unsafe fn lua_islightuserdata(L: *mut lua_State, idx: libc::c_int) -> libc::c_int {
    (lua_type(L, idx) == LUA_TLIGHTUSERDATA as i32) as i32
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_isnil>
#[inline]
pub unsafe fn lua_isnil(L: *mut lua_State, idx: libc::c_int) -> libc::c_int {
    (lua_type(L, idx) == LUA_TNIL as i32) as i32
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_isboolean>
#[inline]
pub unsafe fn lua_isboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int {
    (lua_type(L, idx) == LUA_TBOOLEAN as i32) as i32
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_isthread>
#[inline]
pub unsafe fn lua_isthread(L: *mut lua_State, idx: libc::c_int) -> libc::c_int {
    (lua_type(L, idx) == LUA_TTHREAD as i32) as i32
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_isnone>
#[inline]
pub unsafe fn lua_isnone(L: *mut lua_State, idx: libc::c_int) -> libc::c_int {
    (lua_type(L, idx) == LUA_TNONE as i32) as i32
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_isnoneornil>
#[inline]
pub unsafe fn lua_isnoneornil(L: *mut lua_State, idx: libc::c_int) -> libc::c_int {
    (lua_type(L, idx) <= 0) as i32
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_pushliteral>
#[inline]
pub unsafe fn lua_pushliteral(L: *mut lua_State, s: &str) {
    lua_pushlstring(L, s.as_ptr() as _, s.len() as _);
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_setglobal>
#[inline]
pub unsafe fn lua_setglobal(L: *mut lua_State, k: *const libc::c_char) {
    lua_setfield(L, LUA_GLOBALSINDEX, k);
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_getglobal>
#[inline]
pub unsafe fn lua_getglobal(L: *mut lua_State, k: *const libc::c_char) {
    lua_getfield(L, LUA_GLOBALSINDEX, k)
}

/// <https://www.lua.org/manual/5.1/manual.html#lua_tostring>
#[inline]
pub unsafe fn lua_tostring(L: *mut lua_State, idx: libc::c_int) -> *const libc::c_char {
    lua_tolstring(L, idx, ptr::null_mut())
}

// Additional compatibility items that are defined as macros

/// `luaL_newstate()`
#[inline]
#[deprecated(since = "Lua 5.1", note = "replace with `luaL_newstate()`")]
pub unsafe fn lua_open() -> *mut lua_State {
    luaL_newstate()
}

/// `lua_pushvalue(L, LUA_REGISTRYINDEX)`
#[inline]
#[deprecated(
    since = "Lua 5.1",
    note = "replace with `lua_pushvalue(L, LUA_REGISTRYINDEX)`"
)]
pub unsafe fn lua_getregistry(L: *mut lua_State) {
    lua_pushvalue(L, LUA_REGISTRYINDEX)
}

/// `lua_gc(L, LUA_GCCOUNT as _, 0)`
#[inline]
#[deprecated(
    since = "Lua 5.1",
    note = "replace with `lua_gc(L, LUA_GCCOUNT as _, 0)`"
)]
pub unsafe fn lua_getgccount(L: *mut lua_State) -> libc::c_int {
    lua_gc(L, LUA_GCCOUNT as _, 0)
}

/// `lua_Reader`
#[deprecated(since = "Lua 5.1", note = "replace with `lua_Reader`")]
pub type lua_Chunkreader = lua_Reader;

/// `lua_Writer`
#[deprecated(since = "Lua 5.1", note = "replace with `lua_Writer`")]
pub type lua_Chunkwriter = lua_Writer;
