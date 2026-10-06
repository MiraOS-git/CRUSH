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

	fn lex(input: &[u8]) -> Vec<Token> {
		let mut lx = Lexer::new(input);
		let mut out = Vec::new();
		while let Some(tok) = lx.next_token() {
			out.push(tok);
		}
		out
	}

	#[test]
	fn pipe() {
		assert_eq!(lex(b"|"), vec![Token::Op(Operator::Pipe)]);
	}

	#[test]
	fn or_if() {
		assert_eq!(lex(b"||"), vec![Token::Op(Operator::OrIf)]);
	}

	#[test]
	fn longest_match() {
		assert_eq!(
			lex(b"|||"),
			vec![Token::Op(Operator::OrIf), Token::Op(Operator::Pipe)]
		);
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
		assert_eq!(
			lex(b"| \t |"),
			vec![Token::Op(Operator::Pipe), Token::Op(Operator::Pipe)]
		);
	}
}
