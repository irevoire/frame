use derive_builder::Builder;

type MacAddr = [u8; 6];

#[repr(packed)]
#[derive(Builder)]
struct Ether {
    #[builder(default = "[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]")] // broadcast
    dst_addr: MacAddr,
    src_addr: MacAddr,
    #[builder(default = "0x0800")] // ipv4
    ether_type: u16,
}

impl crate::field::Field for Ether {
    fn serialize(&self) -> &[u8] {
        // should be safe since everything is aligned
        unsafe { crate::field::any_as_u8_slice(self) }
    }
}

#[cfg(test)]
mod tests {
    use crate::field::Field;

    #[test]
    fn builder() {
        let ether = crate::field::ether::EtherBuilder::default()
            .dst_addr([0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff])
            .src_addr([0x00, 0x11, 0x22, 0x33, 0x44, 0x55])
            .ether_type(0x42)
            .build()
            .unwrap();
        unsafe {
            assert_eq!(ether.src_addr, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
            assert_eq!(ether.dst_addr, [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
            assert_eq!(ether.ether_type, 0x42);
        }
    }

    #[test]
    fn serialize() {
        let ether = crate::field::ether::EtherBuilder::default()
            .dst_addr([0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff])
            .src_addr([0x00, 0x11, 0x22, 0x33, 0x44, 0x55])
            .ether_type(0x42)
            .build()
            .unwrap();
        assert_eq!(
            ether.serialize(),
            [
                0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, // dst_addr
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, //src_addr
                0x42, 0x00, // ether_type: take care of the endianness
            ]
        );
    }
}
