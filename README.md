# chumsky-branch ![License](https://img.shields.io/crates/l/chumsky-branch) [![chumsky-branch on crates.io](https://img.shields.io/crates/v/chumsky-branch)](https://crates.io/crates/chumsky-branch) [![chumsky-branch on docs.rs](https://docs.rs/chumsky-branch/badge.svg)](https://docs.rs/chumsky-branch) [![Source Code Repository](https://img.shields.io/badge/Code-On%20github.com-blue)](https://github.com/msrd0/chumsky-branch) [![chumsky-branch on deps.rs](https://deps.rs/repo/github/msrd0/chumsky-branch/status.svg)](https://deps.rs/repo/github/msrd0/chumsky-branch)

branch combinator for the [chumsky parsing library][__link0].


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


 [__cargo_doc2readme_dependencies_info]: ggGkYW0AYXSEG52uRQSwBdezG6GWW8ODAbr5G6KRmT_WpUB5G9hPmBcUiIp6YXKEG5kZkLEeMasBG-4acMviWu2FGy-OFmDxPWHtG1A-UE3mhOxOYWSBgmdjaHVtc2t5ZTAuOC4w
 [__link0]: https://crates.io/crates/chumsky/0.8.0
