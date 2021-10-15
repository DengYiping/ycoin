use super::primitives::*;
use super::byteable::Byteable;

pub trait Hashable {
    fn hash(&self) -> YHash;
}

impl<T> Hashable for T
where T: Byteable
{
    fn hash(&self) -> YHash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}

