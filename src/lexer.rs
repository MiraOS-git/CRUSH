#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
	Word(Vec<u8>),
	IoNumber(u32),
	Newline,
	Op(Operator),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
	Pipe,      // |
	AndIf,     // &&
	OrIf,      // ||
	Semi,      // ;
	DSemi,     // ;;
	SemiAnd,   // ;&
	Amp,       // &
	Less,      // <
	Great,     // >
	DLess,     // <<
	DGreat,    // >>
	LessAnd,   // <&
	GreatAnd,  // >&
	LessGreat, // <>
	DLessDash, // <<-
	Clobber,   // >|
	LParen,    // (
	RParen,    // )
}

pub struct Lexer<'a> {
	input: &'a [u8],
	pos: usize,
}

impl<'a> Lexer<'a> {
	pub fn new(input: &'a [u8]) -> Self {
		Lexer { input, pos: 0 }
	}
	fn peek(&self) -> Option<u8> {
		self.input.get(self.pos).copied()
	}
	fn peek_at(&self, offset: usize) -> Option<u8> {
		self.input.get(self.pos + offset).copied()
	}
	fn advance(&mut self) -> Option<u8> {
		let byte = self.peek()?;
		self.pos += 1;
		Some(byte)
	}
	pub fn next_token(&mut self) -> Option<Token> {
		while let Some(b' ' | b'\t') = self.peek() {
			self.advance();
		}
		let byte = self.peek()?;
		match byte {
			b'\n' => {
				self.advance();
				Some(Token::Newline)
			}
			b'|' => {
				self.advance();
				if self.peek() == Some(b'|') {
					self.advance();
					Some(Token::Op(Operator::OrIf))
				} else {
					Some(Token::Op(Operator::Pipe))
				}
			}
			b'&' => {
				self.advance();
				if self.peek() == Some(b'&') {
					self.advance();
					Some(Token::Op(Operator::AndIf))
				} else {
					Some(Token::Op(Operator::Amp))
				}
			}
			b';' => {
				self.advance();
				if self.peek() == Some(b';') {
					self.advance();
					Some(Token::Op(Operator::DSemi))
				} else if self.peek() == Some(b'&') {
					self.advance();
					Some(Token::Op(Operator::SemiAnd))
				} else {
					Some(Token::Op(Operator::Semi))
				}
			}
			b'<' => {
				self.advance();
				if self.peek() == Some(b'<') {
					self.advance();
					if self.peek() == Some(b'-') {
						self.advance();
						Some(Token::Op(Operator::DLessDash))
					} else {
						Some(Token::Op(Operator::DLess))
					}
				} else if self.peek() == Some(b'&') {
					self.advance();
					Some(Token::Op(Operator::LessAnd))
				} else if self.peek() == Some(b'>') {
					self.advance();
					Some(Token::Op(Operator::LessGreat))
				} else {
					Some(Token::Op(Operator::Less))
				}
			}
			b'>' => {
				self.advance();
				if self.peek() == Some(b'>') {
					self.advance();
					Some(Token::Op(Operator::DGreat))
				} else if self.peek() == Some(b'&') {
					self.advance();
					Some(Token::Op(Operator::GreatAnd))
				} else if self.peek() == Some(b'|') {
					self.advance();
					Some(Token::Op(Operator::Clobber))
				} else {
					Some(Token::Op(Operator::Great))
				}
			}
			b'(' => {
				self.advance();
				Some(Token::Op(Operator::LParen))
			}
			b')' => {
				self.advance();
				Some(Token::Op(Operator::RParen))
			}
			_ => todo!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn lex(input: &[u8]) -> Vec<Token> {
		let mut lx = Lexer::new(input);
		let mut out = Vec::new();
		while let Some(tok) = lx.next_token() {
			out.push(tok);
		}
		out
	}

	fn op(o: Operator) -> Token {
		Token::Op(o)
	}

	#[test]
	fn peek_and_advance() {
		let mut lx = Lexer::new(b"ab");
		assert_eq!(lx.peek(), Some(b'a'));
		assert_eq!(lx.peek_at(1), Some(b'b'));
		assert_eq!(lx.advance(), Some(b'a'));
		assert_eq!(lx.advance(), Some(b'b'));
		assert_eq!(lx.advance(), None);
		assert_eq!(lx.peek(), None);
	}

	#[test]
	fn pipe() {
		assert_eq!(lex(b"|"), vec![op(Operator::Pipe)]);
	}

	#[test]
	fn or_if() {
		assert_eq!(lex(b"||"), vec![op(Operator::OrIf)]);
	}

	#[test]
	fn longest_match() {
		assert_eq!(lex(b"|||"), vec![op(Operator::OrIf), op(Operator::Pipe)]);
	}

	#[test]
	fn newline() {
		assert_eq!(lex(b"\n"), vec![Token::Newline]);
	}

	#[test]
	fn blanks_only() {
		assert_eq!(lex(b"  \t "), vec![]);
	}

	#[test]
	fn blanks_between_tokens() {
		assert_eq!(lex(b"| \t |"), vec![op(Operator::Pipe), op(Operator::Pipe)]);
	}

	#[test]
	fn amp() {
		assert_eq!(lex(b"&"), vec![op(Operator::Amp)]);
	}

	#[test]
	fn and_if() {
		assert_eq!(lex(b"&&"), vec![op(Operator::AndIf)]);
	}

	#[test]
	fn and_if_longest_match() {
		assert_eq!(lex(b"&&&"), vec![op(Operator::AndIf), op(Operator::Amp)]);
	}

	#[test]
	fn semi() {
		assert_eq!(lex(b";"), vec![op(Operator::Semi)]);
	}

	#[test]
	fn dsemi() {
		assert_eq!(lex(b";;"), vec![op(Operator::DSemi)]);
	}

	#[test]
	fn semi_and() {
		assert_eq!(lex(b";&"), vec![op(Operator::SemiAnd)]);
	}

	#[test]
	fn dsemi_longest_match() {
		assert_eq!(lex(b";;;"), vec![op(Operator::DSemi), op(Operator::Semi)]);
	}

	#[test]
	fn semis_split_by_blank() {
		assert_eq!(lex(b"; ;"), vec![op(Operator::Semi), op(Operator::Semi)]);
	}

	#[test]
	fn less() {
		assert_eq!(lex(b"<"), vec![op(Operator::Less)]);
	}

	#[test]
	fn dless() {
		assert_eq!(lex(b"<<"), vec![op(Operator::DLess)]);
	}

	#[test]
	fn dless_dash() {
		assert_eq!(lex(b"<<-"), vec![op(Operator::DLessDash)]);
	}

	#[test]
	fn less_and() {
		assert_eq!(lex(b"<&"), vec![op(Operator::LessAnd)]);
	}

	#[test]
	fn less_great() {
		assert_eq!(lex(b"<>"), vec![op(Operator::LessGreat)]);
	}

	#[test]
	fn dless_longest_match() {
		assert_eq!(lex(b"<<<"), vec![op(Operator::DLess), op(Operator::Less)]);
	}

	#[test]
	fn lesses_split_by_blank() {
		assert_eq!(lex(b"< <"), vec![op(Operator::Less), op(Operator::Less)]);
	}

	#[test]
	fn great() {
		assert_eq!(lex(b">"), vec![op(Operator::Great)]);
	}

	#[test]
	fn dgreat() {
		assert_eq!(lex(b">>"), vec![op(Operator::DGreat)]);
	}

	#[test]
	fn great_and() {
		assert_eq!(lex(b">&"), vec![op(Operator::GreatAnd)]);
	}

	#[test]
	fn clobber() {
		assert_eq!(lex(b">|"), vec![op(Operator::Clobber)]);
	}

	#[test]
	fn dgreat_longest_match() {
		assert_eq!(lex(b">>>"), vec![op(Operator::DGreat), op(Operator::Great)]);
	}

	#[test]
	fn greats_split_by_blank() {
		assert_eq!(lex(b"> >"), vec![op(Operator::Great), op(Operator::Great)]);
	}

	#[test]
	fn parens_with_blank() {
		assert_eq!(
			lex(b"( )"),
			vec![op(Operator::LParen), op(Operator::RParen)]
		);
	}

	#[test]
	fn parens_adjacent() {
		assert_eq!(lex(b"()"), vec![op(Operator::LParen), op(Operator::RParen)]);
	}

	#[test]
	fn mixed_operators() {
		assert_eq!(
			lex(b"|&;"),
			vec![op(Operator::Pipe), op(Operator::Amp), op(Operator::Semi)]
		);
	}

	#[test]
	fn operators_across_newline() {
		assert_eq!(
			lex(b"&&\n||"),
			vec![op(Operator::AndIf), Token::Newline, op(Operator::OrIf)]
		);
	}
}
