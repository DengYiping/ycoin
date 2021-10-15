pub trait Byteable {
    fn bytes(&self) -> Vec<u8>;
}

impl<T> Byteable for Vec<T> where T: Byteable {
    fn bytes(&self) -> Vec<u8> {
        self.iter()
            .flat_map(|nested| nested.bytes())
            .collect::<Vec<u8>>()
    }
}
