use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::{ptr, slice};

extern "C" {
    fn EthSayGo();
    fn EthSayGoHello() -> *const c_char;
}

#[no_mangle]
pub extern "C" fn hello(name: *const libc::c_char) -> *const c_char {
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name = name_cstr.to_str().unwrap();
    // call go
    unsafe { EthSayGo() }

    let hello_cstr = unsafe { CStr::from_ptr(EthSayGoHello()) };
    let hello_str = hello_cstr.to_str().unwrap();
    println!("-----------{}------", hello_str);
    // return to go
    let say = format!("rust 3 Hello {}!", name);
    let say_cstr = CString::new(say).unwrap();
    say_cstr.into_raw()
}


