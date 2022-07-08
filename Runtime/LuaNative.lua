--[[
    auto generated LuaNative Api
]]
local LuaNative = {}
    
---Wrapper of CaReleaseVector
---@param ptr table
---@param size number
---@param type_size number
---@param cap number
CaReleaseVector = CaReleaseVector or function(ptr, size, type_size, cap)
	LogError('CaReleaseVector not found')
end
---Wrapper of CaSetRustLogLevel
---@param log_level number
---@return boolean result
CaSetRustLogLevel = CaSetRustLogLevel or function(log_level)
	LogError('CaSetRustLogLevel not found')
end
---Wrapper of CaTestRustLog
---@param level number
CaTestRustLog = CaTestRustLog or function(level)
	LogError('CaTestRustLog not found')
end
---native 接口测试
function LuaNative:Test()
    CaSetRustLogLevel(4);
    CaMainUpdate()
    Log("LuaNative CaIsBaseDir", CaIsBaseDir(1, 0))
    local x, y = CaExpInverseDirection(1, 0)
    Log("LuaNative CaExpInverseDirection", x, y)
    Log("LuaNative CaExpCreateMap", CaExpCreateMap(10, 10, {11, 42, 32, 14, 56}, 5))
    local tb = {}
    for i = 1, 120 do
        table.insert(tb, i)
    end
    Log("CaExpAddExploredPoints")
    --CaExpAddExploredPoints(tb, 120);
    local ret, point = CaExpFindPath(1, 1, 3, 4, 2)
    Log("LuaNative CaExpFindPath", ret);
    LogTable(point, "path")
    Log("LuaNative CaTimestampMills", CaTimestampMills())
end
Normalize(LuaNative)
return LuaNative