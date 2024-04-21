pub use crate::parser::sources::{any, any_eq, any_if, any_ne, eoi, take};
pub use crate::parser::traits::{Parse, Parser};
pub use crate::response::adapters::{sure::Sure, unsure::Unsure};
pub use crate::stream::adapters::SliceInput;
pub use crate::stream::adapters::StrInput;
pub(crate) use std::ops::ControlFlow::*;
