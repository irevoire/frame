pub mod ether;

pub trait Field {
    fn serialize(&self) -> &[u8];
}
