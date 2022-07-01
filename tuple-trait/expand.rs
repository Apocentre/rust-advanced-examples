#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod common {
    pub trait Trait {
        fn run(&self);
        fn exit(&self);
    }
    pub trait Trait2 {
        fn run(&self);
        fn exit(&self);
    }
    pub struct A;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for A {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for A {
        #[inline]
        fn clone(&self) -> A {
            {
                *self
            }
        }
    }
    pub struct B;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for B {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for B {
        #[inline]
        fn clone(&self) -> B {
            {
                *self
            }
        }
    }
    impl Trait for A {
        fn run(&self) {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["Run A Trait\n"], &[]));
            }
        }
        fn exit(&self) {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["Exit A Trait\n"], &[]));
            }
        }
    }
    impl Trait for B {
        fn run(&self) {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["Run B Trait\n"], &[]));
            }
        }
        fn exit(&self) {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["Exit B Trait\n"], &[]));
            }
        }
    }
    impl Trait2 for A {
        fn run(&self) {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["Run A Trait2\n"], &[]));
            }
        }
        fn exit(&self) {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["Exit A Trait2\n"], &[]));
            }
        }
    }
    impl Trait2 for B {
        fn run(&self) {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["Run B Trait2\n"], &[]));
            }
        }
        fn exit(&self) {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["exit B Trait2\n"], &[]));
            }
        }
    }
    pub type Extra = (A, B);
    pub struct Composite<T: Trait> {
        pub val: T,
    }
    pub struct Composite2<T: Trait2> {
        pub val: T,
    }
}
mod solution1 {
    use seq_macro::seq;
    use crate::common::{Trait, A, B};
    impl Trait for (A, B) {
        fn run(&self) {
            self.0.run();
            self.1.run();
        }
        fn exit(&self) {
            self.0.exit();
            self.1.exit();
        }
    }
}
mod solution2 {
    use crate::common::Trait2;
    impl<T: Trait2> Trait2 for (T, T) {
        fn run(&self) {
            let (A, B) = self;
            (A.run(), B.run());
        }
        fn exit(&self) {
            let (A, B) = self;
            (A.exit(), B.exit());
        }
    }
}
use crate::common::{Trait, Trait2, A, B, Composite, Composite2, Extra};
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["Solution 1\n"], &[]));
    };
    let a = A {};
    let b = B {};
    let val = (a, b);
    let comp = Composite::<Extra> { val };
    comp.val.run();
    comp.val.exit();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["------------------\n"],
            &[],
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["Solution 2\n"], &[]));
    };
    let val = (a, b);
    let comp = Composite2::<Extra> { val };
    comp.val.run();
}
