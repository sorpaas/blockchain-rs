use std::marker::PhantomData;

pub trait BlockHash<H: Copy> {
    fn parent_hash(&self) -> H;
    fn block_hash(&self) -> H;
}

pub trait BlockStore {
    type Hash: Copy;
    type Block: BlockHash<Self::Hash> + Ord;

    fn get(&self, hash: Self::Hash) -> &Self::Block;
    fn insert(&mut self, block: Self::Block);
}

pub struct Chain<H, B, S> {
    best_hash: H,
    store: S,
    _block_marker: PhantomData<B>,
}

impl<H: Copy, B: BlockHash<H> + Ord, S: BlockStore<Hash=H, Block=B>> Chain<H, B, S> {
    pub fn best(&self) -> &B {
        let best_hash = self.best_hash;
        self.get(best_hash)
    }

    pub fn get(&self, hash: H) -> &B {
        self.store.get(hash)
    }

    pub fn insert(&mut self, block: B) {
        let extern_hash = block.block_hash();
        let local_hash = self.best_hash;
        let best_hash = if &block > self.best() {
            extern_hash
        } else {
            local_hash
        };
        self.store.insert(block);
        self.best_hash = best_hash;
    }
}
