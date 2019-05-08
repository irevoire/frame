pub mod arp;
pub mod ether;
pub mod ipv4;

pub trait Field {
    fn serialize(&self) -> &[u8];
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}
