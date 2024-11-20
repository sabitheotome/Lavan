#![cfg(feature = "unstable")]
use lavan::{prelude::*, util::text::ascii};

fn main() {
    // an arbitrary email address
    let input = "sabloat69.420@archlinux.mango";

    // Single alphanum char <- Repeat One or More
    // Example: "sabloat69"
    let alphas = ascii::alphanumeric().del().repeat().min(1);

    // Alphanum sequence <- Repeat one or more separated by dots
    // Example: ["archlinux", "mango"]
    let name = alphas.repeat().min(1).separate_by('.').slice();

    // name'@'name
    let email = name.and('@').and(name).evaluate(input.chars());

    // username@hostname
    let (username, hostname) = email.unwrap();

    // assert correct username and hostname parsing
    assert_eq!(username, "sabloat69.420");
    assert_eq!(hostname, "archlinux.mango");
}
