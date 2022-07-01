#![allow(non_snake_case)]
use crate::common::{Trait2};

// https://stackoverflow.com/questions/56697029/is-there-a-way-to-impl-trait-for-a-tuple-that-may-have-any-number-elements
macro_rules! tuple_impls {
  ($($name:ident)+) => {
    impl<$($name: Trait2),+> Trait2 for ($($name,)+)
    {
        fn run2(&self) {
          let ($($name,)+) = self;
          ($($name.run2(),)+);
        }

        fn exit2(&self) {
          let ($($name,)+) = self;
          ($($name.exit2(),)+);
        }
    }
  };
}

tuple_impls! {A B}
