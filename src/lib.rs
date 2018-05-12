extern crate libc;

use libc::{c_void, size_t};
use std::iter::Iterator;
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

use std::fmt::Debug;

#[inline(never)]
pub fn alloca_collect<T: Debug, I: ExactSizeIterator<Item = T>, R, F: FnOnce(&mut [T]) -> R>(iter: I, f: F) -> Result<R, ()> {
    let len = iter.len();
    let total_size = size_of::<T>() * len;
    // println!("len: {}", len);
    // println!("total_size: {}", total_size);

    // TODO: Check if stack has enough space, Err if not

    let slice = unsafe { from_raw_parts_mut::<T>(c_alloca(total_size) as *mut T, len) };

    for (i, item) in iter.enumerate() {
        // println!("slice[{:?}] = {:?}", i, item);
        slice[i] = item;
    }

    // println!("Slice: {:?}", slice);

    // REVIEW: Should we catch panic and Err?
    Ok(f(slice))
}


#[cfg(test)]
mod tests {
    use super::{alloca_collect};

    #[test]
    fn test_alloca_collect() {
        let v = vec![1, 2, 3, 4];
        let iter = v.iter().map(|v| v + 2);
        let res = alloca_collect(iter, |alloc| {
            assert_eq!(alloc[0], 3, "alloc: {:?}", alloc);
            assert_eq!(alloc[1], 4);
            assert_eq!(alloc[2], 5);
            assert_eq!(alloc[3], 6);

            alloc.iter().sum::<i32>()
        });

        assert_eq!(res.unwrap(), 10);
    }

    // #[test]
    // fn test_scoped_alloca() {
    //     let res = scoped_alloca(5, |alloc| {

    //     });
    // }
}
