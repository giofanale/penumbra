mod three;
use three::{Split, Three};

mod leaf;
pub use leaf::Leaf;

mod node;
pub use node::Node;

mod rightmost;
pub use rightmost::Segment;

pub trait Height {
    const HEIGHT: usize;
}

pub trait Arboreal: Height
where
    Self: Sized,
{
    type Item;
    type Carry;

    fn singleton(item: Self::Item) -> Self;

    fn insert(self, item: Self::Item) -> Result<Self, (Self::Item, Self::Carry)>;
}

pub trait GetHash {
    fn hash(&self) -> Hash;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Hash;

impl Hash {
    fn leaf(height: usize, commitment: &Commitment) -> Hash {
        Hash
    }

    fn node(height: usize, a: Hash, b: Hash, c: Hash, d: Hash) -> Hash {
        Hash
    }
}

pub struct Commitment;
