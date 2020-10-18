use std::{
    hash::Hash,
};

pub trait Actor: Ord + Clone + Hash {}
impl<A: Ord + Clone + Hash> Actor for A {}

pub trait DCRDT<A: Actor> {
    type Delta: Default + Clone;
    type Value;
    
    fn join(s1: &Self::Delta, s2: &Self::Delta) -> Self::Delta;
    fn value(&self) -> Self::Value;
    fn apply(&mut self, delta: &Self::Delta) -> Self::Delta;
}
