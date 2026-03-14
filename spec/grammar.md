# DELTA: Formal Grammar

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

<Algorithm>     ::= "AES256" | "SHA256" | "RSA" | "ECC"
<ChannelParams> ::= { <StringLiteral> }
<StringLiteral> ::= "\"" <Text> "\""
<Identifier>    ::= [a-zA-Z_][a-zA-Z0-9_]*
<Number>        ::= [0-9]+
