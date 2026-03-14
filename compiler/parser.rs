use crate::lexer::Token;
use crate::ast::Command;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    MissingArgument(&'static str),
    InvalidSyntax(&'static str),
}

pub fn parse(tokens: Vec<Token>) -> Result<Command, ParseError> {
    let mut iter = tokens.into_iter();

    match iter.next() {
        Some(Token::Scan) => parse_scan(&mut iter),
        Some(Token::Print) => parse_print(&mut iter),
        Some(Token::Calc) => parse_calc(&mut iter),
        Some(Token::Encrypt) => parse_encrypt(&mut iter),
        Some(Token::Decrypt) => parse_decrypt(&mut iter),
        Some(Token::Verify) => parse_verify(&mut iter),
        Some(Token::Hash) => parse_hash(&mut iter),
        Some(Token::Alert) => parse_alert(&mut iter),
        Some(Token::Log) => parse_log(&mut iter),
        Some(Token::Let) => parse_assign(&mut iter),
        Some(Token::Secure) => parse_secure(&mut iter),
        Some(Token::Function) => parse_function_role(&mut iter),
        Some(tok) => Err(ParseError::UnexpectedToken(tok)),
        None => Err(ParseError::InvalidSyntax("Comando vazio")),
    }
}

// Função de Scan
fn parse_scan(iter: &mut impl Iterator<Item = Token>) -> Result<Command, ParseError> {
    match iter.next() {
        Some(Token::Network) => {
            if let Some(Token::StringLiteral(prefix)) = iter.next() {
                Ok(Command::ScanNetwork { prefix, port: 0 })
            } else {
                Err(ParseError::MissingArgument("esperado IP da rede entre aspas"))
            }
        },
        Some(Token::Unknown(ref s)) if s == "files" => {
            let extension = match iter.next() {
                Some(Token::StringLiteral(ext)) => ext,
                _ => return Err(ParseError::MissingArgument("esperada extensão entre aspas (ex: '.txt')")),
            };

            match iter.next() {
                Some(Token::Unknown(ref in_tok)) if in_tok == "in" => (),
                _ => return Err(ParseError::InvalidSyntax("esperado 'in' após a extensão")),
            }

            let path = match iter.next() {
                Some(Token::StringLiteral(p)) => p,
                _ => return Err(ParseError::MissingArgument("esperado caminho do diretório entre aspas")),
            };

            match iter.next() {
                Some(Token::Encrypt) => {
                    let algo = match iter.next() {
                        Some(Token::Algo(a)) => a,
                        _ => return Err(ParseError::MissingArgument("esperado algoritmo de criptografia")),
                    };
                    Ok(Command::BatchEncrypt { path, extension, algo })
                },
                Some(Token::Decrypt) => {
                    let algo = match iter.next() {
                        Some(Token::Algo(a)) => a,
                        _ => return Err(ParseError::MissingArgument("esperado algoritmo de descriptografia")),
                    };
                    Ok(Command::BatchDecrypt { path, extension, algo })
                },
                _ => Err(ParseError::InvalidSyntax("esperado 'encrypt' ou 'decrypt' após o caminho")),
            }
        },
        Some(Token::Unknown(s)) if s == "ports" => {
            if let Some(Token::StringLiteral(host)) = iter.next() {
                Ok(Command::ScanPorts(host))
            } else {
                Err(ParseError::MissingArgument("esperado host entre aspas"))
            }
        },
        _ => Err(ParseError::InvalidSyntax("esperado 'network', 'ports' ou 'files' após 'scan'")),
    }
}

// Funções de Criptografia
fn parse_encrypt(iter: &mut impl Iterator<Item = Token>) -> Result<Command, ParseError> {
    match iter.next() { Some(Token::File) => (), _ => return Err(ParseError::InvalidSyntax("esperado 'file' após 'encrypt'")) }
    let file = match iter.next() { Some(Token::StringLiteral(f)) => f, _ => return Err(ParseError::MissingArgument("esperado arquivo")) };
    match iter.next() { Some(Token::With) => (), _ => return Err(ParseError::InvalidSyntax("esperado 'with'")) }
    let algo = match iter.next() { Some(Token::Algo(a)) => a, _ => return Err(ParseError::MissingArgument("esperado algoritmo")) };
    Ok(Command::EncryptFile { file, algo })
}

fn parse_decrypt(iter: &mut impl Iterator<Item = Token>) -> Result<Command, ParseError> {
    match iter.next() { Some(Token::File) => (), _ => return Err(ParseError::InvalidSyntax("esperado 'file' após 'decrypt'")) }
    let file = match iter.next() { Some(Token::StringLiteral(f)) => f, _ => return Err(ParseError::MissingArgument("esperado arquivo")) };
    match iter.next() { Some(Token::With) => (), _ => return Err(ParseError::InvalidSyntax("esperado 'with'")) }
    let algo = match iter.next() { Some(Token::Algo(a)) => a, _ => return Err(ParseError::MissingArgument("esperado algoritmo")) };
    Ok(Command::DecryptFile { file, algo })
}

// Função de Atribuição
fn parse_assign(iter: &mut impl Iterator<Item = Token>) -> Result<Command, ParseError> {
    let var = match iter.next() {
        Some(Token::Identifier(v)) => v,
        _ => return Err(ParseError::MissingArgument("esperado nome da variável")),
    };

    match iter.next() {
        Some(Token::Unknown(ref eq)) if eq == "=" => (),
        Some(tok) => return Err(ParseError::UnexpectedToken(tok)),
        None => return Err(ParseError::InvalidSyntax("esperado '=' após variável")),
    }

    let value = match iter.next() {
        Some(Token::StringLiteral(s)) => s,
        Some(Token::Number(n)) => n.to_string(),
        Some(tok) => return Err(ParseError::UnexpectedToken(tok)),
        None => return Err(ParseError::MissingArgument("esperado valor")),
    };

    Ok(Command::Assign { var, value })
}

// Funções Auxiliares
fn parse_verify(iter: &mut impl Iterator<Item = Token>) -> Result<Command, ParseError> {
    let _ = iter.next();
    let file = match iter.next() { Some(Token::StringLiteral(f)) => f, _ => return Err(ParseError::MissingArgument("arquivo")) };
    let _ = iter.next();
    let algo = match iter.next() { Some(Token::Algo(a)) => a, _ => return Err(ParseError::MissingArgument("algoritmo")) };
    Ok(Command::VerifyHash { file, algo })
}

fn parse_hash(iter: &mut impl Iterator<Item = Token>) -> Result<Command, ParseError> {
    let _ = iter.next();
    let file = match iter.next() { Some(Token::StringLiteral(f)) => f, _ => return Err(ParseError::MissingArgument("arquivo")) };
    let _ = iter.next();
    let algo = match iter.next() { Some(Token::Algo(a)) => a, _ => return Err(ParseError::MissingArgument("algoritmo")) };
    Ok(Command::HashFile { file, algo })
}

fn parse_print(iter: &mut impl Iterator<Item = Token>) -> Result<Command, ParseError> {
    match iter.next() { Some(Token::StringLiteral(msg)) => Ok(Command::Print(msg)), _ => Err(ParseError::MissingArgument("texto")) }
}

fn parse_calc(iter: &mut impl Iterator<Item = Token>) -> Result<Command, ParseError> {
    match iter.next() { Some(Token::StringLiteral(expr)) => Ok(Command::Calc(expr)), _ => Err(ParseError::MissingArgument("expressão")) }
}

fn parse_alert(iter: &mut impl Iterator<Item = Token>) -> Result<Command, ParseError> {
    match iter.next() { Some(Token::StringLiteral(msg)) => Ok(Command::Alert(msg)), _ => Err(ParseError::MissingArgument("texto")) }
}

fn parse_log(iter: &mut impl Iterator<Item = Token>) -> Result<Command, ParseError> {
    match iter.next() { Some(Token::StringLiteral(msg)) => Ok(Command::Log(msg)), _ => Err(ParseError::MissingArgument("texto")) }
}

fn parse_secure(iter: &mut impl Iterator<Item = Token>) -> Result<Command, ParseError> {
    match iter.next() {
        Some(Token::Data) => {
            if let Some(Token::StringLiteral(name)) = iter.next() { Ok(Command::SecureData { name }) }
            else { Err(ParseError::MissingArgument("nome do dado")) }
        },
        Some(Token::Channel) => parse_secure_channel(iter),
        _ => Err(ParseError::InvalidSyntax("comando secure incompleto")),
    }
}

fn parse_secure_channel(iter: &mut impl Iterator<Item = Token>) -> Result<Command, ParseError> {
    let name = match iter.next() { Some(Token::StringLiteral(n)) => n, _ => return Err(ParseError::MissingArgument("canal")) };
    let (mut host, mut port, mut cert, mut mutual_tls) = ("example.com".to_string(), 443, None, false);
    for token in iter {
        if let Token::StringLiteral(s) = token {
            if s.starts_with("host(") { host = s[5..s.len()-1].to_string(); }
            else if s.starts_with("port(") { port = s[5..s.len()-1].parse().unwrap_or(443); }
            else if s.starts_with("cert(") { cert = Some(s[5..s.len()-1].to_string()); }
            else if s.starts_with("mutual_tls(") { mutual_tls = s.contains("true"); }
        }
    }
    Ok(Command::SecureChannel { name, host, port, cert, mutual_tls })
}

fn parse_function_role(iter: &mut impl Iterator<Item = Token>) -> Result<Command, ParseError> {
    let name = match iter.next() { Some(Token::StringLiteral(n)) => n, _ => return Err(ParseError::InvalidSyntax("função")) };
    match (iter.next(), iter.next()) {
        (Some(Token::Requires), Some(Token::Role(role))) => Ok(Command::FunctionWithRole { name, role }),
        _ => Err(ParseError::MissingArgument("requires role(...)")),
    }
}