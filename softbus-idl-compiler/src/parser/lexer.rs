//! 词法分析器

use logos::Logos;

/// IDL token定义
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"//[^\n]*")]
pub enum Token {
    #[token("service")]
    Service,

    #[token("interface")]
    Interface,

    #[token("struct")]
    Struct,

    #[token("enum")]
    Enum,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token(";")]
    Semicolon,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice().to_string())]
    String(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    Number(i64),
}

/// 词法分析器
pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    /// 创建新的词法分析器
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: Token::lexer(input),
        }
    }

    /// 获取下一个token
    pub fn next_token(&mut self) -> Option<Token> {
        self.inner.next().and_then(|r| r.ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "service MyService { }";
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token(), Some(Token::Service));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("MyService".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::LBrace));
        assert_eq!(lexer.next_token(), Some(Token::RBrace));
    }
}
