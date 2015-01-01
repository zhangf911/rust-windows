use std::ptr;

use ll::types::HINSTANCE;

#[deriving(Copy)]
pub struct Instance {
    pub instance: HINSTANCE
}

impl Instance {
    pub fn main_instance() -> Instance {
        Instance {
            instance: unsafe { super::ll::all::GetModuleHandleW(ptr::null()) as HINSTANCE },
        }
    }
}
