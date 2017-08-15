pub trait Consensus<H> {
    fn create_for(header: &H) -> Self;
    fn is_applicable(&self, header: &H) -> bool;
    fn mine(&self, header: &mut H);
    fn verify(&self, header: &H) -> bool;
}

pub struct FakeConsensus;

impl<H> Consensus<H> for FakeConsensus {
    fn create_for(header: &H) -> Self {
        FakeConsensus
    }

    fn is_applicable(&self, header: &H) -> bool {
        true
    }

    fn mine(&self, header: &mut H) {
        // Do nothing
    }

    fn verify(&self, header: &H) -> bool {
        true
    }
}
