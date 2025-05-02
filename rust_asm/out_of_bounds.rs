#[cfg(usafe)]
#[inline(never)]
pub fn check(array : &Vec<u32>) {
    unsafe {
        let _x = *array.get_unchecked(7);
    }
}

#[cfg(not(usafe))]
#[inline(never)]
pub fn check(array : &Vec<u32>) {
    // call    _ZN4core9panicking18panic_bounds_check17h883675bd61368af7E
    let _x = array[0];
}
