fn main() {
    println!("Hello, world!");
    loop {
        loop {
            break;
        }
        break;
    }
    // 嵌套循环也不是必须需要 label
    let mut a = vec![1 ,2 ,3 ];
    let i = a.iter_mut();
    for s in i {
        *s = match s  {
            &mut 2 => 6,
            _ => 5,
        }
    }
    println!("a is {:?}", a);
   
}
