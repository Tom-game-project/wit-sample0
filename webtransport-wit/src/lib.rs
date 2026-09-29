#![no_std]

extern crate alloc;
use alloc::format;
use alloc::string::String;
use core::alloc::Layout;

#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cabi_realloc(
    old_ptr: *mut u8,
    old_size: usize,
    align: usize,
    new_size: usize,
) -> *mut u8 {
    unsafe {
        if new_size == 0 {
            if old_size > 0 {
                let layout = Layout::from_size_align_unchecked(old_size, align);
                alloc::alloc::dealloc(old_ptr, layout);
            }
            return align as *mut u8;
        }

        let ptr = if old_size == 0 {
            let layout = Layout::from_size_align_unchecked(new_size, align);
            alloc::alloc::alloc(layout)
        } else {
            let layout = Layout::from_size_align_unchecked(old_size, align);
            alloc::alloc::realloc(old_ptr, layout, new_size)
        };

        if ptr.is_null() {
            core::arch::wasm32::unreachable();
        }

        ptr
    }
}

wit_bindgen::generate!({
    world: "my-world",
    with: { "tombind:webtransport/webtransport": generate },
    path: "./wit",
});

use exports::component::tom::user_funcs::Guest as MainGuest;
use tombind::webtransport::webtransport::log;

struct Component;

impl MainGuest for Component {
    fn hello_world(name: String,) -> String {
        log(&name);
        format!("Hello, {name}! (no_std wasm)")
    }
}

export!(Component);
