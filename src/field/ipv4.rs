use derive_builder::Builder;

type IpAddr = [u8; 4];

#[repr(packed)]
#[derive(Builder)]
struct Ipv4 {
    /// version 4 bits | ihl 4 bits
    #[builder(default = "0b0100_0101")] // version 4 | header length = 20B
    version_ihl: u8,
    /// dscp 6 bits | ecn 2 bits
    #[builder(default = "0b000000_00")] // default 0 | no explicit congestion
    dscp_ecn: u8,
    #[builder(default = "0xffff")] // should be updated last
    total_len: u16,
    #[builder(default = "0x0000")]
    identification: u16,
    /// flags 3 bits | fragmentation offset 13 bits
    #[builder(default = "0b010_0000000000000")] // no fragmentation | frag offset = 0
    flags_fragmentation: u16,
    #[builder(default = "64")] // common default value
    ttl: u8,
    #[builder(default = "0x06")] // TCP by default
    protocol: u8,
    #[builder(default = "0x0000")] // should be updated last
    checksum: u16,
    src_addr: IpAddr,
    #[builder(default = "[0xFF, 0xFF, 0xFF, 0xFF]")] // broadcast
    dst_addr: IpAddr,
}

impl crate::field::Field for Ipv4 {
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
        let ip = crate::field::ipv4::Ipv4Builder::default()
            .src_addr([0xaa, 0xbb, 0xcc, 0xdd])
            .build()
            .unwrap();
        unsafe {
            assert_eq!(ip.src_addr, [0xaa, 0xbb, 0xcc, 0xdd]);
            assert_eq!(ip.dst_addr, [0xff, 0xff, 0xff, 0xff]);
            assert_eq!(ip.protocol, 0x06);
        }
    }

    #[test]
    fn serialize() {
        let ip = crate::field::ipv4::Ipv4Builder::default()
            .src_addr([0xaa, 0xbb, 0xcc, 0xdd])
            .build()
            .unwrap();
        assert_eq!(
            ip.serialize(),
            [
                0x45, // version ihl
                0x00, // dscp ecn
                0xff, 0xff, // total len
                0x00, 0x00, // identification
                0x00, 0x40, // flags fragmentation
                0x40, // ttl
                0x06, // protocol
                0x00, 0x00, // checksum
                0xaa, 0xbb, 0xcc, 0xdd, // src addr
                0xff, 0xff, 0xff, 0xff, // dst addr
            ]
        );
    }
}
