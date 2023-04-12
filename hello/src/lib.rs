use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn hello(name: *const libc::c_char) -> *const c_char {
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name = name_cstr.to_str().unwrap();
    let say = format!("Hello {}!", name);
    println!("rust say {}", say.clone());
    let say_cstr = CString::new(say).unwrap();
    say_cstr.into_raw()
}


