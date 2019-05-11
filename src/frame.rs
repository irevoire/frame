use crate::field::Field;

pub struct Frame {
    fields: Vec<Box<Field>>,
}

impl Frame {
    pub fn new() -> Self {
        Frame { fields: Vec::new() }
    }

    pub fn with(&mut self, f: Box<Field>) -> &mut Self {
        self.fields.push(f);
        self
    }

    pub fn raw(&self) -> Vec<u8> {
        self.fields
            .iter()
            .flat_map(|f| f.serialize())
            .map(|f| *f)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn builder() {
        let mut frame = crate::frame::Frame::new();

        frame
            .with(Box::new(
                crate::field::ether::EtherBuilder::default()
                    .src_addr([0x00, 0x11, 0x22, 0x33, 0x44, 0x55])
                    .build()
                    .unwrap(),
            ))
            .with(Box::new(
                crate::field::arp::ArpBuilder::default()
                    .sha([0x00, 0x11, 0x22, 0x33, 0x44, 0x55])
                    .spa([0xaa, 0xbb, 0xcc, 0xdd])
                    .build()
                    .unwrap(),
            ));

        assert_eq!(
            frame.raw(),
            vec![
                // ETHER
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // dst_addr
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, //src_addr
                0x00, 0x08, // ether_type: take care of the endianness
                // ARP
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
