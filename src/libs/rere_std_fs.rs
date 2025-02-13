use tokio::fs;
use mlua::prelude::*;
use lune_utils::TableBuilder;
use bstr::{BString, ByteSlice};

pub fn module(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(luau)?
        .with_async_function("readFile", fs_read_file)?
        .with_async_function("writeFile", fs_write_file)?
        .with_async_function("writeDir", fs_write_dir)?
        .with_async_function("removeFile", fs_remove_file)?
        .with_async_function("removeDir", fs_remove_dir)?
        .with_async_function("appendFile", fs_append_file)?
        .build_readonly()
}

async fn fs_read_file(luau: &Lua, path: String) -> LuaResult<LuaString> {
    let bytes = fs::read(&path).await.into_lua_err()?;

    luau.create_string(bytes)
}

async fn fs_write_file(_: &Lua, (path, contents): (String, BString)) -> LuaResult<()> {
    fs::write(&path, contents.as_bytes()).await.into_lua_err()
}

async fn fs_write_dir(_: &Lua, path: String) -> LuaResult<()> {
    fs::create_dir_all(&path).await.into_lua_err()
}

async fn fs_remove_file(_: &Lua, path: String) -> LuaResult<()> {
    fs::remove_file(&path).await.into_lua_err()
}

async fn fs_remove_dir(_: &Lua, path: String) -> LuaResult<()> {
    fs::remove_dir_all(&path).await.into_lua_err()
}

async fn fs_append_file(_: &Lua, (path, contents): (String, BString)) -> LuaResult<()> {
    let file_contents = fs::read_to_string(&path).await.into_lua_err().to_owned()?;
    let owned_contents = &contents;
    let new_contents = format!("{file_contents}{owned_contents}");

    fs::write(&path, new_contents.as_bytes()).await.into_lua_err()
}