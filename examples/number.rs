use lavan::parser::{sources::*, traits::Parser};

fn main() {
    let input = "69420";
    // stream starting at index 0
    let mut stream = (input, 0);

    let output: Option<u32> = 
        // any ascii digit
        any_if(|c: &char| c.is_ascii_digit())
        // repeat until you can't find an ascii digit
        .repeat()
        // collect it into a string
        .collect::<String>()
        // then parse the string into a u32
        .then(|string| string.parse::<u32>().ok())
        // return the response (in this case, a Option<u32>)
        .parse_stream(&mut stream);

    // crash the program if parsing failed
    let number: u32 = output.unwrap();
    
    // print the number
    println!("{number}");
    assert_eq!(input, number.to_string());
}
