use std::fmt::Display;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"({},{})\n({},{})",self.0,self.1,self.2,self.3)
    }
}
fn t(m: Matrix) -> Matrix{
    let m2 = Matrix(m.0, m.2,m.1,m.3);
    m2
}
fn main() {
    println!("Hello, world!");
    let m = Matrix(1.1,1.2,2.1,2.2);
    println!("{}", m);
    println!("{}", t(m));
}
