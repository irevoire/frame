use crate::field;

pub struct Frame<'a> {
    fields: Vec<&'a crate::field::Field>,
}

impl<'a> Frame<'a> {
    pub fn new() -> Self {
        Frame { fields: Vec::new() }
    }

    pub fn with(&mut self, f: &'a field::Field) -> &Self {
        self.fields.push(f);
        self
    }
}
