#[derive(Debug, PartialEq)]
pub enum Keywords {
    AlignAs,
    AlignOf,
    And,
    AndEq,
    ASM,
    AtomicCancel,
    AtomicCommit,
    AtomicNoexcept,
    Auto,
    BitAnd,
    BitOr,
    Bool,
    Break,
    Case,
    Catch,
    Char,
    Char8t,
    Char16t,
    Char32t,
    Class,
    Compl,
    Concept,
    Const,
    ConstEval,
    ConstExpr,
    ConstInit,
    ConstCast,
    Continue,
    CoAwait,
    CoReturn,
    CoYield,
    DeclType,
    Default,
    Delete,
    Do,
    Double,
    DynamicCast,
    Else,
    Enum,
    Explicit,
    Export,
    Extern,
    False,
    Float,
    For,
    Friend,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Mutable,
    Namespace,
    New,
    NoExcept,
    Not,
    NotEq,
    NullPtr,
    Operator,
    Or,
    OrEq,
    Private,
    Protected,
    Public,
    Reflexpr,
    Register,
    ReinterpretCast,
    Requires,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    StaticAssert,
    StaticCast,
    Struct,
    Switch,
    Synchronized,
    Template,
    This,
    ThreadLocal,
    Throw,
    True,
    Try,
    TypeDef,
    TypeId,
    TypeName,
    Union,
    Unsigned,
    Using,
    Virtual,
    Void,
    Volatile,
    WCharT,
    While,
    Xor,
    XorEq
}

#[derive(Debug, PartialEq)]
pub enum SpecialCharacters {
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
    Plus,
    Increment,
    Minus,
    Decrement,
    Divide,
    Equals,
    Greater,
    GreaterOrEq,
    Less,
    LessOrEq,
    Assignment,
    Semicolon,
    Negation,
    DoubleNegation,
    ArrowFn
}

#[derive(Debug, PartialEq)]
pub enum TokenTypes {
    Keywords(Keywords),
    SpecialCharacters(SpecialCharacters),
    Unknown
}

