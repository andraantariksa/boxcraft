#[inline]
pub fn any_sized_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>()) }
}

#[inline]
pub fn any_slice_as_u8_slice<T>(p: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(p.as_ptr() as *const u8, p.len() * std::mem::size_of::<T>())
    }
}
