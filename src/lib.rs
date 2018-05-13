extern crate libc;

use libc::{c_void, size_t};
use std::mem::size_of;
use std::slice::from_raw_parts_mut;

#[link(name = "alloca")]
extern "C" {
    #[no_mangle]
    #[inline(always)]
    fn c_alloca(_: size_t) -> *mut c_void;
}

/// Alloca is uninitialized and f is not guarenteed to initialize it before modifying it
// pub unsafe fn scoped_alloca<'f, T: 'f, R, F: FnOnce(&'f mut [T]) -> R>(len: size_t, f: F) -> Result<R, ()> {
//     let total_size = size_of::<T>() * len;
//     // TODO: Check if stack has enough space, Err if not

//     let slice = from_raw_parts_mut::<'f, T>(c_alloca(total_size) as *mut T, len);

//     // TODO: catch panic and err?
//     Ok(f(slice))
// }

// pub fn scoped_alloca_default<T: Default>(size: size_t) {
// }

#[inline(never)]
pub fn alloca_collect<T, I, R, F>(iter: I, f: F) -> Result<R, ()> where I: ExactSizeIterator<Item = T>, F: FnOnce(&mut [T]) -> R {
    let len = iter.len();
    let total_size = size_of::<T>() * len;

    // TODO: Check if stack has enough space, Err if not

    let slice = unsafe { from_raw_parts_mut::<T>(c_alloca(total_size) as *mut T, len) };

    for (i, item) in iter.enumerate() {
        slice[i] = item;
    }

    // REVIEW: Should we catch panic and Err?
    Ok(f(slice))
}


#[cfg(test)]
mod tests {
    use super::{alloca_collect};

    #[test]
    fn test_alloca_collect() {
        let v = vec![1, 2, 3, 4];
        let iter = v.iter().map(|v| v + 4);
        let res = alloca_collect(iter, |alloc| {
            assert_eq!(alloc[0], 5, "alloc: {:?}", alloc);
            assert_eq!(alloc[1], 6);
            assert_eq!(alloc[2], 7);
            assert_eq!(alloc[3], 8);

            alloc.iter().sum::<i32>()
        });

        assert_eq!(res, Ok(26));
    }
}
