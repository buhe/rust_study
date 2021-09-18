struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(r: Rectangle) -> f32{
    let (w,h) = (r.p1.x - 0f32, r.p2.y - 0f32 ) ;
    w * h
}
fn main() {
    println!("Hello, world!");
    println!("rect area is {}", rect_area(Rectangle{p1: Point{x: 1.1, y: 1.2}, p2: Point{x: 2.1, y: 2.2}}))
}
