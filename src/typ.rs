use crate::identifier::QualifiedIdent;

// Type = TypeName | TypeLit | "(" Type ")" .
pub enum GoType<'a> {
    TypeName(TypeName<'a>),
    TypeLit(TypeLit),
}

// TypeName  = identifier | QualifiedIdent .
pub enum TypeName<'a> {
    // e.g) int, string ... and user-defined types. because the primitives aren't reserved-keyword.
    Identifier(&'a str),

    //
    QualifiedIdent(QualifiedIdent<'a>),
}

impl<'a> From<&'a str> for TypeName<'a> {
    fn from(v: &'a str) -> Self {
        Self::Identifier(v)
    }
}

impl<'a> From<QualifiedIdent<'a>> for TypeName<'a> {
    fn from(v: QualifiedIdent<'a>) -> Self {
        Self::QualifiedIdent(v)
    }
}
// TypeLit = ArrayType | StructType | PointerType | FunctionType | InterfaceType |
// 	         SliceType | MapType | ChannelType .
pub enum TypeLit {}
