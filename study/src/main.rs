use std::{fmt::{self, Display}, sync::Mutex};
#[derive(Debug)]
struct Demo {
  pub name: i32,
}
impl Display for Demo {
 fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
fn main() {
    let demo = Demo {name: 5};
    // let n = demo.name.lock().unwrap();
    // println!("e={:}", n);
    // println!("e2={:}", n);
    println!("{} days.", 31);

    println!("{:b} ", 31);

    println!("{}", demo);
}
