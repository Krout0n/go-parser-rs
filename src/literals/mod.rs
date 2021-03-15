use self::integer::IntLit;

pub mod integer;
pub mod letter_and_digit;

///
/// Literal     = BasicLit | CompositeLit | FunctionLit .
/// OperandName = identifier | QualifiedIdent .
pub enum Literal<'a> {
    /// BasicLit    = int_lit | float_lit | imaginary_lit | rune_lit | string_lit .
    IntLit(IntLit<'a>),
    FloatLit,
    ImaginaryLit,
    RuneLit,
    StringLit(&'a str),
    // TODO: Composite, FunctionLit...
}

impl<'a> Literal<'a> {
    pub fn int_lit(i: IntLit<'a>) -> Self {
        Self::IntLit(i)
    }
}
