fn main() {
    let a: i32 = 1;

    println!("{}", a);

    println!("{}", a);

    let b = 2;
    println!("{}", b);

    let f = 1 as f64 + 2.0;

    let g = false;

    println!("{}", g);

    let t1 = (1, false, 3.9);

    println!("{:?}", t1);
    let (x, y, z) = t1;
    println!("{}", x);

    let l1 = [1, 2, 3];
    let l3 = &l1[0..];

    println!("{:?}", l3);

    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);

    println!("{:?}", v3);

    let x = v3.pop();
    println!("{:?}", x);

    let s1 = "Rust";
    let s2 = "java".to_string();
}
