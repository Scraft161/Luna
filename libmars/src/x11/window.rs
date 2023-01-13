extern crate x11;

use std::ffi::*;
use std::mem;
use std::mem::MaybeUninit;
use std::os::raw::c_void;
use std::ptr;
use std::slice;
use x11::xlib;

use crate::x11::*;


pub trait X11Window {
    fn x11_net_wm_state_add(&self, display: *mut xlib::Display, state: xlib::Atom);
    fn x11_net_wm_state_remove(&self, display: *mut xlib::Display, state: xlib::Atom);
    fn x11_attributes(&self, display: *mut xlib::Display) -> Result<xlib::XWindowAttributes, String>;
    fn x11_class_hint(&self, display: *mut xlib::Display) -> Result<(String, String), String>;
    fn x11_destroy(&self, display: *mut xlib::Display);
    fn x11_get_state(&self, display: *mut xlib::Display) -> Result<u64, &'static str>;
    fn x11_get_text_list_property(&self, display: *mut xlib::Display, property: xlib::Atom) -> Result<Vec<String>, &'static str>;
    fn x11_read_property_long(&self, display: *mut xlib::Display, property: xlib::Atom, prop_type: c_ulong) -> Result<Vec<u64>, &'static str>;
    fn x11_replace_property_long(&self, display: *mut xlib::Display, property: xlib::Atom, prop_type: c_ulong, data: &[c_ulong]);
    fn x11_set_state(&self, display: *mut xlib::Display, state: i32);
    fn x11_set_text_list_property(&self, display: *mut xlib::Display, property: xlib::Atom, list: Vec<CString>);
    fn x11_dimensions(&self, display: *mut xlib::Display) -> Result<Dimensions, String>;
    fn x11_geometry(&self, display: *mut xlib::Display) -> Result<(u64, i32, i32, u32, u32, u32, u32), String>;
    fn x11_get_window_types(&self, display: *mut xlib::Display) -> Vec<xlib::Atom>;
    fn x11_is_transient_for(&self, display: *mut xlib::Display) -> Option<xlib::Window>;
    fn x11_message(&self, display: *mut xlib::Display, msg_type: atoms::X11Atom, msg_format: c_int, msg_data: xlib::ClientMessageData);
    fn x11_wm_protocols(&self, display: *mut xlib::Display) -> Vec<xlib::Atom>;
    fn x11_wm_name(&self, display: *mut xlib::Display) -> Result<String, &'static str>;
    fn x11_wm_normal_hints(&self, display: *mut xlib::Display) -> Result<(xlib::XSizeHints, c_long), String>;
}

impl X11Window for xlib::Window {
    fn x11_net_wm_state_add(&self, display: *mut xlib::Display, state: xlib::Atom) {
        let states_result = self.x11_read_property_long(display, NetWMState.to_xlib_atom(display), xlib::XA_ATOM);
        let mut states = match states_result {
            Ok(states) => states,
            Err(_) => Vec::new(),
        };

        if !states.contains(&state) {
            states.push(state);
        }
        self.x11_replace_property_long(display, NetWMState.to_xlib_atom(display), xlib::XA_ATOM, &states);
    }

    fn x11_net_wm_state_remove(&self, display: *mut xlib::Display, state: xlib::Atom) {
        let states_result = self.x11_read_property_long(display, NetWMState.to_xlib_atom(display), xlib::XA_ATOM);
        let mut states = match states_result {
            Ok(states) => states,
            Err(_) => return,
        };

        states.retain(|s| *s != state);
        self.x11_replace_property_long(display, NetWMState.to_xlib_atom(display), xlib::XA_ATOM, &states);
    }

    fn x11_attributes(&self, display: *mut xlib::Display) -> Result<xlib::XWindowAttributes, String> {
        unsafe {
            let mut attributes: MaybeUninit<xlib::XWindowAttributes> = MaybeUninit::uninit();
            match xlib::XGetWindowAttributes(display, *self, attributes.as_mut_ptr()) {
                0 => return Err(String::from("Unable to retrieve attributes")),
                _ => return Ok(attributes.assume_init()),
            }
        }
    }

    fn x11_class_hint(&self, display: *mut xlib::Display) -> Result<(String, String), String> {
        unsafe {
            let mut class_hints: MaybeUninit<xlib::XClassHint> = MaybeUninit::zeroed();
            let status = xlib::XGetClassHint(display, *self, class_hints.as_mut_ptr());
            if status != 0 {
                let class_hints = class_hints.assume_init();
                if !class_hints.res_name.is_null() && !class_hints.res_class.is_null() {
                    let res_name = CStr::from_ptr(class_hints.res_name)
                        .to_str().unwrap().to_owned();
                    let res_class = CStr::from_ptr(class_hints.res_class)
                        .to_str().unwrap().to_owned();
                    let ret_val = Ok((res_name, res_class));
                    xlib::XFree(class_hints.res_name as *mut c_void);
                    xlib::XFree(class_hints.res_class as *mut c_void);
                    return ret_val;
                } else {
                    return Err("Class hint returned null ptr".to_owned());
                }
            } else {
                return Err("Error getting class hint from window".to_owned());
            }
        }
    }

