#![feature(test)]

extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

use ropey::Rope;

fn ropey_simple() {
    let mut rope = Rope::from_str("Hello 世界!");
    println!(
        "{}, len_bytes = {}, , len_chars = {}",
        rope,
        rope.len_bytes(),
        rope.len_chars()
    );

    let middle = rope.slice(3..8);
    println!("{}", middle);

    rope.remove(6..8);
    rope.insert(6, "world");
    println!(
        "{}, len_bytes = {}, , len_chars = {}",
        rope,
        rope.len_bytes(),
        rope.len_chars()
    );

    let ml_rope = Rope::from_str("Hello みんなさん!\nHow are you?\nThis text has multiple lines!");
    println!("{}", ml_rope.len_lines());
    println!(
        "{} {} {}",
        ml_rope.line_to_char(3),
        ml_rope.line_to_char(1),
        ml_rope.line_to_char(2)
    );
}

fn ropey_insert() {
    let mut rope = Rope::from_str("Hello 世界!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }

    #[bench]
    fn bench_rope(b: &mut Bencher) {
        b.iter(|| ropey_simple());
    }
}
