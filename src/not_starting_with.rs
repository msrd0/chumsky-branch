use chumsky::prelude::*;
use std::collections::HashMap;

enum Subpatterns<'a> {
	Last,
	Subpatterns(Vec<&'a str>)
}

impl<'a> Subpatterns<'a> {
	fn new(subpat: &'a str) -> Self {
		match subpat.len() {
			0 => Self::Last,
			_ => Self::Subpatterns(vec![subpat])
		}
	}

	fn push(&mut self, subpat: &'a str) {
		match (self, subpat.len()) {
			(Self::Last, _) => {},
			(this, 0) => *this = Self::Last,
			(Self::Subpatterns(subpats), _) => subpats.push(subpat)
		}
	}
}

// rustc is stupid and doesn't allow this to be a closure since it's part of a recursive
// function
fn is_not_empty(s: &&str) -> bool {
	!s.is_empty()
}

/// Parses a string until it diverges from all of the patterns.
pub fn not_starting_with<'a, I, E>(
	patterns: I
) -> Box<dyn Parser<char, Vec<char>, Error = E> + 'a>
where
	I: IntoIterator<Item = &'a str>,
	E: chumsky::Error<char> + 'a
{
	let mut patterns = patterns.into_iter().peekable();
	if patterns.peek().is_none() {
		// iterator is empty, so we've diverged from all patterns
		Box::new(empty().map(|()| vec![]))
	} else {
		// store all chars that can appear as the first char in a pattern,
		// and the subpatterns that may follow
		let mut first_chars = HashMap::<char, Subpatterns<'a>>::new();
		for pat in patterns {
			let mut chars = pat.chars();
			let first = chars.next().expect("Patterns must not be empty");
			let subpattern = &pat[1 ..];
			if let Some(subpatterns) = first_chars.get_mut(&first) {
				subpatterns.push(subpattern);
			} else {
				first_chars.insert(first, Subpatterns::new(subpattern));
			}
		}

		let mut parser: Box<dyn Parser<char, Vec<char>, Error = E> + 'a> = Box::new(
			none_of(first_chars.keys().copied().collect::<Vec<char>>())
				.map(|ch| vec![ch])
		);
		for (ch, subpats) in first_chars {
			if let Subpatterns::Subpatterns(subpats) = subpats {
				parser = Box::new(choice((
					parser,
					one_of(ch)
						.then(choice((
							not_starting_with(subpats.into_iter().filter(is_not_empty)),
							end().map(|()| vec![])
						)))
						.map(|(ch, mut vec)| {
							vec.insert(0, ch);
							vec
						})
				)));
			}
		}

		parser
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn test_lexer() -> impl Parser<char, String, Error = Simple<char>> {
		not_starting_with(["{%", "{{"]).map(|vec| vec.into_iter().collect())
	}

	#[test]
	fn test_starting_with_other() {
		let parsed = test_lexer().parse("foo{%");
		assert_eq!(parsed, Ok("f".to_owned()));
	}

	#[test]
	fn test_starting_with_first_char() {
		let parsed = test_lexer().parse("{foo");
		assert_eq!(parsed, Ok("{f".to_owned()));
	}

	#[test]
	fn test_starting_with_first_pattern() {
		let parsed = test_lexer().parse("{%foo");
		assert_matches!(parsed, Err(_));
	}
}
