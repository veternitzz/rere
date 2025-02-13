-- Typedefs for rere_std_fs

--[[
    Reads the file at ``path`` and returns it's contents as a string.
]]
local function readFile(path: string): string return nil :: any end

--[[
    Writes ``contents`` to the file at ``path``.
]]
local function writeFile(path: string, contents: string | buffer): () end

--[[
    Removes the file at ``path``.
]]
local function removeFile(path: string): () end

--[[
    Removes the directory at ``path``.
]]
local function removeDir(path: string): () end

--[[
    Appends ``contents`` to the file at ``path``.
]]
local function appendFile(path: string, contents: string | buffer): () end

return {
    readFile = readFile,
    writeFile = writeFile,
    removeFile = removeFile,
    removeDir = removeDir,
    appendFile = appendFile
}