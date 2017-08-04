extern crate blockchain;

use blockchain::*;
use std::time::Duration;
use std::thread;

#[derive(Debug)]
pub struct AdderDefinition;

impl Definition for AdderDefinition {
    type Transaction = usize;
    type Extra = ();
    type Hash = usize;
    type WorldState = usize;
    type Block = AdderBlock;
    type TransitionRule = AdderTransitionRule;
    type Consensus = FakeConsensus<AdderBlock, ()>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdderBlock {
    pub number: usize,
    pub sum: usize
}

impl Default for AdderBlock {
    fn default() -> AdderBlock {
        AdderBlock {
            number: 0,
            sum: 0
        }
    }
}

impl Block for AdderBlock {
    type Transaction = usize;
    type Extra = ();
    type Hash = usize;

    fn next(&self, transactions: &[usize], extra: &[()], new_world_state_hash: usize) -> AdderBlock {
        AdderBlock {
            number: self.number + 1,
            sum: new_world_state_hash,
        }
    }

    fn unconsensusly_equal(&self, other: &AdderBlock) -> bool {
        self.eq(other)
    }
}

pub struct AdderTransitionRule;

impl TransitionRule for AdderTransitionRule {
    type Transaction = usize;
    type Extra = ();
    type WorldState = usize;

    fn transit(transaction: &usize, state: &usize) -> (usize, ()) {
        (*transaction + *state, ())
    }
}

fn main() {
    let mut blockchain: Blockchain<AdderDefinition> = Blockchain::default();

    loop {
        blockchain.mine(&[1]);
        println!("mined one block: {:?}", blockchain);

        thread::sleep(Duration::from_millis(1000));
    }
}
