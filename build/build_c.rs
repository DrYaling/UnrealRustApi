//! 生成c++头文件

use std::{env, fs::{File}, io::{Write, Read}, path::PathBuf, collections::BTreeMap};

use once_cell::sync::Lazy;

use crate::if_else;
static mut DEF_NAMES: Vec<String> = Vec::new();

const EXPORT_API_NAME:&'static str = "UE_RUST_API_EXPORT";
const INTERFACE_API:&'static str = "UE_RUST_INTERFACE_API";

pub static PTR_CHECKER: Lazy<BTreeMap<String, String>> = Lazy::new(||{
    let map = 
    vec![
    ("*mut i32".into(),"int32_t*".into()), 
    ("*mut u32".into(), "uint32_t*".into()), 
    ("*mut u8".into(), "unsigned char*".into()), 
    ("*mut i8".into(), "char*".into()),
    ("*const i32".into(),"const int32_t*".into()), 
    ("*const u32".into(), "const uint32_t*".into()), 
    ("*const u8".into(), "unsigned const char*".into()), 
    ("*const i8".into(), "const char*".into()),
    ("*mut c_char".into(),"char*".into()), 
    ("*const c_char".into(),"const char*".into()), 
    ("*mut c_void".into(),"void*".into()),  
    ("*const c_void".into(),"void*".into()), 
    ].into_iter().collect();
    map
});
pub static RESULT_PTR_CHECKER: Lazy<BTreeMap<String, String>> = Lazy::new(||{
    let map = 
    vec![
    ("*mut i32".into(),"I32Ptr".into()), 
    ("*mut u32".into(), "U32Ptr".into()), 
    ("*const i32".into(),"CI32Ptr".into()), 
    ("*const u32".into(), "CU32Ptr".into()), 
    ("*mut u8".into(), "UCharPtr".into()), 
    ("*mut i8".into(), "CharPtr".into()),
    ("*const u8".into(), "UCCharPtr".into()), 
    ("*const i8".into(), "CCharPtr".into()),
    ("*mut c_char".into(),"CharPtr".into()), 
    ("*const c_char".into(),"CCharPtr".into()), 
    ("*mut c_void".into(),"CCharPtr".into()), 
    ("*const c_void".into(),"CCharPtr".into()), 
    ].into_iter().collect();
    map
});



