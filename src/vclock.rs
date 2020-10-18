use std::collections::BTreeMap;
use std::cmp::PartialOrd;
use std::ops::Add;

use crate::traits::Actor;

use serde::{
    Serialize,
    Deserialize,
};

use num_traits::{
    One,
    Zero,
};

pub trait Value: PartialOrd + Add + One + Zero + Clone + Copy {}
impl<V: PartialOrd + Add + One + Zero + Clone + Copy> Value for V {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VClock<A: Actor, V: Value> {
    pub dots: BTreeMap<A, V>,
}

impl<A: Actor, V: Value> VClock<A, V> {
    pub fn new() -> Self {
        VClock {
            dots: BTreeMap::new(),
        }
    }
}

impl<A: Actor, V: Value> Default for VClock<A, V> {
    fn default() -> Self {
        Self::new()
    }
}
