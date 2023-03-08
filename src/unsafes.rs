use log::info;
use std::slice;

pub fn rawpointer() {
    let mut a = 10;
    let p1 = &a as *const i32;
    let p2 = &mut a as *mut i32;

    let address = 0x012345usize;
    let _r = address as *const i32;
    unsafe {
        info!("p1: {}", *p1);
        info!("p2: {}", *p2);

        // info!("r: {}", *r);
    }
}

fn split_at_mut(values: &mut [i32], idx: usize) -> (&mut [i32], &mut [i32]) {
    assert!(idx < values.len());
    let ptr = values.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, idx),
            slice::from_raw_parts_mut(ptr.add(idx), values.len() - idx),
        )
    }
}

pub fn slices() {
    let mut slice = vec![1, 2, 3, 4, 5];
    let (l, r) = split_at_mut(&mut slice, 3);
    info!("l: {:?}, r: {:?}", l, r);
}

extern "C" {
    fn abs(val: i32) -> i32;
}

pub fn extern_test() {
    let a = -5;
    unsafe {
        info!("{}", abs(a));
    }
}

static mut COUNT: u32 = 0;
