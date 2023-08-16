use std::{ffi::c_void, mem::transmute};

type HModule = *const c_void;
type FarProc = *const c_void;

extern "stdcall" {
    fn LoadLibraryA(lib: *const u8) -> HModule;
    fn GetProcAddress(module: HModule, name: *const u8) -> FarProc;
}

const USER32: *const u8 = "USER32.dll\0".as_ptr();

fn main() {
    unsafe {
        let h_mod = LoadLibraryA(USER32);

        let msg_box: extern "stdcall" fn(*const c_void, *const u8, *const u8, u32) =
            transmute(GetProcAddress(h_mod, "MessageBoxA\0".as_ptr()));

        use std::ptr::null;
        msg_box(null(), "Hello, from Rust!\0".as_ptr(), null(), 0);
    }
}
