use crate::not_containing;
use chumsky::prelude::*;

/// The return type of [branch].
#[allow(clippy::type_complexity)]
pub struct Branch<'a, T, E> {
	branches: Vec<(&'a str, Box<dyn Parser<char, T, Error = E> + 'a>)>
}

/// Branch into a parser when and only when encountering the `begin` pattern.
pub fn branch<'a, P, T>(begin: &'a str, parser: P) -> Branch<'a, T, P::Error>
where
	P: Parser<char, T> + 'a
{
	Branch {
		branches: vec![(begin, Box::new(parser))]
	}
}

impl<'a, T, E> Branch<'a, T, E>
where
	E: chumsky::Error<char> + 'a
{
	/// Branch into another parser if the first branch didn't match.
	pub fn or_branch<P>(mut self, begin: &'a str, parser: P) -> Self
	where
		P: Parser<char, T, Error = E> + 'a
	{
		self.branches.push((begin, Box::new(parser)));
		self
	}

	/// Add an else branch if non of the branches succeeded.
	pub fn or_else<B>(self, branch: B) -> Box<dyn Parser<char, T, Error = E> + 'a>
	where
		B: Fn(String) -> T + 'a,
		T: 'a
	{
		let mut parser: Box<dyn Parser<char, T, Error = E> + 'a> = Box::new(
			not_containing(
				self.branches
					.iter()
					.map(|(pattern, _)| *pattern)
					.collect::<Vec<_>>()
			)
			.map(branch)
		);

		for (pattern, branch) in self.branches {
			parser = Box::new(choice((parser, just(pattern).ignore_then(branch))));
		}

		parser
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug, Eq, PartialEq)]
	enum Token {
		Foo,
		Bar,
		Comment,
		Verbatim(String)
	}

	fn test_lexer() -> impl Parser<char, Token, Error = Simple<char>> {
		branch(
			"{{",
			just("foo").then_ignore(just("}}")).map(|_| Token::Foo)
		)
		.or_branch(
			"{%",
			just("bar").then_ignore(just("%}")).map(|_| Token::Bar)
		)
		.or_branch(
			"/*",
			just("TODO").then_ignore(just("*/")).map(|_| Token::Comment)
		)
		.or_else(Token::Verbatim)
		.then_ignore(end())
	}

	#[test]
	fn parse_foo() {
		let token = test_lexer().parse("{{foo}}");
		assert_eq!(token, Ok(Token::Foo));
	}

	#[test]
	fn parse_bar() {
		let token = test_lexer().parse("{%bar%}");
		assert_eq!(token, Ok(Token::Bar));
	}

	#[test]
	fn parse_comment() {
		let token = test_lexer().parse("/*TODO*/");
		assert_eq!(token, Ok(Token::Comment));
	}

	#[test]
	fn parse_verbatim() {
		let token = test_lexer().parse("just some random text");
		assert_eq!(
			token,
			Ok(Token::Verbatim("just some random text".to_owned()))
		);
	}

	#[test]
	fn parse_foo_unclosed() {
		let token = test_lexer().parse("{{foo}");
		assert_matches!(token, Err(_));
	}

	#[test]
	fn parse_bar_unclosed() {
		let token = test_lexer().parse("{%foo%");
		assert_matches!(token, Err(_));
	}

	#[test]
	fn parse_comment_unclosed() {
		let token = test_lexer().parse("/*TODO//");
		assert_matches!(token, Err(_));
	}

	#[test]
	fn parse_invalid() {
		let token = test_lexer().parse("foo{{bar");
		assert_matches!(token, Err(_));
	}
}
