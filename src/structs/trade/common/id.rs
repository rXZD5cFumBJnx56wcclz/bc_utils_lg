pub enum Id {
    Sequence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Hash32(pub [u8; 32]);
