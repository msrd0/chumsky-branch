# chumsky-branch ![License](https://img.shields.io/crates/l/chumsky-branch) [![chumsky-branch on crates.io](https://img.shields.io/crates/v/chumsky-branch)](https://crates.io/crates/chumsky-branch) [![chumsky-branch on docs.rs](https://docs.rs/chumsky-branch/badge.svg)](https://docs.rs/chumsky-branch) [![Source Code Repository](https://img.shields.io/badge/Code-On%20github.com-blue)](https://github.com/msrd0/chumsky-branch)

This crate defines three parsing combinators for the [chumsky parsing library][__link0]:

 - [`not_starting_with`][__link1]: This combinator takes a list of patterns, and matches the shortest string from the input that diverges from all patterns.
 - [`not_containing`][__link2]: This combinator takes a list of patterns, and any string that does not contain any of the patterns.
 - [`branch`][__link3]: This combinator allows branching into a parser. Each branch defines two parsers. When the first parser matches, it chooses that branch and that branch only, even if the second parser fails. The second parser is then used to produce the output type. You can combine as many branches as you want (similar to `if else`). Then, you have to define an else branch which just takes a `String` and needs to produce output from that. Useful if you want to parse verbatim input plus some syntax.


## Example


```rust
use chumsky::prelude::*;
use chumsky_branch::prelude::*;

#[derive(Debug, Eq, PartialEq)]
enum Token {
	Placeholder(String),
	Comment(String),
	Verbatim(String)
}

impl Token {
	fn lexer() -> impl Parser<char, Self, Error = Simple<char>> {
		branch(
			"{{",
			text::ident().then_ignore(just("}}")).map(Self::Placeholder)
		)
		.or_branch(
			"/*",
			not_containing(["*/"])
				.then_ignore(just("*/"))
				.map(Self::Comment)
		)
		.or_else(Self::Verbatim)
	}
}

fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
	Token::lexer().repeated().then_ignore(end())
}

let input = "/* Greet the user */Hello {{name}}!";
assert_eq!(&lexer().parse(input).unwrap(), &[
	Token::Comment(" Greet the user ".to_owned()),
	Token::Verbatim("Hello ".to_owned()),
	Token::Placeholder("name".to_owned()),
	Token::Verbatim("!".to_owned())
]);
```


 [__cargo_doc2readme_dependencies_info]: ggGkYW0AYXSEG_yB7JfhovBLGygsATPTD76BG2F5aYNOA7k2G-77ogSBRovuYXKEG_ojqDs4RFzeG5AY-iY45EvCG3W9C5VbCNUrG6ZVVP55P6sGYWSCgmdjaHVtc2t5ZTAuOC4wg25jaHVtc2t5LWJyYW5jaGUwLjAuMG5jaHVtc2t5X2JyYW5jaA
 [__link0]: https://crates.io/crates/chumsky/0.8.0
 [__link1]: https://docs.rs/chumsky-branch/0.0.0/chumsky_branch/?search=chumsky_branch::not_starting_with
 [__link2]: https://docs.rs/chumsky-branch/0.0.0/chumsky_branch/?search=chumsky_branch::not_containing
 [__link3]: https://docs.rs/chumsky-branch/0.0.0/chumsky_branch/?search=chumsky_branch::branch
