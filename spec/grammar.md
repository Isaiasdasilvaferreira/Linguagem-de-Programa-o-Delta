# DELTA: Gramática Formal

## Regras Sintáticas

**PROGRAM** ::= **COMMAND**

**COMMAND** ::= **SCAN_COMMAND** | **CRYPTO_COMMAND** | **UTIL_COMMAND** | **ASSIGN_COMMAND** | **SECURE_COMMAND**
            | **FUNCTION_COMMAND**

**SCAN_COMMAND** ::= "scan" "network" **STRING**
                | "scan" "ports" **STRING**
                | "scan" "files" **STRING** "in" **STRING** ("encrypt" | "decrypt") **ALGORITHM**

**CRYPTO_COMMAND** ::= ("encrypt" | "decrypt") "file" **STRING** "with" **ALGORITHM**
                  | ("hash" | "verify") "file" **STRING** "with" **ALGORITHM**

**ASSIGN_COMMAND** ::= "let" **IDENTIFIER** "=" (**STRING** | **NUMBER**)

**UTIL_COMMAND** ::= "print" **STRING**
                | "calc" **STRING**
                | "alert" **STRING**
                | "log" **STRING**

**SECURE_COMMAND** ::= "secure" "data" **STRING**
                  | "secure" "channel" **STRING** **PARAMS**

**FUNCTION_COMMAND** ::= "function" **STRING** "requires" "role(" **IDENTIFIER** ")"

---

## Terminais Essenciais

* **ALGORITHM** ::= "AES256" | "SHA256" | "RSA" | "ECC"
* **PARAMS** ::= { **STRING** }  *(Ex: host, port, cert)*
* **STRING** ::= "\"" text "\""
* **IDENTIFIER** ::= [a-zA-Z_][a-zA-Z0-9_]*
* **NUMBER** ::= [0-9]+
