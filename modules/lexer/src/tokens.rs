#[derive(Debug, Clone, PartialEq)]
pub enum NesiaDataType {
    NumberDataType,
    StringDataType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Basic tokens
    Identifier(String), // e.g identifiers, my_variable
    VariableIdentifier,
    Assign,             // assignment operator =
    Equals,             // equality operator ==
    NotOperator,        // not operator !
    NotEquals,          // inequality operator !=
    GreaterThan,        // greater than operator >
    LessThan,           // less than operator <
    GreaterThanOrEqual, // greater than or equal operator >=
    LessThanOrEqual,    // less than or equal operator <=
    Colon,              // colon :
    Print,              // print statement
    OpenParen,          // open parenthesis (
    CloseParen,         // close parenthesis )
    OpenBrace,          // open brace {
    CloseBrace,         // close brace }
    OpenBracket,        // open bracket [
    CloseBracket,       // close bracket ]

    // Data Types
    DataType(NesiaDataType),
    Number(i64),
    String(String),

    //
    Function,
    Return,
}
