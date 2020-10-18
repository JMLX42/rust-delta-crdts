use std::collections::HashSet;

use crate::vclock::{VClock, Value};

use crate::traits::{Actor, DCRDT};

use dcrdt_macro::*;

#[derive(Debug)]
struct GCounter<A: Actor, V: Value> {
    owner: A,
    state: VClock<A, V>,
}

impl<A: Actor, V: Value> GCounter<A, V> {
    fn new(owner: A) -> Self {
        GCounter {
            owner,
            state: VClock::new(),
        }
    }

    #[dcrdt_mutator]
    fn inc(&self) -> VClock<A, V> {
        let mut res = VClock::new();

        res.dots.insert(
            self.owner.clone(),
            self.state.dots.get(&self.owner).unwrap_or(&V::zero()).to_owned() + V::one(),
        );

        res
    }
}

impl<A: Actor, V: Value> DCRDT<A> for GCounter<A, V> {
    type Delta = VClock<A, V>;
    type Value = V;

    fn join(s1: &Self::Delta, s2: &Self::Delta) -> Self::Delta {
        let mut actors: HashSet<&A> = HashSet::new();

        s1.dots.keys().for_each(|k| { actors.insert(k); });
        s2.dots.keys().for_each(|k| { actors.insert(k); });

        let mut delta = VClock::<A, V>::new();

        actors.iter().for_each(|actor| {
            let a = s1.dots.get(actor).unwrap_or(&Self::Value::zero()).to_owned();
            let b = s2.dots.get(actor).unwrap_or(&Self::Value::zero()).to_owned();
            let max = if a > b { a } else { b };

            delta.dots.insert((*actor).clone(), max);
        });

        delta
    }

    fn value(&self) -> Self::Value {
        self.state.dots.values().fold(V::zero(), |acc, x| acc + *x)
    }

    fn apply(&mut self, delta: &Self::Delta) -> Self::Delta {
        let new_state = Self::join(&self.state, delta);

        self.state = new_state.clone();

        new_state
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use num_traits::One;

    type Actor = String;
    type Value = u64;
    type Delta = VClock<Actor, Value>;

    #[test]
    fn test_new() {
        GCounter::<Actor, Value>::new("a".to_string());
    }

    #[test]
    fn test_inc() {
        let mut a = GCounter::<Actor, Value>::new("a".to_string());

        a.inc();

        assert_eq!(a.value(), Value::one());
    }

    #[test]
    fn test_apply() {
        let mut a = GCounter::<Actor, Value>::new("a".to_string());
        let mut b = GCounter::<Actor, Value>::new("b".to_string());
        let mut deltas: Vec<Vec<Delta>> = vec![vec![], vec![]];

        deltas[0].push(a.inc());
        deltas[0].push(a.inc());
        deltas[0].push(a.inc());
        deltas[1].push(b.inc());
        deltas[1].push(b.inc());

        deltas[0].iter().for_each(|d| { b.apply(d); });
        deltas[1].iter().for_each(|d| { a.apply(d); });

        dbg!(&a);
        dbg!(&b);

        assert_eq!(a.value(), b.value());
        assert_eq!(a.value(), 5u64);
    }
}
