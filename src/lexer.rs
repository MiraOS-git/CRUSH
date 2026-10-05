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