pub fn classify_token(token: &str) -> TokenTypes {
    match token {
        "alignas" => {
            TokenTypes::Keywords(Keywords::AlignAs)
        }
        "alignof" => {
            TokenTypes::Keywords(Keywords::AlignOf)
        }
        "and" => {
            TokenTypes::Keywords(Keywords::And)
        }
        "and_eq" => {
            TokenTypes::Keywords(Keywords::AndEq)
        }
        "asm" => {
            TokenTypes::Keywords(Keywords::ASM)
        }
        "atomic_cancel" => {
            TokenTypes::Keywords(Keywords::AtomicCancel)
        }
        "atomic_commit" => {
            TokenTypes::Keywords(Keywords::AtomicCommit)
        }
        "atomic_noexcept" => {
            TokenTypes::Keywords(Keywords::AtomicNoexcept)
        }
        "auto" => {
            TokenTypes::Keywords(Keywords::Auto)
        }
        "bitand" => {
            TokenTypes::Keywords(Keywords::BitAnd)
        }
        "bitor" => {
            TokenTypes::Keywords(Keywords::BitOr)
        }
        "bool" => {
            TokenTypes::Keywords(Keywords::Bool)
        }
        "break" => {
            TokenTypes::Keywords(Keywords::Break)
        }
        "case" => {
            TokenTypes::Keywords(Keywords::Case)
        }
        "catch" => {
            TokenTypes::Keywords(Keywords::Catch)
        }
        "char" => {
            TokenTypes::Keywords(Keywords::Char)
        }
        "char8_t" => {
            TokenTypes::Keywords(Keywords::Char8t)
        }
        "char16_t" => {
            TokenTypes::Keywords(Keywords::Char16t)
        }
        "char32_t" => {
            TokenTypes::Keywords(Keywords::Char32t)
        }
        "class" => {
            TokenTypes::Keywords(Keywords::Class)
        }
        "compl" => {
            TokenTypes::Keywords(Keywords::Compl)
        }
        "concept" => {
            TokenTypes::Keywords(Keywords::Concept)
        }
        "const" => {
            TokenTypes::Keywords(Keywords::Const)
        }
        "consteval" => {
            TokenTypes::Keywords(Keywords::ConstEval)
        }
        "constexpr" => {
            TokenTypes::Keywords(Keywords::ConstExpr)
        }
        "constinit" => {
            TokenTypes::Keywords(Keywords::ConstInit)
        }
        "const_cast" => {
            TokenTypes::Keywords(Keywords::ConstCast)
        }
        "continue" => {
            TokenTypes::Keywords(Keywords::Continue)
        }
        "co_return" => {
            TokenTypes::Keywords(Keywords::CoReturn)
        }
        "co_await" => {
            TokenTypes::Keywords(Keywords::CoAwait)
        }
        "co_yield" => {
            TokenTypes::Keywords(Keywords::CoYield)
        }
        "decltype" => {
            TokenTypes::Keywords(Keywords::DeclType)
        }
        "default" => {
            TokenTypes::Keywords(Keywords::Default)
        }
        "delete" => {
            TokenTypes::Keywords(Keywords::Delete)
        }
        "do" => {
            TokenTypes::Keywords(Keywords::Do)
        }
        "double" => {
            TokenTypes::Keywords(Keywords::Double)
        }
        "dynamic_cast" => {
            TokenTypes::Keywords(Keywords::DynamicCast)
        }
        "else" => {
            TokenTypes::Keywords(Keywords::Else)
        }
        "enum" => {
            TokenTypes::Keywords(Keywords::Enum)
        }
        "explicit" => {
            TokenTypes::Keywords(Keywords::Explicit)
        }
        "export" => {
            TokenTypes::Keywords(Keywords::Export)
        }
        "extern" => {
            TokenTypes::Keywords(Keywords::Extern)
        }
        "false" => {
            TokenTypes::Keywords(Keywords::False)
        }
        "float" => {
            TokenTypes::Keywords(Keywords::Float)
        }
        "for" => {
            TokenTypes::Keywords(Keywords::For)
        }
        "friend" => {
            TokenTypes::Keywords(Keywords::Friend)
        }
        "goto" => {
            TokenTypes::Keywords(Keywords::Goto)
        }
        "if" => {
            TokenTypes::Keywords(Keywords::If)
        }
        "inline" => {
            TokenTypes::Keywords(Keywords::Inline)
        }
        "int" => {
            TokenTypes::Keywords(Keywords::Int)
        }
        "long" => {
            TokenTypes::Keywords(Keywords::Long)
        }
        "mutable" => {
            TokenTypes::Keywords(Keywords::Mutable)
        }
        "namespace" => {
            TokenTypes::Keywords(Keywords::Namespace)
        }
        "new" => {
            TokenTypes::Keywords(Keywords::New)
        }
        "noexcept" => {
            TokenTypes::Keywords(Keywords::NoExcept)
        }
        "not" => {
            TokenTypes::Keywords(Keywords::Not)
        }
        "not_eq" => {
            TokenTypes::Keywords(Keywords::NotEq)
        }
        "nullptr" => {
            TokenTypes::Keywords(Keywords::NullPtr)
        }
        "operator" => {
            TokenTypes::Keywords(Keywords::Operator)
        }
        "or" => {
            TokenTypes::Keywords(Keywords::Or)
        }
        "or_eq" => {
            TokenTypes::Keywords(Keywords::OrEq)
        }
        "private" => {
            TokenTypes::Keywords(Keywords::Private)
        }
        "protected" => {
            TokenTypes::Keywords(Keywords::Protected)
        }
        "public" => {
            TokenTypes::Keywords(Keywords::Public)
        }
        "reflexpr" => {
            TokenTypes::Keywords(Keywords::Reflexpr)
        }
        "register" => {
            TokenTypes::Keywords(Keywords::Register)
        }
        "reinterpret_cast" => {
            TokenTypes::Keywords(Keywords::ReinterpretCast)
        }
        "requires" => {
            TokenTypes::Keywords(Keywords::Requires)
        }
        "return" => {
            TokenTypes::Keywords(Keywords::Return)
        }
        "short" => {
            TokenTypes::Keywords(Keywords::Short)
        }
        "signed" => {
            TokenTypes::Keywords(Keywords::Signed)
        }
        "sizeof" => {
            TokenTypes::Keywords(Keywords::Sizeof)
        }
        "static" => {
            TokenTypes::Keywords(Keywords::Static)
        }
        "static_assert" => {
            TokenTypes::Keywords(Keywords::StaticAssert)
        }
        "static_cast" => {
            TokenTypes::Keywords(Keywords::StaticCast)
        }
        "struct" => {
            TokenTypes::Keywords(Keywords::Struct)
        }
        "switch" => {
            TokenTypes::Keywords(Keywords::Switch)
        }
        "synchronized" => {
            TokenTypes::Keywords(Keywords::Synchronized)
        }
        "template" => {
            TokenTypes::Keywords(Keywords::Template)
        }
        "this" => {
            TokenTypes::Keywords(Keywords::This)
        }
        "thread_local" => {
            TokenTypes::Keywords(Keywords::ThreadLocal)
        }
        "throw" => {
            TokenTypes::Keywords(Keywords::Throw)
        }
        "true" => {
            TokenTypes::Keywords(Keywords::True)
        }
        "try" => {
            TokenTypes::Keywords(Keywords::Try)
        }
        "typedef" => {
            TokenTypes::Keywords(Keywords::TypeDef)
        }
        "typeid" => {
            TokenTypes::Keywords(Keywords::TypeId)
        }
        "typename" => {
            TokenTypes::Keywords(Keywords::TypeName)
        }
        "union" => {
            TokenTypes::Keywords(Keywords::Union)
        }
        "unsigned" => {
            TokenTypes::Keywords(Keywords::Unsigned)
        }
        "using" => {
            TokenTypes::Keywords(Keywords::Using)
        }
        "virtual" => {
            TokenTypes::Keywords(Keywords::Virtual)
        }
        "void" => {
            TokenTypes::Keywords(Keywords::Void)
        }
        "volatile" => {
            TokenTypes::Keywords(Keywords::Volatile)
        }
        "wchar_t" => {
            TokenTypes::Keywords(Keywords::WCharT)
        }
        "while" => {
            TokenTypes::Keywords(Keywords::While)
        }
        "xor" => {
            TokenTypes::Keywords(Keywords::Xor)
        }
        "xor_eq" => {
            TokenTypes::Keywords(Keywords::XorEq)
        }

        "(" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::OpenParen)
        }
        ")" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::CloseParen)
        }
        "[" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::OpenSquare)
        }
        "]" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::CloseSquare)
        }
        "{" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::OpenCurly)
        }
        "}" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::CloseCurly)
        }
        "+" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::Plus)
        }
        "++" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::Increment)
        }
        "-" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::Minus)
        }
        "--" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::Decrement)
        }
        "/" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::Divide)
        }
        "==" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::Equals)
        }
        ">" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::Greater)
        }
        "=>" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::ArrowFn)
        }
        ">=" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::GreaterOrEq)
        }
        "<" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::Less)
        }
        "<=" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::LessOrEq)
        }
        "=" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::Assignment)
        }
        ";" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::Semicolon)
        }

        "!!" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::DoubleNegation)
        }
        "!" => {
            TokenTypes::SpecialCharacters(SpecialCharacters::Negation)
        }

        _ => {
            TokenTypes::Unknown
        }
    }
}
