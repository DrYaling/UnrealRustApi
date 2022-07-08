//! native plugin logger
use std::{ffi::CString, os::raw::c_char, sync::atomic::{AtomicBool, Ordering}, ops::Deref};

use once_cell::sync::{Lazy};
pub type UELogCallback = unsafe extern "C" fn(data: *const c_char, log_level: i32);
static mut _LOGGER: Option<UELogCallback> = None;
 static LOGGER: Lazy<RustApiLogger> = Lazy::new(||{
    RustApiLogger
 });
static LOGGER_INITED: AtomicBool = AtomicBool::new(false);
static mut LOG_LEVEL: i32 = 4;
struct RustApiLogger;
impl log::Log for RustApiLogger{
    fn enabled(&self, _: &log::Metadata) -> bool {
        //println!("check enabled");
        true
    }

    fn log(&self, record: &log::Record) {
        let log_level: i32 = unsafe{LOG_LEVEL};
        match record.level(){
            log::Level::Error => {
                log(&format!("[Native][ERROR] [{}]: {}", record.target(), record.args()), 1);
            }, 
            log::Level::Warn if log_level >= 2 => {
                log(&format!("[Native][ERROR] [{}]: {}", record.target(), record.args()), 2);
            },
            log::Level::Info if log_level >= 3 => {
                log(&format!("[Native][DEBUG] [{}]: {}", record.target(), record.args()), 3);
            },
            log::Level::Debug if log_level >= 4 => {
                log(&format!("[Native][TRACE] [{}]: {}", record.target(), record.args()), 4);
            },
            _ => ()
        }
    }

    fn flush(&self) {
        //println!("flush");
    }
}
pub fn init_logger(log_level: i32) -> anyhow::Result<()>{
    unsafe{ 
        LOG_LEVEL = log_level;
    }
    if LOGGER_INITED.load(Ordering::Acquire){
        return Ok(());
    }
    log::set_logger(LOGGER.deref()).map_err(|e| anyhow!(e.to_string()))?;    
    log::set_max_level(log::LevelFilter::Debug);
    LOGGER_INITED.store(true, Ordering::Release);
    std::panic::set_hook(Box::new(|info|{
        log(&format!("system panic {:?}",info), 1);
    }));
    log_debug!("init logger");
    Ok(())
}
fn log(data:&str, level: i32){
    unsafe{
        match &_LOGGER{
            Some(logger) => {
                let c_str = CString::new(data).unwrap();
                logger(c_str.as_ptr(), level);
            },
            _ => {
                println!("[std log] {}",data)
            }
        }
    }
}

///在外部语言调用进行绑定
#[no_mangle]
pub extern fn BindDebugLogger(func: UELogCallback){
    unsafe{
        _LOGGER = Some(func);
    }
}
// pub unsafe fn clear(){
//     _LOGGER = None;
// }
// #[no_mangle]
// extern fn log_something(){
//     log("this is log from rust!");
//     log_err("this is log from rust!");
//     log("中文日志打印!");
// }
#[cfg(test)]
#[test]
fn test_logger(){
    println!("log init");
    init_logger(1).expect("fail to init logger");
    println!("log error");
    log_info!("info log {}", 222);
    debug!("debug info ");
    log_debug!("load battle");
    log_error!("error log {} {} {}", "ok", "i'm fine ", std::mem::size_of_val(&0u32));
    
}