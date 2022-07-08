extern crate anyhow;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
mod build_c;
mod build_lua;
#[macro_use]
mod macros;
pub static PRIMARY_NAMES: Lazy<Vec<String>> =  Lazy::new(||{
    vec!["i32".into(), "u32".into(), "i64".into(), "u64".into(), "usize".into(), "f32".into(), "f64".into(), "i16".into(), "u16".into(), "u8".into(), "i8".into(), "bool".into()]
});
// pub static CS_PRIMARY_TYPE: Lazy<BTreeMap<String, String>> = Lazy::new(||{
//     let map = 
//     vec![("i32".into(),"int".into()), 
//     ("u32".into(), "uint".into()), 
//     ("u64".into(), "ulong".into()),
//     ("i64".into(), "long".into()), 
//     ("usize".into(), "ulong".into()), 
//     ("f32".into(), "float".into()),
//     ("f64".into(), "double".into()), 
//     ("i16".into(),"short".into()), 
//     ("u16".into(),"ushort".into()), 
//     ("u8".into(), "byte".into()), 
//     ("bool".into(), "bool".into()), 
//     ("i8".into(), "byte".into())].into_iter().collect();
//     map
// });
pub static C_PRIMARY_TYPE: Lazy<BTreeMap<String, String>> = Lazy::new(||{
    let map = 
    vec![
        ("i32".into(),"int32_t".into()), 
        ("u32".into(), "uint32_t".into()), 
        ("u64".into(), "uint64_t".into()),
        ("i64".into(), "int64_t".into()), 
        ("usize".into(), "size_t".into()), 
        ("f32".into(), "float".into()),
        ("f64".into(), "double".into()), 
        ("i16".into(),"int16_t".into()), 
        ("u16".into(),"uint16_t".into()), 
        ("u8".into(), "unsigned char".into()), 
        ("i8".into(), "char".into()),
        ("bool".into(), "bool".into())
    ].into_iter().collect();
    map
});
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TypeDefination{
    Struct, 
    Enum,
}
pub static TYPE_MAP: Lazy<BTreeMap<&'static str, TypeDefination>> = Lazy::new(type_define);
///判断某个数据结构是否是枚举
pub fn is_enum(tp: &str) -> bool{
    for (k,value) in  TYPE_MAP.iter(){
        if k.contains(tp) && *value == TypeDefination::Enum{
            return true;
        }
    }
    return false;
}
pub fn is_struct(tp: &str) -> bool{
    for (k,value) in  TYPE_MAP.iter(){
        if k.contains(tp) && *value == TypeDefination::Struct{
            return true;
        }
    }
    return false;
}
///注册数据类型
pub fn type_define() -> BTreeMap<& 'static str, TypeDefination>{
    let mut map = BTreeMap::new();
    map.insert("BattleState", TypeDefination::Enum);
    map.insert("Position", TypeDefination::Struct);
    map
}
#[allow(unreachable_code)]
fn main() -> anyhow::Result<()>{
    build_c::build()?;
    Ok(())
}