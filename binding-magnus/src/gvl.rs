use std::mem::MaybeUninit;

/// # Safety
///
/// Must be called on a thread with the GVL aquired.
/// While the GVL is released *do not* call into ruby code.
pub unsafe fn without_gvl<F: FnOnce() -> R, R>(f: F) -> R {
    extern "C" fn gvl_fn<F: FnOnce() -> R, R>(ptr: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
        let (f, return_val) = unsafe { std::ptr::read(ptr.cast::<(F, &mut MaybeUninit<R>)>()) };
        return_val.write(f());
        std::ptr::null_mut()
    }

    let f = MaybeUninit::new(f);
    let mut return_val = MaybeUninit::uninit();

    unsafe {
        let mut gvl_fn_arg = (f, &mut return_val);
        rb_sys::rb_thread_call_without_gvl(
            Some(gvl_fn::<F, R>),
            std::ptr::from_mut(&mut gvl_fn_arg).cast(),
            // https://github.com/ruby/ruby/blob/8ab517698a76238d0afc7669b0ea1d5c6c1c0391/include/ruby/internal/intern/thread.h#L377-L382
            std::mem::transmute::<isize, rb_sys::rb_unblock_function_t>(-1),
            std::ptr::null_mut(),
        );
    }

    unsafe { return_val.assume_init() }
}