pub fn build() -> anyhow::Result<()>{
    unsafe{ 
        DEF_NAMES.clear();
    }
    super::build_lua::clear();
    let root = std::path::PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let main_dir = root.as_path().display().to_string();
    let src_dir = dunce::canonicalize(root.join("src"))?;
    println!("src path: {:?}", src_dir);
    // let root_dir = src_dir.as_path().display().to_string();
    let mut file_list = Vec::new();
    load_file(src_dir, &mut file_list)?;
    parse_headers(file_list, main_dir)?;
    // panic!("exit with log");
    Ok(())
}
fn load_file(dir: PathBuf, file_list: &mut Vec<String>) -> anyhow::Result<()>{
    let files = std::fs::read_dir(dir)?;
    for path in files{  //for循环读出文件名，并且存入parse_files中
        let path = path?.path();
        let file = path.as_path().display().to_string();
        if file.contains(".rs"){
            println!("path file {}", file);  //目录dir下包含的文件有file...
            file_list.push(file);
        }
        else{
            println!("path dir {}", file);  //目录dir下包含的文件有file...
            load_file(path, file_list)?
        }
    }
    Ok(())
}
fn parse_headers(file_list: Vec<String>, root: String) -> anyhow::Result<()>{
    let mut header: Vec<String> = Vec::new();
    let mut cpp: Vec<String> = Vec::new();
    //(name, api)
    let mut lua_api: Vec<(String, Vec<String>)> = Vec::new();
    header.push("#pragma once".to_string());
    header.push("/*".to_string());
    header.push("auto generated c header, do not modify this".to_string());
    header.push("*/".to_string());
    header.push("#include <stdint.h>".to_string());
    header.push("#include \"type_wrapper.h\"".to_string());
    header.push("#include \"RustBinders.h\"".to_string());
    /*    
    ("*mut i32".into(),"int32_t*".into()), 
    ("*mut u32".into(), "uint32_t*".into()), 
    ("*mut u8".into(), "unsigned char*".into()), 
    ("*mut i8".into(), "char*".into()),
    ("*const i32".into(),"const int32_t*".into()), 
    ("*const u32".into(), "const uint32_t*".into()), 
    ("*const u8".into(), "unsigned const char*".into()), 
    ("*const i8".into(), "const char*".into()),
    ("*mut c_char".into(),"char*".into()), 
    ("*const c_char".into(),"const char*".into()), 
     */
    header.push("typedef int32_t* I32Ptr;".to_string());
    header.push("typedef uint32_t* U32Ptr;".to_string());
    header.push("typedef const int32_t* CI32Ptr;".to_string());
    header.push("typedef const uint32_t* CU32Ptr;".to_string());
    header.push("typedef char* CharPtr;".to_string());
    header.push("typedef unsigned char* UCharPtr;".to_string());
    header.push("typedef const char* CCharPtr;".to_string());
    header.push("typedef unsigned const char* UCCharPtr;".to_string());
    header.push("extern \"C\"{".to_string());
    cpp.push("/*".to_string());
    cpp.push("auto generated c header, do not modify this".to_string());
    cpp.push("*/".to_string());
    cpp.push("#include \"core_api.h\"".to_string());
    cpp.push("extern \"C\"{".to_string());
    let mut lua_content = Vec::new();
    lua_content.push("/*".to_string());
    lua_content.push("auto generated c header, do not modify this".to_string());
    lua_content.push("*/".to_string());
    lua_content.push("#include \"../core_api.h\"".to_string());
    lua_content.push("#include \"tolua.h\"".to_string());
    //#include <stdint.h>
    for file in file_list{
        parse_file(file, &mut header, &mut cpp, &mut lua_api)?;
    }
    header.push("}".to_string());
    cpp.push("}".to_string());
    //lua接口
    for (_, lines) in &mut lua_api {
        lua_content.append(lines);
    }
    //lua注册接口
    lua_content.push("void InitRust2Lua(lua_State L){".to_string());
    lua_content.push("  BeginClass(L, nullptr);".to_string());
    for (api_name, _) in lua_api{
        lua_content.push(format!("  RegisterFunc(L, nullptr, \"{}\", &{});", api_name, api_name));
    }
    lua_content.push("  EndClass(L, nullptr);".to_string());
    lua_content.push("}".to_string());
    let header_content = header.join("\r\n");
    let header_path = root.replace("\\", "/") + "/Runtime/core_api.h";
    let cpp_path = root.replace("\\", "/") + "/Runtime/core_api.cpp";
    let lua_path = root.replace("\\", "/") + "/Runtime/RustToLua.cpp";
    let def_path = root.replace("\\", "/") + "/Runtime/core_api.def";
    let lua_native_path = root.replace("\\", "/") + "/Runtime/LuaNative.lua";
    println!("header path {}", header_path);
    //api file
    let mut file = File::create(header_path)?;
    file.write_all(header_content.as_bytes())?;
    //cpp file 
    let mut file = File::create(cpp_path)?;
    file.write_all(cpp.join("\r\n").as_bytes())?;    
    //lua file
    let mut file = File::create(lua_path)?;
    file.write_all(lua_content.join("\r\n").as_bytes())?;    
    //.def file
    let mut file = File::create(def_path)?;
    println!("names {:?}", unsafe{&DEF_NAMES});
    file.write_all(unsafe{ DEF_NAMES.join("\r\n")}.as_bytes())?;

       
    //lua native file
    let mut file = File::create(lua_native_path)?;
    file.write_all(super::build_lua::generate()?.join("\r\n").as_bytes())?;    
    Ok(())
}
#[derive(Debug, Clone, Default)]
struct LuaExplortInfo{
    not_to_lua: bool, 
    lua_out_parameters: Vec<String>,    //lua的多返回值
}
fn parse_file(file_path: String, header: &mut Vec<String>, cpp: &mut Vec<String>, lua_api: &mut Vec<(String, Vec<String>)>) -> anyhow::Result<()>{
    /*
    正常导出需要api按照固定格式
    #[no_mangle]
    extern [unsafe] "C" fn api(..) -> ..{

    }
    如果参数或者返回类型是函数指针或者回调,需要使用type申明,且包含 Callback
    函数的参数不能含有()
    */
    let mut file = File::open(&file_path)?;
    let mut content = String::default();
    file.read_to_string(&mut content)?;
    let content = content.replace("\r\n", "\n");
    let lines = content.split("\n").collect::<Vec<_>>();
    let mut macros = String::default();
    let mut c_sharp_api = false;
    let mut lua_info = LuaExplortInfo::default();
    for index in 0..lines.len() {
        let line = lines[index];
        //println!("file {} line {} {}", file_path, index + 1, line);
        if lines[index].trim_start() == "#[no_mangle]"{
            parse_fn(lines[index + 1], header, cpp, &macros, c_sharp_api, lua_api, lua_info.clone())?;
            macros = "".to_string();
            c_sharp_api = false;
            lua_info = Default::default();
        }
        else if {
            let trim = line.trim_matches(|c| c == ' ' || c == '\t');
            trim.starts_with("// export-c-api") ||
            trim.starts_with("//export-c-api") ||
            trim.starts_with("/// export-c-api") ||
            trim.starts_with("///export-c-api")

        }{
            c_sharp_api = true;
        }
        else if line.starts_with("/// not to lua"){
            lua_info.not_to_lua = true;
        }
        else if line.starts_with("/// lua-results:"){
            for result in line.replace("/// lua-results:", "").split(";") {
                if result.len() > 0{
                    lua_info.lua_out_parameters.push(result.trim_start().trim_end().to_string());
                }
            }
        }
        else if line.starts_with("#[cfg(") && line.contains("target_os"){
            macros = line.to_string();
            println!("macro {}", line);
        }
    }
    Ok(())
}
fn parse_fn(fn_name: &str, header: &mut Vec<String>, cpp: &mut Vec<String>, macros: &str, c_sharp_api: bool, lua_api: &mut Vec<(String, Vec<String>)>, lua_info: LuaExplortInfo) -> anyhow::Result<()>{
    //println!("parse fn {}", fn_name);
    let line_name = fn_name.to_string();
    let fn_content = line_name.replace("extern", "")
    .replace("\"C\"", "")
    .replace("unsafe", "")
    .replace("fn", "")
    .replace("pub", "");
    let mut lua_function: Vec<String> = Vec::new();
    //宏
    let mut macro_defines: Vec<String> = Vec::new();
    if macros.contains("windows"){
        macro_defines.push("UE_RS_WIN".to_string());
    }
    if macros.contains("macos"){
        macro_defines.push("UE_RS_MAC".to_string());
    }
    if macro_defines.len() > 0{
        let macro_define = "#if ".to_string() + &macro_defines.join(" || ");
        header.push(macro_define.clone());
        if !lua_info.not_to_lua{
            lua_function.push(macro_define.clone());
        }
        if c_sharp_api{
            cpp.push(macro_define);
        }
    }
    //函数信息
    let fn_name = fn_content[0..fn_content.find(|c| c == '(').unwrap()].trim().to_string();
    let result_type = if fn_content.contains("->"){
        fn_content[fn_content.find(|c| c == '>').unwrap() + 1..fn_content.find(|c| c == '{').unwrap()].trim().to_string()
    }
    else{
        "()".to_string()
    };
    let parameter_content = fn_content[fn_content.find(|c| c == '(').unwrap() + 1..fn_content.find(|c| c == ')').unwrap()].replace("::", "^").to_string();
    let mut parameters = Vec::new();
    if parameter_content.trim().len() > 0{
        let parameters_list = parameter_content.split(",").map(|p| p.trim()).collect::<Vec<_>>();
        for p in parameters_list {
            let ps = p.split(":").collect::<Vec<_>>();
            let name = ps[0].to_string();
            let type_name = ps[1].trim_start().to_string();
            parameters.push((name, type_name));
        }
    }
    //println!("fn_name {}, result_type {}, parameters {:?}", fn_name, result_type, parameters);
    let mut c_file = String::from("    ");
    let mut c_exporter = String::from("    ");
    let mut ret_list: Vec<String> = Vec::new();
    let mut param_list: Vec<(String, String)> = Vec::new();
    //返回值
    if result_type == "()"{
        c_file += "void ";
        c_exporter += &format!("void {} {} ", EXPORT_API_NAME, INTERFACE_API);
    }
    else{
        if super::PRIMARY_NAMES.contains(&result_type){
            c_file += &super::C_PRIMARY_TYPE[&result_type];
            ret_list.push(result_type.to_string());
            c_exporter += &format!("{} {} {} ", &super::C_PRIMARY_TYPE[&result_type], EXPORT_API_NAME, INTERFACE_API);
        }
        else if result_type.contains("*") || result_type.to_lowercase().contains("callback"){
            let type_def = if result_type.to_lowercase().contains("callback"){
                if result_type.contains("::"){
                     result_type.split("::").collect::<Vec<_>>().last().cloned().unwrap().to_string()
                 }
                 else{
                     result_type.to_string()
                 }
            }
            else{
                if RESULT_PTR_CHECKER.contains_key(&result_type){
                    RESULT_PTR_CHECKER[&result_type].clone()
                }
                else{
                    "CVoidPtr".to_string()
                }
            };
            c_file += &type_def;
            ret_list.push(result_type.clone());
            c_file += &format!("/*wraper of {}*/", result_type.replace("^", "::"));
            c_exporter += &format!("{} {} {} ", type_def, EXPORT_API_NAME, INTERFACE_API);
        }
        else{
            c_file += &result_type;
            ret_list.push(result_type.clone());
            c_exporter += &format!("{} {} {} ", result_type, EXPORT_API_NAME, INTERFACE_API);
        }
        c_file += " ";
    }    
    c_file += &fn_name;
    let explore_name = format!("Ca{}", fn_name.replace("RustApi", "").replace("rust_api_", ""));
    c_exporter += &explore_name;
    unsafe{
        if c_sharp_api{
            DEF_NAMES.push(format!("   {}", explore_name));
        }
    }
    c_file += "(";
    c_exporter += "(";
    //函数参数
    let mut c_file_pm = Vec::new();
    let mut c_file_em = Vec::new();
    for (pn, pt) in parameters {
        param_list.push((pt.clone(), pn.clone()));
        if super::PRIMARY_NAMES.contains(&pt){
            c_file_pm.push(format!("{} {}", super::C_PRIMARY_TYPE[&pt], pn));
        }
        else if pt.contains("*") || pt.to_lowercase().contains("callback") || pt.to_lowercase().contains("fptr"){
            if pt.to_lowercase().contains("callback") || pt.to_lowercase().contains("fptr"){
                let type_def = if pt.contains("^"){
                    pt.split("^").collect::<Vec<_>>().last().cloned().unwrap().to_string()
                }
                else{
                    pt.to_string()
                };
                c_file_pm.push(format!("{}/*wraper of {}*/ {}", type_def, pt.replace("^", "::"), pn));
            }
            else{
                let pointer_name = if PTR_CHECKER.contains_key(&pt){
                    PTR_CHECKER[&pt].clone()
                }
                else{
                    "CVoidPtr".to_string()
                };
                c_file_pm.push(format!("{}/*wraper of {}*/ {}", pointer_name, pt.replace("^", "::"), pn));
            }
            
        }
        else{
            c_file_pm.push(format!("{} {}", pt, pn));
        }
        c_file_em.push(pn);
    }
    c_file += &c_file_pm.join(", ");
    c_file += ");";
    c_exporter += &c_file_pm.join(", ");
    c_exporter += "){\r\n";
    if result_type == "()"{
        c_exporter += &format!("        {}({});\r\n", fn_name, c_file_em.join(", "));
    }
    else{
        c_exporter += &format!("        return {}({});\r\n", fn_name, c_file_em.join(", "));
    }
    if !lua_info.not_to_lua{
        parse_lua_api(&explore_name, &fn_name, ret_list, param_list, &mut lua_function, &lua_info)?;
    }
    c_exporter += "    }";
    header.push(c_file);
    if c_sharp_api{
        cpp.push(c_exporter);
    }
    if macro_defines.len() > 0{
        header.push("#endif".to_string());
        if !lua_info.not_to_lua{
            lua_function.push("#endif".to_string());
        }
        if c_sharp_api{
            cpp.push("#endif".to_string());
        }
    }
    //暂不支持回调,所以这里可能会是空
    if !lua_info.not_to_lua && lua_function.len() > 0{
        lua_api.push((explore_name, lua_function));
    }
    Ok(())
}

