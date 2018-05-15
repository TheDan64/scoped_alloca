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

// FIXME: Check if stack has enough space, Err if not
#[must_use] // Nightly only; warning otherwise
#[inline]
fn enough_stack_space(bytes: size_t) -> Result<(), ()> {
    Ok(())
}

#[inline(never)]
pub fn alloca_collect<T, I, R, F>(iter: I, f: F) -> Result<R, ()> where I: ExactSizeIterator<Item = T>, F: FnOnce(&mut [T]) -> R {
    let len = iter.len();
    let total_size = size_of::<T>() * len;

    enough_stack_space(total_size)?;

    let slice = unsafe { from_raw_parts_mut::<T>(c_alloca(total_size) as *mut T, len) };

    for (i, item) in iter.enumerate() {
        slice[i] = item;
    }

    // REVIEW: Should we catch panic and Err?
    Ok(f(slice))
}

#[inline(never)]
pub unsafe fn alloca_uninitialized_slice<T, F, R>(len: size_t, f: F) -> Result<R, ()> where F: FnOnce(&mut [T]) -> R {
    let total_size = size_of::<T>() * len;

    enough_stack_space(total_size)?;

    let slice = from_raw_parts_mut::<T>(c_alloca(total_size) as *mut T, len);

    // REVIEW: Should we catch panic and Err?
    Ok(f(slice))
}

#[inline(never)]
pub unsafe fn alloca_uninitialized_block<F, R>(bytes: size_t, f: F) -> Result<R, ()> where F: FnOnce(&mut c_void) -> R {
    enough_stack_space(bytes)?;

    let ptr = c_alloca(bytes);

    // REVIEW: Should we catch panic and Err?
    Ok(f(&mut *ptr))
}

#[cfg(test)]
mod tests {
    use super::{alloca_collect, alloca_uninitialized_slice};

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

    #[test]
    fn test_alloca_uninitialized_slice() {
        let res = unsafe { alloca_uninitialized_slice(4, |alloc| {
            alloc[0] = 1;
            alloc[1] = 3;
            alloc[2] = 5;
            alloc[3] = 7;

            alloc.iter().sum::<i32>()
        })};

        assert_eq!(res, Ok(16));
    }
}
