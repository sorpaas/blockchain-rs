use std::marker::PhantomData;
use std::collections::HashMap;
use std::hash::Hash;

pub trait HeaderHash<H: Copy> {
    fn parent_hash(&self) -> H;
    fn header_hash(&self) -> H;
}

pub trait HeaderStore {
    type Hash: Copy;
    type Header: HeaderHash<Self::Hash> + Ord;

    fn fetch(&self, hash: Self::Hash) -> Option<&Self::Header>;
    fn put(&mut self, block: Self::Header);
}

impl<Ha: Copy + Eq + Hash, He: HeaderHash<Ha> + Ord> HeaderStore for HashMap<Ha, He> {
    type Hash = Ha;
    type Header = He;

    fn fetch(&self, hash: Ha) -> Option<&He> {
        self.get(&hash)
    }

    fn put(&mut self, header: He) {
        self.insert(header.header_hash(), header);
    }
}

pub struct Chain<H, B, S> {
    best_hash: H,
    store: S,
    _block_marker: PhantomData<B>,
}

impl<H: Copy, B: HeaderHash<H> + Ord, S: HeaderStore<Hash=H, Header=B> + Default> Chain<H, B, S> {
    pub fn new(genesis: B) -> Self {
        let best_hash = genesis.header_hash();
        let mut store = S::default();
        store.put(genesis);

        Self {
            best_hash, store,
            _block_marker: PhantomData,
        }
    }

    pub fn best(&self) -> &B {
        let best_hash = self.best_hash;
        self.fetch(best_hash).unwrap()
    }

    pub fn fetch(&self, hash: H) -> Option<&B> {
        self.store.fetch(hash)
    }

    pub fn put(&mut self, block: B) -> bool {
        if self.fetch(block.parent_hash()).is_none() {
            return false;
        }

        let extern_hash = block.header_hash();
        let local_hash = self.best_hash;
        let best_hash = if &block > self.best() {
            extern_hash
        } else {
            local_hash
        };
        self.store.put(block);
        self.best_hash = best_hash;

        return true;
    }
}