fn get_lua_type(type_name: &str) -> Option<String>{
    match type_name{   
        "bool" => "LUA_TBOOL",
        "u64" | "i64" => "LUA_TI64",
        "f32" => "LUA_TFLOAT",
        "u32" | "i32" | "u16" | "i16" | "i8" | "u8" => "LUA_TINT",
        "*const u8" | "*const i8"=> "LUA_TSTRING",
        _ => "LUA_TUSERDATA"
    }.to_string().into()
}
fn get_lua_ret(type_name: &str, out_parameter: bool) -> Option<String>{
    match type_name{
        //lua list out result
        x if x.contains("*mut *mut") || x.contains("*mut *const") => {
            match x.replace("*mut *mut", "").replace("*mut *const", "").trim_start(){                
                "u64" | "i64" | "f64" | "f32" | "u32" | "i32" | "u16" | "i16" => "LUA_TUSERDATA",
                "i8" | "u8" => "LUA_TSTRING",
                _ =>{
                    println!("fail to parse type {} ret", type_name);
                    //return None;
                    "LUA_TUSERDATA"
                }
            }
        },
        //lua single out result
        x if x.contains("*mut") => {
            match x.replace("*mut", "").trim_start(){                
                "f32"  => "LUA_TFLOAT",
                "u64" | "i64" => "LUA_TI64",
                "u32" | "i32" | "u16" | "i16" | "i8" | "u8" => "LUA_TINT",
                _ =>{
                    println!("fail to parse type {} ret", type_name);
                    "LUA_TUSERDATA"
                }
            }
        },
        _ if out_parameter => return None,
        "bool" => "LUA_TBOOL",
        "u64" | "i64" => "LUA_TI64",
        "f32" => "LUA_TFLOAT",
        "u32" | "i32" | "u16" | "i16" | "i8" | "u8" => "LUA_TINT",
        "*const u8" => "LUA_TSTRING",
        _ => "LUA_TUSERDATA"
    }.to_string().into()
}
fn get_lua_get_fn(lua_type: &str) -> String{
    match lua_type{
        "LUA_TFLOAT" => "lua_tonumber",
        "LUA_TBOOL" => "tolua_toboolean",
        "LUA_TI64" => "tolua_toint64",
        "LUA_TINT" => "lua_tointeger",
        "LUA_TSTRING" => "lua_tolstring",
        "LUA_TUSERDATA" => "lua_tolightuserdata",
        _ => panic!("unexpected error lua_type {}", lua_type)
    }.to_string()
}
fn get_lua_set_fn(lua_type: &str) -> String{
    match lua_type{
        "LUA_TFLOAT" => "lua_pushnumber",
        "LUA_TBOOL" => "lua_pushboolean",
        "LUA_TI64" => "tolua_pushint64",
        "LUA_TINT" => "lua_pushnumber",
        "LUA_TSTRING" => "lua_pushlstring",
        "LUA_TUSERDATA" => "lua_pushlightuserdata",
        _ => panic!("unexpected error lua_type {}", lua_type)
    }.to_string()
}
static LUA_TYPE_WRAPPER: Lazy<BTreeMap<String, String>> = Lazy::new(||{
    let mut map = BTreeMap::new();
    map.insert("LUA_TI64".to_string(), "LUA_TNUMBER".to_string());
    map.insert("LUA_TINT".to_string(), "LUA_TNUMBER".to_string());
    map.insert("LUA_TFLOAT".to_string(), "LUA_TNUMBER".to_string());
    map.insert("LUA_TUSERDATA".to_string(), "LUA_TTABLE".to_string());
    map.insert("LUA_TSTRING".to_string(), "LUA_TSTRING".to_string());
    map.insert("LUA_TBOOL".to_string(), "LUA_TBOOLEAN".to_string());
    map
});

