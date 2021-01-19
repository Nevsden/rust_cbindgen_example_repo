use project::{Point,mult_two};

/// Takes a floating-value and multiplies by two
#[no_mangle]
pub extern "C" fn ext_mult_two(x: f32) -> f32 {
    mult_two(x)
}

/// Takes a floating-value and multiplies by two
#[no_mangle]
pub extern "C" fn ext_point_mult_two(ptr_point: *mut Point) {
    let point = get_point_mut(ptr_point);
    point.mult_two();
}

fn get_point_mut<'a>(ptr: *mut Point) -> &'a mut Point {
    assert!(!ptr.is_null());
    unsafe { &mut *ptr }
}