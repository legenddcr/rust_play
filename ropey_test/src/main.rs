use ropey::Rope;

fn main() {
    let mut rope = Rope::from_str("Hello 世界!");
    println!("{}, len_bytes = {}, , len_chars = {}", 
        rope, rope.len_bytes(), rope.len_chars());

    let middle = rope.slice(3..8);
    println!("{}", middle);


    rope.remove(6..8);
    rope.insert(6, "world");
    println!("{}, len_bytes = {}, , len_chars = {}", 
        rope, rope.len_bytes(), rope.len_chars());

    let ml_rope = Rope::from_str("Hello みんなさん!\nHow are you?\nThis text has multiple lines!");
    println!("{}", ml_rope.len_lines());
    println!("{} {} {}", ml_rope.line_to_char(3), ml_rope.line_to_char(1), ml_rope.line_to_char(2));
}
