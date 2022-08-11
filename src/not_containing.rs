use crate::not_starting_with;
use chumsky::prelude::*;

/// Parses any non-empty string that does not contain any of the patterns as a substring.
///
/// # Panics
///
/// If any of the patterns is the empty string (`""`).
pub fn not_containing<'a, I, E>(patterns: I) -> impl Parser<char, String, Error = E> + 'a
where
	I: IntoIterator<Item = &'a str>,
	E: chumsky::Error<char> + 'a
{
	not_starting_with(patterns)
		.repeated()
		.at_least(1)
		.map(|vecs| vecs.into_iter().flatten().collect())
}

#[cfg(test)]
mod tests {
	use super::*;

	fn test_lexer() -> impl Parser<char, String, Error = Simple<char>> {
		not_containing(["{%", "{{"]).then_ignore(end())
	}

	#[test]
	fn test_not_containing_other_chars() {
		let parsed = test_lexer().parse("foo");
		assert_eq!(parsed, Ok("foo".to_owned()));
	}

	#[test]
	fn test_not_containing_first_char() {
		let parsed = test_lexer().parse("foo{bar");
		assert_eq!(parsed, Ok("foo{bar".to_owned()));
	}

	#[test]
	fn test_not_containing_first_char_first() {
		let parsed = test_lexer().parse("{bar");
		assert_eq!(parsed, Ok("{bar".to_owned()));
	}

	#[test]
	fn test_not_containing_first_char_last() {
		let parsed = test_lexer().parse("foo{");
		assert_eq!(parsed, Ok("foo{".to_owned()));
	}

	#[test]
	fn test_not_containing_second_char() {
		let parsed = test_lexer().parse("foo%bar");
		assert_eq!(parsed, Ok("foo%bar".to_owned()));
	}

	#[test]
	fn test_not_containing_second_char_first() {
		let parsed = test_lexer().parse("%bar");
		assert_eq!(parsed, Ok("%bar".to_owned()));
	}

	#[test]
	fn test_not_containing_second_char_last() {
		let parsed = test_lexer().parse("foo%");
		assert_eq!(parsed, Ok("foo%".to_owned()));
	}

	#[test]
	fn test_containing_first_pattern() {
		let parsed = test_lexer().parse("foo{%bar");
		assert_matches!(parsed, Err(_));
	}

	#[test]
	fn test_containing_first_pattern_first() {
		let parsed = test_lexer().parse("{%bar");
		assert_matches!(parsed, Err(_));
	}

	#[test]
	fn test_containing_first_pattern_last() {
		let parsed = test_lexer().parse("foo{%");
		assert_matches!(parsed, Err(_));
	}

	#[test]
	fn test_containing_second_pattern() {
		let parsed = test_lexer().parse("foo{{bar");
		assert_matches!(parsed, Err(_));
	}

	#[test]
	fn test_containing_second_pattern_first() {
		let parsed = test_lexer().parse("{{bar");
		assert_matches!(parsed, Err(_));
	}

	#[test]
	fn test_containing_second_pattern_last() {
		let parsed = test_lexer().parse("foo{{");
		assert_matches!(parsed, Err(_));
	}
}
