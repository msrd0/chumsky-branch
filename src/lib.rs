#![allow(clippy::tabs_in_doc_comments)]
#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(elided_lifetimes_in_paths, unsafe_code)]

//! branch combinator for the [chumsky parsing library](chumsky).
//!
//! # Example
//!
//! ```rust
//! use chumsky::prelude::*;
//! use chumsky_branch::prelude::*;
//!
//! #[derive(Debug, Eq, PartialEq)]
//! enum Token {
//! 	Placeholder(String),
//! 	Comment(String),
//! 	Verbatim(String)
//! }
//!
//! impl Token {
//! 	fn lexer() -> impl Parser<char, Self, Error = Simple<char>> {
//! 		branch(
//! 			"{{",
//! 			text::ident().then_ignore(just("}}")).map(Self::Placeholder)
//! 		)
//! 		.or_branch(
//! 			"/*",
//! 			not_containing(["*/"])
//! 				.then_ignore(just("*/"))
//! 				.map(Self::Comment)
//! 		)
//! 		.or_else(Self::Verbatim)
//! 	}
//! }
//!
//! fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
//! 	Token::lexer().repeated().then_ignore(end())
//! }
//!
//! let input = "/* Greet the user */Hello {{name}}!";
//! assert_eq!(&lexer().parse(input).unwrap(), &[
//! 	Token::Comment(" Greet the user ".to_owned()),
//! 	Token::Verbatim("Hello ".to_owned()),
//! 	Token::Placeholder("name".to_owned()),
//! 	Token::Verbatim("!".to_owned())
//! ]);
//! ```

#[cfg(test)]
macro_rules! assert_matches {
	($expr:expr, $pat:pat) => {
		let expr = $expr;
		assert!(
			matches!(expr, $pat),
			"Assertion failed: expected {} to match {}, but found {:?}",
			stringify!($expr),
			stringify!($pat),
			expr
		);
	};
}

mod branch;
mod not_containing;
mod not_starting_with;

pub use branch::{branch, Branch};
pub use not_containing::not_containing;
pub use not_starting_with::not_starting_with;

pub mod prelude {
	pub use crate::{
		branch::branch, not_containing::not_containing,
		not_starting_with::not_starting_with
	};
}
