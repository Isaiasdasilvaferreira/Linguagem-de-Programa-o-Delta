# DELTA: Formal Grammar

A gramática do DELTA é linear e baseada em comandos iniciados por palavras-chave específicas. Abaixo está a representação EBNF deduzida do `parser.rs`.

## Regras Sintáticas

<Program> ::= <Command>

<Command> ::= <ScanCommand> 
            | <CryptoCommand> 
            | <UtilCommand> 
            | <AssignCommand> 
            | <SecureCommand>
            | <FunctionCommand>

<ScanCommand> ::= "scan" "network" <StringLiteral>
                | "scan" "ports" <StringLiteral>
                | "scan" "files" <StringLiteral> "in" <StringLiteral> ("encrypt" | "decrypt") <Algorithm>

<CryptoCommand> ::= ("encrypt" | "decrypt") "file" <StringLiteral> "with" <Algorithm>
                  | ("hash" | "verify") "file" <StringLiteral> "with" <Algorithm>

<AssignCommand> ::= "let" <Identifier> "=" (<StringLiteral> | <Number>)

<UtilCommand> ::= "print" <StringLiteral>
                | "calc" <StringLiteral>
                | "alert" <StringLiteral>
                | "log" <StringLiteral>

<SecureCommand> ::= "secure" "data" <StringLiteral>
                  | "secure" "channel" <StringLiteral> <ChannelParams>

<FunctionCommand> ::= "function" <StringLiteral> "requires" "role(" <Identifier> ")"

## Terminais Essenciais

<Algorithm> ::= "AES256" | "SHA256" | "RSA" | "ECC"
<ChannelParams> ::= { <StringLiteral> }  /* Aceita literais no formato "host(...)", "port(...)", "cert(...)", "mutual_tls(...)" */
<StringLiteral> ::= "\"" <Text> "\""
<Identifier> ::= [a-zA-Z_][a-zA-Z0-9_]*
<Number> ::= [0-9]+