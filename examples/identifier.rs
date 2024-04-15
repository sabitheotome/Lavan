use lavan::prelude::*;

fn main() {
    let input = "_identifier0123456789_abc123_789";

    let output: Option<String> = 
        // first char: any ascii alphanumeric or any underscore
        any_if(char::is_ascii_alphabetic).or(any_eq('_'))
        // And then, the subsequent chars
        .and(
            // any ascii alphanumeric or any underscore
            any_if(char::is_ascii_alphanumeric).or(any_eq('_'))
            // repeat until the condition is false
            .repeat()
            // collect it into a string
            .collect::<String>()
        )
        // concatenate the first char in the beginning of the string
        .map(|(first, tail)| format!("{first}{tail}"))
        // return the response (Option<String>)
        .evaluate(input);

    // crash the program if parsing failed
    let identifier: String = output.unwrap();
    
    // print the identifier
    println!("{identifier}");
    assert_eq!(input, identifier);
}