    fn x11_destroy(&self, display: *mut xlib::Display) {
        unsafe {
            xlib::XDestroyWindow(display, *self);
        }
    }

    fn x11_get_state(&self, display: *mut xlib::Display) -> Result<u64, &'static str> {
        let result = self.x11_read_property_long(display, WMState.to_xlib_atom(display), WMState.to_xlib_atom(display))?;
        return Ok(result[0]);
    }

    fn x11_get_text_list_property(&self, display: *mut xlib::Display, property: xlib::Atom) -> Result<Vec<String>, &'static str> {
        let mut text: MaybeUninit<xlib::XTextProperty> = MaybeUninit::uninit();
        let mut nitems = 0;
        let mut data_ptr: *mut *mut i8 = ptr::null_mut();
        let mut data = Vec::new();
        unsafe {
            if xlib::XGetTextProperty(display, *self, text.as_mut_ptr(), property) == 0 {
                return Err("unable to get text property");
            } else if xlib::Xutf8TextPropertyToTextList(display, text.as_ptr(), &mut data_ptr, &mut nitems) != 0 {
                return Err("unable to convert text property to string list");
            } else {
                for ptr in slice::from_raw_parts(data_ptr, nitems.try_into().unwrap()) {
                    let cstr = CStr::from_ptr(*ptr);
                    let s = match cstr.to_str() {
                        Ok(s) => s,
                        Err(_) => {
                            xlib::XFreeStringList(data_ptr);
                            return Err("unable to convert to string");
                        },
                    };
                    data.push(s.to_owned());
                }
                xlib::XFreeStringList(data_ptr);
                return Ok(data);
            }
        }
    }

    fn x11_read_property_long(&self, display: *mut xlib::Display, property: xlib::Atom, prop_type: c_ulong) -> Result<Vec<u64>, &'static str> {
        let mut actual_type = 0;
        let mut actual_format = 0;
        let mut nitems = 0;
        let mut remaining_bytes: u64 = 0;
        let mut data: Vec<u64> = Vec::new();
        let mut first_run = true;
        while first_run || remaining_bytes != 0 {
            unsafe {
                let mut data_ptr: *mut u8 = ptr::null_mut();
                first_run = false;
                let status = xlib::XGetWindowProperty(display, *self, property,
                    data.len() as i64 / 2, 8, xlib::False,
                    prop_type, &mut actual_type,
                    &mut actual_format,
                    &mut nitems, &mut remaining_bytes,
                    &mut data_ptr);
                if status != 0 {
                    return Err("XGetWindowProperty failed");
                } else if actual_type == XLIB_NONE
                        && actual_format == 0
                        && remaining_bytes == 0 {
                    return Err("Property does not exist for specified window");
                } else if actual_type != prop_type {
                    return Err("Property as wrong actual type");
                } else if actual_format != 32 {
                    return Err("Property is not u32 format");
                } else {
                    data.extend_from_slice(slice::from_raw_parts(data_ptr as *mut u64, nitems.try_into().unwrap()));
                    xlib::XFree(data_ptr as *mut c_void);
                }
            }
        }
        return Ok(data);
    }

    fn x11_replace_property_long(&self, display: *mut xlib::Display, property: xlib::Atom, prop_type: c_ulong, data: &[c_ulong]) {
        unsafe {
            xlib::XChangeProperty(display,
                                  *self,
                                  property,
                                  prop_type,
                                  32,
                                  xlib::PropModeReplace,
                                  data.as_ptr().cast::<u8>(),
                                  data.len() as i32);
        }
    }

    fn x11_set_state(&self, display: *mut xlib::Display, state: i32) {
        let data = state;
        let state_atom = WMState.to_xlib_atom(display);

        unsafe {
            xlib::XChangeProperty(display, *self, state_atom, state_atom,
                                  32, xlib::PropModeReplace, mem::transmute(&data), 1);
        }
    }


    fn x11_set_text_list_property(&self, display: *mut xlib::Display, property: xlib::Atom, list: Vec<CString>) {
        let mut pointers: Vec<*mut i8> = list.iter().map(|cstr| cstr.clone().into_raw()).collect();
        let slice = &mut pointers;
        let mut text: MaybeUninit<xlib::XTextProperty> = MaybeUninit::uninit();
        let size = slice.len().try_into().unwrap();
        unsafe {
            xlib::Xutf8TextListToTextProperty(display, slice.as_mut_ptr(), size, xlib::XUTF8StringStyle, text.as_mut_ptr());
            xlib::XSetTextProperty(display, *self, &mut text.assume_init(), property);
        }
    }

    fn x11_dimensions(&self, display: *mut xlib::Display) -> Result<Dimensions, String> {
        return match self.x11_geometry(display) {
            Ok((_root, x, y, w, h, _bw, _depth)) => Ok(Dimensions { x, y, w, h }),
            Err(e) => Err(e),
        };
    }

    fn x11_geometry(&self, display: *mut xlib::Display) -> Result<(u64, i32, i32, u32, u32, u32, u32), String> {
        unsafe {
            let mut root: u64 = 0;
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let mut w: u32 = 0;
            let mut h: u32 = 0;
            let mut bw: u32 = 0;
            let mut depth: u32 = 0;
            match xlib::XGetGeometry(display, *self, &mut root, &mut x, &mut y, &mut w, &mut h, &mut bw, &mut depth) {
                0 => return Err(String::from("Unable to retrieve geometry")),
                _ => return Ok((root, x, y, w, h, bw, depth)),
            }
        }
    }

    fn x11_get_window_types(&self, display: *mut xlib::Display) -> Vec<xlib::Atom> {
        let mut types = Vec::new();

        let mut da = XLIB_NONE;
        let mut di = 0;
        let mut nitems = 0;
        let mut dl = 0;
        unsafe {
            let mut win_types_ptr: *mut u8 = ptr::null_mut();
            // TODO use x11_read_property_long
            let result = xlib::XGetWindowProperty(display, *self, NetWMWindowType.to_xlib_atom(display),
                0, 8, xlib::False, xlib::XA_ATOM,
                &mut da, &mut di, &mut nitems, &mut dl, &mut win_types_ptr);
            if result == xlib::Success.into() {
                let temp = slice::from_raw_parts(win_types_ptr as *mut xlib::Atom, nitems.try_into().unwrap());
                types = temp.to_vec();
            }
        }
        return types;
    }

    fn x11_is_transient_for(&self, display: *mut xlib::Display) -> Option<xlib::Window> {
        unsafe {
            let mut window: xlib::Window = XLIB_NONE;
            match xlib::XGetTransientForHint(display, *self, &mut window) {
                0 => return None,
                _ => return Some(window),
            }
        }
    }

    fn x11_message(&self, display: *mut xlib::Display, msg_type: atoms::X11Atom, msg_format: c_int, msg_data: xlib::ClientMessageData) {
        unsafe {
            let msg_type_x11 = msg_type.to_xlib_atom(display);
            let msg_event = xlib::XClientMessageEvent {
                type_: xlib::ClientMessage,
                serial: 0,
                send_event: xlib::False,
                display,
                window: *self,
                message_type: msg_type_x11,
                format: msg_format,
                data: msg_data,
            };

            let mut event = xlib::XEvent { client_message: msg_event };

            xlib::XSendEvent(display, *self, xlib::False, 0, &mut event);
        }
    }

    fn x11_wm_protocols(&self, display: *mut xlib::Display) -> Vec<xlib::Atom> {
        let mut supported_atoms = Vec::new();
        unsafe {
            let mut atoms: *mut xlib::Atom = ptr::null_mut();
            let mut natoms: c_int = 0;
            xlib::XGetWMProtocols(display, *self, &mut atoms, &mut natoms);
            for n in slice::from_raw_parts(atoms, natoms as usize) {
                supported_atoms.push(*n);
            }
        }
        return supported_atoms;
    }

    fn x11_wm_name(&self, display: *mut xlib::Display) -> Result<String, &'static str> {
        // let mut str_ptr: *mut i8 = ptr::null_mut();
        // unsafe {
        //     if xlib::XFetchName(display, *self, &mut str_ptr) == 0 {
        //         return Err("unable to get name for window");
        //     }
        //     let cstr = CStr::from_ptr(str_ptr);
        //     let string = cstr.to_string_lossy().to_string();
        //     xlib::XFree(str_ptr as *mut c_void);
        //     return Ok(string);
        // }
        let v = self.x11_get_text_list_property(display, WMName.to_xlib_atom(display))?;
        match v.get(0) {
            Some(wm_name) => return Ok(wm_name.to_owned()),
            None => return Err("unable to get name for window"),
        }
        // let text_prop: MaybeUninit<xlib::XTextProperty> = MaybeUninit::uninit();
        // unsafe {
        //     if xlib::XGetWMName(display, *self, text_prop.as_mut_ptr()) == 0 {
        //         return Err("unable to get wm name for window");
        //     }
        //     let text_prop = text_prop.assume_init();

        // }

    }

    fn x11_wm_normal_hints(&self, display: *mut xlib::Display) -> Result<(xlib::XSizeHints, c_long), String> {
        unsafe {
            let mut size_hints: MaybeUninit<xlib::XSizeHints> = MaybeUninit::uninit();
            let mut supplied_hints: c_long = 0;
            match xlib::XGetWMNormalHints(display, *self, size_hints.as_mut_ptr(), &mut supplied_hints) {
                0 => return Err(String::from("Unable to retrieve WMNormalHints")),
                _ => return Ok((size_hints.assume_init(), supplied_hints)),
            }
        }
    }
}
