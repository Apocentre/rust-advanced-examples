pub trait Trait {
  fn run(&self);
  fn exit(&self);
}

pub trait Trait2 {
  fn run2(&self);
  fn exit2(&self);
}

#[derive(Copy, Clone)]
pub struct A;

#[derive(Copy, Clone)]
pub struct B;

impl Trait for A {
  fn run(&self) {
    println!("Run A Trait")
  }

  fn exit(&self) {
    println!("Exit A Trait")
  }
}

impl Trait for B {
  fn run(&self) {
    println!("Run B Trait")
  }

  fn exit(&self) {
    println!("Exit B Trait")
  }
}

impl Trait2 for A {
  fn run2(&self) {
    println!("Run A Trait2")
  }

  fn exit2(&self) {
    println!("Exit A Trait2")
  }
}

impl Trait2 for B {
  fn run2(&self) {
    println!("Run B Trait2")
  }

  fn exit2(&self) {
    println!("exit B Trait2")
  }
}

pub type Extra = (A, B);

pub struct Composite<T: Trait> {
  pub val: T,
}

pub struct Composite2<T: Trait2> {
  pub val: T,
}
