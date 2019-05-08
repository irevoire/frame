struct Ether {
    dst_addr: [u8; 6],
    src_addr: [u8; 6],
    ether_type: u16,
}

impl Ether {
    pub fn new() -> Self {
        Ether {
            dst_addr: [0; 6],
            src_addr: [0; 6],
            ether_type: 0,
        }
    }

    pub fn src_addr(&mut self, addr: [u8; 6]) -> &mut Self {
        self.src_addr = addr;
        self
    }

    pub fn dst_addr(&mut self, addr: [u8; 6]) -> &mut Self {
        self.dst_addr = addr;
        self
    }

    pub fn ether_type(&mut self, ether_type: u16) -> &mut Self {
        self.ether_type = ether_type;
        self
    }
}

impl crate::field::Field for Ether {
    fn serialize(&self) -> &[u8] {
        &self.dst_addr
    }
}

struct EtherBuilder {
    ether: Ether,
}

impl EtherBuilder {
    pub fn new() -> Self {
        EtherBuilder {
            ether: Ether::new(),
        }
    }

    pub fn src_addr(mut self, addr: [u8; 6]) -> Self {
        self.ether.src_addr(addr);
        self
    }

    pub fn dst_addr(mut self, addr: [u8; 6]) -> Self {
        self.ether.dst_addr(addr);
        self
    }

    pub fn ether_type(mut self, ether_type: u16) -> Self {
        self.ether.ether_type(ether_type);
        self
    }

    pub fn build(self) -> Ether {
        self.ether
    }
}

#[cfg(test)]
mod tests {
    use crate::field::Field;

    #[test]
    fn builder() {
        let ether = crate::field::ether::EtherBuilder::new()
            .src_addr([0x00, 0x11, 0x22, 0x33, 0x44, 0x55])
            .dst_addr([0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff])
            .ether_type(42)
            .build();
        assert_eq!(ether.src_addr, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(ether.dst_addr, [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
        assert_eq!(ether.ether_type, 42);
    }
}
