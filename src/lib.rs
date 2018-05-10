extern crate libc;

use libc::{c_void, size_t};
use std::iter::Iterator;
use std::mem::size_of;
use std::slice::from_raw_parts_mut;

extern "C" {
    fn alloca(size: size_t) -> *mut c_void;
}

/// Alloca is uninitialized and f is not guarenteed to initialize it before modifying it
#[inline(never)]
pub unsafe fn scoped_alloca<'f, T: 'f, R, F: FnOnce(&'f mut [T]) -> R>(len: size_t, f: F) -> Result<R, ()> {
    let total_size = size_of::<T>() * len;
    // TODO: Check if stack has enough space, Err if not

    let slice = from_raw_parts_mut::<'f, T>(alloca(total_size) as *mut T, len);

    // TODO: catch panic and err?
    Ok(f(slice))
}

pub fn scoped_alloca_default<T: Default>(size: size_t) {

}

pub fn alloca_collect<T, I: Iterator<Item = T>, R, F: FnOnce(&mut [T]) -> R>(iter: I) -> Result<R, ()> {
    let total_size = size_of::<T>() * len;
    // TODO: Check if stack has enough space, Err if not

    let slice = from_raw_parts_mut::<T>(alloca(total_size) as *mut T, len);

    // TODO: catch panic and err?
    Ok(f(slice))
}


#[cfg(test)]
mod tests {
    use super::{scoped_alloca, alloca_collect};

    #[test]
    fn test_scoped_collect() {
        // fn inkwell_does_stuff(input: &[&InkwellValue]) {
        //     let mut input = Vec<LLVMValueRef> = input.iter().map(|val| val.as_value_ref()).collect();
        // collect_alloca(iter)
        let iter = vec![1, 2, 3, 4].iter().map(|v| v + 3);
        let res = alloca_collect(iter, |alloc| {

        });
    }

    #[test]
    fn test_scoped_alloca() {
        let res = scoped_alloca(5, |alloc| {

        });
    }
}