pub static LUA_OUT_PARAMETERS: Lazy<BTreeMap<String, String>> = Lazy::new(||{
    let map = 
    vec![
    ("*mut i32".into(),"int32_t".into()), 
    ("*mut u32".into(), "uint32_t".into()), 
    ("*mut u8".into(), "unsigned char".into()), 
    ("*mut i8".into(), "char".into()),
    ("*const i32".into(),"const int32_t".into()), 
    ("*const u32".into(), "const uint32_t".into()), 
    ("*const u8".into(), "unsigned const char".into()), 
    ("*const i8".into(), "const char".into()),
    ("*mut c_char".into(),"char".into()), 
    ("*const c_char".into(),"const char".into()), 
    ("*mut *mut i32".into(),"int32_t*".into()), 
    ("*mut *mut u32".into(), "uint32_t*".into()), 
    ("*mut *mut u8".into(), "unsigned char*".into()), 
    ("*mut *mut i8".into(), "char*".into()),
    ("*mut *const i32".into(),"const int32_t*".into()), 
    ("*mut *const u32".into(), "const uint32_t*".into()), 
    ("*mut *const u8".into(), "unsigned const char*".into()), 
    ("*mut *const i8".into(), "const char*".into()),
    ("*mut *mut Position".into(), "Position*".into()),
    ("*mut *const Position".into(), "Position*".into()),
    ].into_iter().collect();
    map
});
fn parse_lua_api(api_name: &str, native_fn_name: &str, mut ret_list: Vec<String>, parameters: Vec<(String, String)>, lua_fn: &mut Vec<String>, lua_info: &LuaExplortInfo) -> anyhow::Result<()>{
    if ret_list.iter().find(|name| name.to_lowercase().contains("callback") || name.to_lowercase().contains("fptr")).is_some(){
        return Ok(());
    }
    if parameters.iter().find(|name| name.0.to_lowercase().contains("callback") || name.0.to_lowercase().contains("fptr")).is_some(){
        return Ok(());
    }
    let mut call_result = ret_list.first().cloned().unwrap_or("".to_string());
    let mut call_result_type = String::default();
    //println!("call ret {}-{}", call_result, call_result.len());
    if call_result.len() > 1{
        call_result_type = call_result.clone();
        call_result = format!("const auto call_result = ");
    }
    else{
        call_result = Default::default();
    }
    ret_list.clear();
    lua_fn.push(format!("int {}(lua_State L){{", api_name));
    lua_fn.push(format!("\tCheckArgsCount(L, {});", parameters.len() - lua_info.lua_out_parameters.len()));
    let mut out_list: Vec<(String, String)> = Vec::new();
    let mut arg_index = 1;
    let mut release_list: Vec<String> = Vec::new();
    //用于生成lua api的参数列表
    let mut parameter_list : Vec<(String, String)> = Vec::new();
    //用于生成lua api的结果类型列表
    let mut result_list: Vec<(String, String)> = Vec::new();
    for (index, (tp, name)) in parameters.iter().enumerate(){
        if let Some(parameter) = lua_info.lua_out_parameters.iter().find(|parameter| name.trim().to_lowercase() == parameter.trim().to_lowercase()){
            if let Some(lua_ret) = get_lua_ret(&tp, true) {
                //println!("rs tp {}: {} to lua_ret {}", tp, name, lua_ret);
                let c_type =  LUA_OUT_PARAMETERS[tp].clone();
                //let tp = tp.trim_start();
                lua_fn.push(format!("\t{} {}{};", c_type, name, if_else!(c_type.contains("*"), " = nullptr", "")));
                out_list.push((c_type, name.clone()));
                ret_list.push(lua_ret);
            }
            else{
                println!("fail to parse rust out parameters {}:{}-({},{})", native_fn_name, parameter, tp, name);
                panic!("")
            }
        }
        else if let Some(lua_type) = get_lua_type(&tp){
            // println!("rs tp {} to lua_type {}", tp, lua_type);
            lua_fn.push(format!("\tCheckLuaType(L, {}, {});", arg_index, LUA_TYPE_WRAPPER[&lua_type]));
            let fn_name = get_lua_get_fn(&lua_type);
            if fn_name == "lua_tolightuserdata"{
                lua_fn.push(format!("\tto_array(L, {}, {}, {});", arg_index, PTR_CHECKER[tp].replace("*", ""), name));
                release_list.push(name.clone());
                parameter_list.push(("table".to_string(), name.to_string()));
            }
            else{
                lua_fn.push(format!("\tconst auto {} = {}(L, {}{});", name, fn_name, arg_index, if_else!(fn_name.contains("lua_tolstring"), ", nullptr", "")));
                match fn_name.as_str(){
                    "lua_tolstring" => {
                        parameter_list.push(("string".to_string(), name.to_string()));
                    },
                    "tolua_toint64" | "lua_tonumber" | "lua_tointeger" => {
                        parameter_list.push(("number".to_string(), name.to_string()));
                    },
                    "tolua_toboolean" => {
                        parameter_list.push(("boolean".to_string(), name.to_string()));
                    }
                    _ => {
                        parameter_list.push(("userdata?".to_string(), name.to_string()));
                    }
                }
            }
            arg_index += 1;
        }
        else{
            println!("fail to parse rust parameters {}:{}-({},{})", native_fn_name, index, tp, name);
            panic!("")
        }
    }
    if arg_index != parameters.len() - lua_info.lua_out_parameters.len() + 1{
        panic!("导出的lua函数[{}]参数错误,实际参数 {}, 总参数 {}, out 参数 {}", native_fn_name, arg_index, parameters.len(), lua_info.lua_out_parameters.len());
    }
    if parameters.len() > 0{
        let mut params = Vec::new();
        for (_, (tp, name)) in parameters.iter().enumerate() {   
            if out_list.iter().find(|t| t.1.trim().to_lowercase() == name.trim().to_lowercase()).is_some(){
                continue;
            }         
            params.push(format!("({}){}", PTR_CHECKER.get(tp).cloned().unwrap_or(super::C_PRIMARY_TYPE.get(tp).cloned().unwrap_or(tp.clone())), name));
        }
        for (_, (_, name)) in out_list.iter().enumerate() {            
            params.push(format!("&{}", name));
        }
        lua_fn.push(format!("\t{}{}({});", call_result, native_fn_name, params.join(", ")));
    }
    else{
        lua_fn.push(format!("\t{}{}();", call_result, native_fn_name));
    }
    let mut out_count = 0;
    if call_result.len() > 0{
        if let Some(lua_type) = get_lua_ret(&call_result_type, false){
            let push_fn = get_lua_set_fn(&lua_type);
            lua_fn.push(format!("\t{}(L, call_result);", push_fn));
            println!("fn name {} call_result fn {}, name {} ", api_name, push_fn, call_result_type);
            match push_fn.as_str(){
                "lua_tolstring" => {
                    result_list.push(("string".to_string(), "result".to_string()));
                },
                "lua_pushnumber" | "tolua_pushint64"=> {
                    result_list.push(("number".to_string(), "result".to_string()));
                },
                "lua_pushboolean" => {
                    result_list.push(("boolean".to_string(), "result".to_string()));
                }
                _ => {
                    result_list.push(("userdata?".to_string(), "result".to_string()));
                }
            }
            out_count += 1;
        }
        else{
            panic!("fail to parse function call result {}", call_result_type);
        }
    }
    let mut idx = 0;
    while idx < ret_list.len(){
        let ret = &ret_list[idx];
        //println!(" ret {}, {}, lua_info out {:?}", ret, out_list[idx].1, lua_info.lua_out_parameters);
        let name = 
        if lua_info.lua_out_parameters.contains(&out_list[idx].1){
            format!("{}", out_list[idx].1)
        }else{
            format!("*{}", out_list[idx].1)
        };
        let fn_name = get_lua_set_fn(&ret);
        if fn_name == "lua_pushlstring"{
            lua_fn.push(format!("\tif(nullptr != {}){{", name));
            lua_fn.push(format!("\t\t{}(L, (const char*){}, {});", fn_name, name, &out_list[idx + 1].1));
            lua_fn.push(format!("\t\tReleaseVector((unsigned char*){}, {}, sizeof(const char), out_cap);", name, &out_list[idx + 1].1));
            lua_fn.push(format!("\t}}"));
            lua_fn.push(format!("\telse{{\r\n\t\tlua_pushnil(L);\r\n\t}}"));
            result_list.push(("string".to_string(), name.to_string()));
            idx += 2;
        }
        else{
            if fn_name == "lua_pushlightuserdata"{
                if out_list[idx].0.contains("Position"){
                    lua_fn.push(format!("\tpush_positions(L, {}, {}, out_cap);", name, &out_list[idx + 1].1));
                }
                else{
                    lua_fn.push(format!("\tpush_integers(L, {}, {}, {}, out_cap);", name, &out_list[idx + 1].1, out_list[idx].0.replace("*", "")));
                }
                result_list.push(("table".to_string(), name.to_string()));
                idx += 2;
            }
            else{
                lua_fn.push(format!("\t{}(L, {});", fn_name, name));
                match fn_name.as_str(){
                    "lua_tolstring" => {
                        result_list.push(("string".to_string(), name.to_string()));
                    },
                    "lua_pushnumber" | "tolua_pushint64" => {
                        result_list.push(("number".to_string(), name.to_string()));
                    },
                    "lua_pushboolean" => {
                        result_list.push(("boolean".to_string(), name.to_string()));
                    }
                    _ => {
                        result_list.push(("userdata?".to_string(), name.to_string()));
                    }
                }
            }
        }
        out_count += 1;
        idx += 1;
    }
    for delete in release_list {        
        lua_fn.push(format!("\tif ({}) delete[] {};", delete, delete));
    }
    out_list.insert(0, (call_result_type, String::default()));
    lua_fn.push(format!("\treturn {};\r\n}}", out_count.max(1)));
    super::build_lua::push(&api_name, parameter_list, result_list);
    Ok(())
}