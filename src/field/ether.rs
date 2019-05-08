#[macro_use]
use derive_builder::Builder;

#[derive(Builder)]
struct Ether {
    dst_addr: [u8; 6],
    src_addr: [u8; 6],
    ether_type: u16,
}

impl crate::field::Field for Ether {
    fn serialize(&self) -> &[u8] {
        &self.dst_addr
    }
}

#[cfg(test)]
mod tests {
    use crate::field::Field;

    #[test]
    fn builder() {
        let ether = crate::field::ether::EtherBuilder::default()
            .src_addr([0x00, 0x11, 0x22, 0x33, 0x44, 0x55])
            .dst_addr([0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff])
            .ether_type(42)
            .build()
            .unwrap();
        assert_eq!(ether.src_addr, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(ether.dst_addr, [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
        assert_eq!(ether.ether_type, 42);
    }
}
