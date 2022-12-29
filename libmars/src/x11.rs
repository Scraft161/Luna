extern crate x11;

use std::ffi::*;
use x11::xlib;
use x11::xinerama;

use crate::*;
use crate::x11::atoms::X11Atom::*;

pub mod backend;
pub mod window;
pub mod atoms;
mod client;

const XLIB_NONE: u64 = 0;
const BUTTONMASK: i64 = xlib::ButtonPressMask | xlib::ButtonReleaseMask;
const MOUSEMASK: i64 = BUTTONMASK | xlib::PointerMotionMask;
const WINDOW_MIN_SIZE: u32 = 40;
// Cursor selectors (see cursorfont.h)
const CURSOR_NORMAL: u32 = 68;
const CURSOR_RESIZE: u32 = 120;
const CURSOR_MOVE: u32 = 52;
// Window states
const WITHDRAWN_STATE: i32 = 0;
const NORMAL_STATE: i32 = 1;
const ICONIC_STATE: i32 = 3;
// Motif hints
const MWM_HINTS_FLAGS_FIELD: usize = 0;
const MWM_HINTS_DECORATIONS_FIELD: usize = 2;
const MWM_HINTS_DECORATIONS: u64 = 1 << 1;
const MWM_DECOR_ALL: u64 = 1 << 0;
const MWM_DECOR_BORDER: u64 = 1 << 1;
const MWM_DECOR_TITLE: u64 = 1 << 3;


impl From<xinerama::XineramaScreenInfo> for MonitorConfig {
    fn from(info: xinerama::XineramaScreenInfo) -> MonitorConfig {
        let area = Dimensions { x: info.x_org.into(), y: info.y_org.into(),
                                w: info.width.try_into().unwrap(), h: info.height.try_into().unwrap() };
        MonitorConfig {
            num: info.screen_number.try_into().unwrap(),
            dims: area,
            win_area: area,
        }
    }
}


pub fn get_keysym(name: &str) -> xlib::KeySym {
    unsafe {
        let cstring = CString::new(name).unwrap();
        return xlib::XStringToKeysym(cstring.as_ptr());
    }
}

extern "C" fn on_wm_detected(_: *mut xlib::Display, _: *mut xlib::XErrorEvent) -> c_int {
    panic!("Another Window Manager seems to be running already");
}

extern "C" fn on_error(display: *mut xlib::Display, error: *mut xlib::XErrorEvent) -> c_int {
    let msg = unsafe {
        let bufsize = 1024;
        let mut buf = vec![0u8; bufsize];
        xlib::XGetErrorText(display, (*error).error_code.into(), buf.as_mut_ptr() as *mut i8,
                            (bufsize - 1) as c_int);
        let msg_cstring = CStr::from_ptr(buf.as_mut_ptr() as *mut i8);
        msg_cstring.to_str().unwrap().to_owned()
        // println!("{}", msg);
    };

    unsafe {
        match (*error).error_code {
            xlib::Success => println!("X11 non-error: {} (request code: {})", msg, (*error).request_code),
            xlib::BadMatch => println!("X11 error: {} (request code: {})", msg, (*error).request_code),
            xlib::BadWindow => println!("X11 error: {} (request code: {})", msg, (*error).request_code),
            _ => panic!("Fatal X11 error: {} (request code: {})", msg, (*error).request_code),
        }
    }

    // unsafe {
    //     match (*error).error_code {
    //         // @TODO add error types
    //         xlib::Success => println!("everything's okay"),
    //         xlib::BadRequest => panic!("bad request code"),
    //         xlib::BadValue => panic!("int parameter out of range"),
    //         xlib::BadWindow => println!("parameter not a Window"),
    //         xlib::BadPixmap => panic!("parameter not a Pixmap"),
    //         xlib::BadAtom => panic!("parameter not an Atom"),
    //         xlib::BadCursor => panic!("parameter not a Cursor"),
    //         xlib::BadFont => panic!("parameter not a Font"),
    //         xlib::BadMatch => panic!("parameter mismatch"),
    //         xlib::BadDrawable => panic!("parameter not a Pixmap or Window"),
    //         xlib::BadAccess => panic!("depending on context (see X.h)"),
    //         xlib::BadAlloc => panic!("insufficient resources"),
    //         xlib::BadColor => panic!("no such colormap"),
    //         xlib::BadGC => panic!("parameter not a GC"),
    //         xlib::BadIDChoice => panic!("choice not in range or already used"),
    //         xlib::BadName => panic!("font or color name doesn't exist"),
    //         xlib::BadLength => panic!("Request length incorrect"),
    //         xlib::BadImplementation => panic!("server is defective"),
    //         _ => panic!("unknown error occured"),
    //     }
    // }

    return 0;
}

extern "C" fn on_error_dummy(_display: *mut xlib::Display, _error: *mut xlib::XErrorEvent) -> c_int {
    return 0;
}

fn sanitize_modifiers(modifiers: u32) -> u32 {
    return modifiers & (xlib::ShiftMask | xlib::ControlMask | xlib::Mod1Mask | xlib::Mod2Mask
                        | xlib::Mod3Mask | xlib::Mod4Mask |xlib::Mod5Mask);
}
