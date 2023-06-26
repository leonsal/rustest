mod strings;
mod structs;
mod vecs;

fn main() {
    println!("Hello, Rust!");

    strings::test1();
    vecs::test1();
    vecs::test2();
    vecs::test3();
    vecs::test4();

    structs::test1();
}
