use seq_macro::seq;
use crate::common::{Trait, A, B};

/// This is what we want our impl_trait to do
// impl Trait for Extra {
//   fn run(&self) {
//     self.0.run();
//     self.1.run();
//   }
// }

macro_rules! impl_Trait {
 ($count:literal, $($x:ident, )*) => {
    impl Trait for ($($x,)+) {
      fn run(&self) {
        seq!{ i in 0..$count {
          self.i.run();
        }}
      }

      fn exit(&self) {
        seq!{ i in 0..$count {
          self.i.exit();
        }}
      }
    }
  }
}

impl_Trait!(2, A, B,);
