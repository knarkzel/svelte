mod parser;

fn main() {
    let input = include_str!("../input.svelte");
    let tag = parser::parse(input);
    dbg!(&tag);
}
