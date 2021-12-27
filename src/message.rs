#![allow(non_snake_case)]

use std::{ffi::c_void, mem::transmute, ptr::null};

type HModule = *const c_void;
type FarProc = *const c_void;

extern "stdcall" {
    fn LoadLibraryA(name: *const u8) -> HModule;
    fn GetProcAddress(module: HModule, name: *const u8) -> FarProc;
}

type MessageBoxA = extern "stdcall" fn(*const c_void, *const u8, *const u8, u32);

pub fn show_message(title: &str, content: &str) {
    unsafe {
        let h = LoadLibraryA("USER32.dll\0".as_ptr());
        
        let MessageBoxA: MessageBoxA = transmute(GetProcAddress(h, "MessageBoxA\0".as_ptr()));

        let title = format!("{}\0", title);
        let content = format!("{}\0", content);

        MessageBoxA(
            null(),
            content.as_ptr(),
            title.as_ptr(),
            0,
        );
    }
}

