use std::marker::PhantomData;

pub trait Consensus {
    type Block: Block;
    type Extra;

    fn mine(block: &mut Self::Block);
    fn verify(block: &Self::Block) -> bool;
}

#[derive(Debug, Clone)]
pub struct FakeConsensus<B, E> {
    block: PhantomData<B>,
    extra: PhantomData<E>,
}

impl<B, E> Default for FakeConsensus<B, E> {
    fn default() -> Self {
        Self {
            block: PhantomData,
            extra: PhantomData,
        }
    }
}

impl<B: Block, E> Consensus for FakeConsensus<B, E> {
    type Block = B;
    type Extra = E;

    fn mine(block: &mut Self::Block) {
        // Do nothing
    }

    fn verify(block: &Self::Block) -> bool {
        true
    }
}

pub trait Hashable<T> {
    fn hash(&self) -> T;
}

impl<T: Copy> Hashable<T> for T {
    fn hash(&self) -> T {
        *self
    }
}

pub trait Block {
    type Transaction;
    type Extra;
    type Hash;

    fn next(
        &self, transactions: &[Self::Transaction], extra: &[Self::Extra],
        new_world_state_hash: Self::Hash
    ) -> Self;
    fn is_next(
        &self, other: &Self,
        transactions: &[Self::Transaction], extra: &[Self::Extra],
        new_world_state_hash: Self::Hash) -> bool;
}

pub trait TransitionRule {
    type Block;
    type Transaction;
    type Extra;
    type WorldState;

    fn transit(
        current_block: &Self::Block,
        transaction: &Self::Transaction, state: &Self::WorldState
    ) -> (Self::WorldState, Self::Extra);
}

pub trait Definition {
    type Transaction;
    type Extra;
    type Hash;
    type WorldState: Hashable<Self::Hash>;
    type Block: Block<Transaction=Self::Transaction, Extra=Self::Extra, Hash=Self::Hash>;
    type TransitionRule: TransitionRule<Block=Self::Block, Transaction=Self::Transaction, Extra=Self::Extra, WorldState=Self::WorldState>;
    type Consensus: Consensus<Block=Self::Block, Extra=Self::Extra>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Blockchain<D: Definition> {
    current_block: D::Block,
    current_world_state: D::WorldState,
}

impl<H, W: Hashable<H> + Default, T, E, B: Block<Transaction=T, Extra=E, Hash=H> + Default, D: Definition<WorldState=W, Transaction=T, Extra=E, Hash=H, Block=B>> Default for Blockchain<D> {
    fn default() -> Self {
        Self {
            current_world_state: W::default(),
            current_block: B::default(),
        }
    }
}

impl<D: Definition> Blockchain<D> {
    pub fn new(block: D::Block, world_state: D::WorldState) -> Self {
        Self {
            current_block: block,
            current_world_state: world_state,
        }
    }

    pub fn current_world_state(&self) -> &D::WorldState {
        &self.current_world_state
    }

    pub fn current_block(&self) -> &D::Block {
        &self.current_block
    }

    pub fn mine(&mut self, transactions: &[D::Transaction]) {
        let mut extras = Vec::new();
        let mut world_state = None;

        for transaction in transactions {
            let ret = D::TransitionRule::transit(&self.current_block, transaction, if world_state.is_none() {
                &self.current_world_state
            } else {
                world_state.as_ref().unwrap()
            });
            world_state = Some(ret.0);
            extras.push(ret.1);
        }

        self.current_block = self.current_block.next(
            &transactions, &extras, world_state.as_ref().unwrap().hash());
        D::Consensus::mine(&mut self.current_block);
        self.current_world_state = world_state.unwrap();
    }

    pub fn verify(&mut self, block: D::Block, transactions: &[D::Transaction]) -> bool {
        let mut extras = Vec::new();
        let mut world_state = None;

        for transaction in transactions {
            let ret = D::TransitionRule::transit(&self.current_block, transaction, if world_state.is_none() {
                &self.current_world_state
            } else {
                world_state.as_ref().unwrap()
            });
            world_state = Some(ret.0);
            extras.push(ret.1);
        }

        if self.current_block.is_next(
            &block, transactions, &extras, world_state.as_ref().unwrap().hash()) {
            self.current_block = block;
            self.current_world_state = world_state.unwrap();

            true
        } else {
            false
        }
    }
}
