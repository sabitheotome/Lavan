use lavan::parser::{
    sources::{any, any_if},
    traits::*,
};

fn main() {
    let input = "testA0";
    let mut stream = (input, 0usize);

    let _x = any().ignore().map(|| 1).parse_stream(&mut stream);

    let result = any_if(|c: &char| c.is_alphabetic() || *c == '_')
        .and(
            any_if(|c: &char| c.is_alphanumeric() || *c == '_')
                .repeat()
                .collect::<String>(),
        )
        .map(|(first, mut tail)| {
            tail.insert(0, first);
            tail
        })
        .parse_stream(&mut stream);

    println!("{result:?}")
}
