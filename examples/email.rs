use lavan::prelude::*;

fn main() {
    let input = "es4fbero15181@r65dgh51.com";

    let names_and_dots = any_if(char::is_ascii_alphanumeric)
        .ignore()
        .repeat_min(1)
        .repeat_min(1)
        .separated_by(any_eq('.').ignore())
        .slice();

    let email: Option<(&str, &str)> = names_and_dots
        .as_ref()
        .and(any_eq('@').ignore())
        .and(names_and_dots.as_ref())
        .evaluate(input);

    let (username, hostname) = email.unwrap();
    assert_eq!(username, "es4fbero15181");
    assert_eq!(hostname, "r65dgh51.com");
}
