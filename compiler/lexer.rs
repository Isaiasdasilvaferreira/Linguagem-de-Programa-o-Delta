#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Scan,
    Network,
    Encrypt,
    Decrypt,             
    File,
    Verify,
    Hash,
    Alert,
    Log,                 
    With,
    Algo(String),
    StringLiteral(String),
    Identifier(String),

    Print,
    Calc,
    Let,
    Number(i32),

    Secure,
    Data,
    Function,
    Requires,
    Role(String),
    Channel,

    Unknown(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        if c == '"' {
            chars.next();
            let mut value = String::new();
            while let Some(&ch) = chars.peek() {
                chars.next();
                if ch == '"' {
                    break;
                }
                value.push(ch);
            }
            tokens.push(Token::StringLiteral(value));
            continue;
        }

        let mut word = String::new();
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() {
                break;
            }
            word.push(ch);
            chars.next();
        }

        let token = match word.as_str() {
            "scan" => Token::Scan,
            "network" => Token::Network,
            "encrypt" => Token::Encrypt,
            "decrypt" => Token::Decrypt,  
            "file" => Token::File,
            "verify" => Token::Verify,
            "hash" => Token::Hash,
            "alert" => Token::Alert,
            "log" => Token::Log,          
            "with" => Token::With,
            "secure" => Token::Secure,
            "data" => Token::Data,
            "function" => Token::Function,
            "requires" => Token::Requires,
            "channel" => Token::Channel,
            "AES256" | "SHA256" | "RSA" | "ECC" => Token::Algo(word.clone()),

            "print" => Token::Print,
            "calc" => Token::Calc,
            "let" => Token::Let,
            _ if word.parse::<i32>().is_ok() => Token::Number(word.parse().unwrap()),

            _ if word.starts_with("role(") && word.ends_with(")") => {
                let role = word.trim_start_matches("role(").trim_end_matches(")").to_string();
                Token::Role(role)
            }

            _ => Token::Unknown(word.clone()),
        };
        tokens.push(token);
    }

    tokens
}