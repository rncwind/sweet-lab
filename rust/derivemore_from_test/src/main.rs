use derive_more::From;

#[derive(From)]
struct A {
    x: i32,
    y: String,
}

#[derive(From)]
struct B {
    i: i32,
    j: String,
}

fn main() {
    let a = A {
        x: 0xAA,
        y: "structa".to_string(),
    };
    let b = B {
        i: 0xBB,
        j: "structb".to_string(),
    };
    let a_as_b: B = a.into();
    println!("Hello, world!");
}
