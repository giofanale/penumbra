use crate::prelude::*;

/// A witnessed hash of a commitment at the true leaf of a complete tree.
#[derive(Clone, Copy, PartialEq, Eq, Derivative, Serialize, Deserialize)]
#[derivative(Debug = "transparent")]
pub struct Item(Hash);

impl Item {
    /// Create a new `Item` from a [`Hash`](struct@Hash).
    pub fn new(hash: Hash) -> Self {
        Self(hash)
    }
}

impl GetHash for Item {
    #[inline]
    fn hash(&self) -> Hash {
        self.0
    }

    #[inline]
    fn cached_hash(&self) -> Option<Hash> {
        Some(self.0)
    }
}

impl Height for Item {
    type Height = Zero;
}

impl Complete for Item {
    type Focus = frontier::Item;
}

impl Witness for Item {
    #[inline]
    fn witness(&self, index: impl Into<u64>) -> Option<(AuthPath<Self>, Hash)> {
        debug_assert_eq!(index.into(), 0, "non-zero index when witnessing leaf");
        Some((path::Leaf, self.0))
    }
}

impl ForgetOwned for Item {
    fn forget_owned(self, index: impl Into<u64>) -> (Insert<Self>, bool) {
        debug_assert_eq!(index.into(), 0, "non-zero index when forgetting leaf");
        (Insert::Hash(self.0), true)
    }
}

impl Visit for Item {
    fn visit_indexed<V: Visitor>(&self, index: u64, visitor: &mut V) -> V::Output {
        visitor.complete_item(index, self)
    }
}

impl Traverse for Item {
    fn traverse<T: Traversal, V: Visitor>(
        &self,
        traversal: &mut T,
        visitor: &mut V,
        output: &mut impl FnMut(V::Output),
    ) {
        traversal.traverse_complete(visitor, output, self, visit::NO_CHILDREN);
    }
}
