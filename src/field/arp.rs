/// TODO: This file is broken.
/// We should **NOT** force the user to use the ip or ethernet stack.
/// Instead we need a way to allow to implements this function with your own *sized* type
use derive_builder::Builder;

#[allow(dead_code)]
#[repr(packed)]
#[derive(Builder)]
pub struct Arp {
    #[builder(default = "0x0001")] // ethernet
    htype: u16, // hardware type
    #[builder(default = "0x0008")] // ipv4
    ptype: u16, // protocol type
    #[builder(default = "0x06")] // 6 byte for a mac address
    hlen: u8, // hardware address len
    #[builder(default = "0x04")] // 4 byte for an ipv4 address
    plen: u8, // protocol address len
    #[builder(default = "0x0001")] // request
    oper: u16, // operation

    sha: crate::field::ether::MacAddr, // sender hardware address
    spa: crate::field::ipv4::Ipv4Addr, // sender protocol address
    #[builder(default = "[0xff, 0xff, 0xff, 0xff, 0xff, 0xff]")] // broadcast
    tha: crate::field::ether::MacAddr, // target hardware address
    #[builder(default = "[0xff, 0xff, 0xff, 0xff]")] // broadcast
    tpa: crate::field::ipv4::Ipv4Addr, // target protocol address
}

impl crate::field::Field for Arp {
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
        let arp = crate::field::arp::ArpBuilder::default()
            .sha([0x00, 0x11, 0x22, 0x33, 0x44, 0x55])
            .spa([0xaa, 0xbb, 0xcc, 0xdd])
            .build()
            .unwrap();
        unsafe {
            assert_eq!(arp.spa, [0xaa, 0xbb, 0xcc, 0xdd]);
            assert_eq!(arp.tpa, [0xff, 0xff, 0xff, 0xff]);
            assert_eq!(arp.oper, 0x0001);
        }
    }

    #[test]
    fn serialize() {
        let arp = crate::field::arp::ArpBuilder::default()
            .sha([0x00, 0x11, 0x22, 0x33, 0x44, 0x55])
            .spa([0xaa, 0xbb, 0xcc, 0xdd])
            .build()
            .unwrap();
        assert_eq!(
            arp.serialize(),
            [
                0x01, 0x00, // hardware type
                0x08, 0x00, // protocol type
                0x06, // hardware address len
                0x04, // protocol address len
                0x01, 0x00, // operation
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, // src addr
                0xaa, 0xbb, 0xcc, 0xdd, // src addr
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // dst addr
                0xff, 0xff, 0xff, 0xff, // dst addr
            ]
        );
    }
}
