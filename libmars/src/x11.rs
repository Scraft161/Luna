extern crate x11;

use x11::xlib;
use std::ffi::*;

pub mod backend;
mod client;
mod atoms;

// see cursorfont.h
const CURSOR_NORMAL: u32 = 68;
const CURSOR_RESIZE: u32 = 120;
const CURSOR_MOVE: u32 = 52;
const XLIB_NONE: u64 = 0;
const BUTTONMASK: i64 = xlib::ButtonPressMask | xlib::ButtonReleaseMask;
const MOUSEMASK: i64 = BUTTONMASK | xlib::PointerMotionMask;
const WINDOW_MIN_SIZE: u32 = 40;
const NORMAL_STATE: i32 = 1;
const ICONIC_STATE: i32 = 3;



fn button_mask(button: u32) -> u32 {
    return 1 << (7 + button);
}

extern "C" fn on_wm_detected(_: *mut xlib::Display, _: *mut xlib::XErrorEvent) -> c_int {
    panic!("Another Window Manager seems to be running already");
}

extern "C" fn on_error(_display: *mut xlib::Display, error: *mut xlib::XErrorEvent) -> c_int {
    // unsafe {
    //     let bufsize = 1024;
    //     let mut buf = vec![0u8; bufsize];
    //     xlib::XGetErrorText(display, (*error).error_code.into(), buf.as_mut_ptr() as *mut i8,
    //                         (bufsize - 1) as c_int);
    //     let msg = CString::new(buf).unwrap().into_string().unwrap();
    //     println!("{}", msg);
    // }

    unsafe {
        match (*error).error_code {
            // @TODO add error types
            xlib::Success => println!("everything's okay"),
            xlib::BadRequest => panic!("bad request code"),
            xlib::BadValue => panic!("int parameter out of range"),
            xlib::BadWindow => println!("parameter not a Window"),
            xlib::BadPixmap => panic!("parameter not a Pixmap"),
            xlib::BadAtom => panic!("parameter not an Atom"),
            xlib::BadCursor => panic!("parameter not a Cursor"),
            xlib::BadFont => panic!("parameter not a Font"),
            xlib::BadMatch => panic!("parameter mismatch"),
            xlib::BadDrawable => panic!("parameter not a Pixmap or Window"),
            xlib::BadAccess => panic!("depending on context (see X.h)"),
            xlib::BadAlloc => panic!("insufficient resources"),
            xlib::BadColor => panic!("no such colormap"),
            xlib::BadGC => panic!("parameter not a GC"),
            xlib::BadIDChoice => panic!("choice not in range or already used"),
            xlib::BadName => panic!("font or color name doesn't exist"),
            xlib::BadLength => panic!("Request length incorrect"),
            xlib::BadImplementation => panic!("server is defective"),
            _ => panic!("unknown error occured"),
        }
    }

    return 0;
}

extern "C" fn on_error_dummy(_display: *mut xlib::Display, _error: *mut xlib::XErrorEvent) -> c_int {
    return 0;
}

fn sanitize_modifiers(modifiers: u32) -> u32 {
    return modifiers & (xlib::ShiftMask | xlib::ControlMask | xlib::Mod1Mask | xlib::Mod2Mask
                        | xlib::Mod3Mask | xlib::Mod4Mask |xlib::Mod5Mask);
}
