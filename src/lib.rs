#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;
#[macro_use]
mod macros;
mod logger;
///debug模式开启
static mut DEBUG_ENABLED: bool = false;
///调试模式开启
pub fn debug_enabled() -> bool{
    unsafe{
        DEBUG_ENABLED
    }
}
static DEBUG_LOCK: once_cell::sync::Lazy<std::sync::Mutex<()>> = once_cell::sync::Lazy::new(||std::sync::Mutex::new(()));
///开启调试模式
pub fn set_debug(enabled: bool){
    let _m = DEBUG_LOCK.lock().unwrap();
    unsafe{
        DEBUG_ENABLED = enabled;
    }
}


/// export-c-api
#[no_mangle]
extern "C" fn ReleaseVector(ptr: *mut u8, size: u32, type_size: u32, cap: u32) {
    if ptr.is_null(){
        return;
    }
    unsafe{
        let len = (size * type_size) as usize;
        // log_debug!("ReleaseVector size {}, type_size {}, cap {}", size, type_size, cap);
        Vec::from_raw_parts(ptr, len, cap as usize);
    }
}
#[no_mangle]
extern fn SetRustLogLevel(log_level: i32) -> bool{
    let logger_init = logger::init_logger(log_level).is_ok();
    log_info!("native plugin initialized success {}", logger_init);
    logger_init
}

#[no_mangle]
extern fn TestRustLog(level: i32){
    if level == 4{
        log_debug!("rust trace log");
    }
    else if level == 3{
        log_info!("rust info log");
    }
    else if level == 2{
        log_warn!("rust warn log");
    }
    else {
        log_error!("rust error log");
    }
}