//! lua 接口生成函数
//! 
struct LuaApi{
    ///函数名
    fn_name: String, 
    ///参数(类型,名字)
    parameters: Vec<(String, String)>,
    ///返回值类型
    results: Vec<(String, String)>
}

static mut LUA_APIS: Vec<LuaApi> = Vec::new();
///根据lua的参数和函数,生成对应的lua接口文件
pub fn generate() -> anyhow::Result<Vec<String>>{
    let mut result = Vec::new();
    result.push(format!(r#"--[[
    auto generated LuaNative Api
]]
local LuaNative = {{}}
    "#));//, chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()));
    for api in unsafe{&LUA_APIS} {
        result.push(format!("---Wrapper of {}", api.fn_name));
        for (pt, name) in &api.parameters {
            result.push(format!("---@param {} {}", name, pt));
        }
        for (rt, name) in &api.results {
            result.push(format!("---@return {} {}", rt, name));
        }
        result.push(format!("{} = {} or function({})", api.fn_name, api.fn_name, api.parameters.iter().map(|t| t.1.clone()).collect::<Vec<_>>().join(", ")));
        result.push(format!("\tLogError('{} not found')", api.fn_name));
        result.push(format!("end"));
    }
    result.push(format!(r#"---native 接口测试
function LuaNative:Test()
    CaSetRustLogLevel(4);
    CaMainUpdate()
    Log("LuaNative CaIsBaseDir", CaIsBaseDir(1, 0))
    local x, y = CaExpInverseDirection(1, 0)
    Log("LuaNative CaExpInverseDirection", x, y)
    Log("LuaNative CaExpCreateMap", CaExpCreateMap(10, 10, {{11, 42, 32, 14, 56}}, 5))
    local tb = {{}}
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
return LuaNative"#));
    Ok(result)
}
pub fn clear(){
    unsafe{
        LUA_APIS.clear();
    }
}
///添加lua接口
pub fn push(fn_name: &str, parameters: Vec<(String, String)>, results: Vec<(String, String)>){
    unsafe{
        LUA_APIS.push(LuaApi{fn_name: fn_name.to_string(), parameters, results})
    }
}