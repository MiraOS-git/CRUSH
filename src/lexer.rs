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
}
