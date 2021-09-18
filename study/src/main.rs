use std::sync::Mutex;
#[derive(Debug)]
struct Demo {
   name: Mutex<i32>,
}
fn main() {
    let demo = Demo {name: Mutex::new(5)};
    // let n = demo.name.lock().unwrap();
    // println!("e={:}", n);
    // println!("e2={:}", n);
    println!("{} days.", 31);

    println!("{:b} ", 31);

    println!("{:?}", demo);
}
