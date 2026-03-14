# DELTA: Arquitetura do Sistema

A arquitetura do DELTA é estruturada como um interpretador REPL (Read-Eval-Print Loop) modular focado em operações de cibersegurança, criptografia e rede. O sistema adota um modelo de execução baseado em estado global seguro e Controle de Acesso Baseado em Papéis (RBAC).

## Componentes Principais

* **Lexer (`lexer.rs`):** Responsável pela análise léxica. Converte a entrada de texto bruta do usuário em um vetor de tokens estruturados (`enum Token`), ignorando espaços em branco e isolando literais de string e identificadores.
* **Parser (`parser.rs`):** Realiza a análise sintática. Consome os tokens gerados pelo Lexer e constrói a Árvore de Sintaxe Abstrata (AST) representada pelo `enum Command`. Trata erros de sintaxe e argumentos ausentes.
* **AST (`ast.rs`):** Define a estrutura de dados central (`Command`) que mapeia todas as intenções de execução do sistema (ex: `EncryptFile`, `ScanNetwork`, `SecureChannel`).
* **Context & State (`context.rs`):** Gerencia o estado global e a sessão do usuário.
    * Utiliza `lazy_static` e `Mutex` para manter um contexto global (`CTX`) thread-safe.
    * Armazena a role atual do usuário, variáveis em memória e conexões TLS ativas (Canais).
* **Interpreter (`interpreter.rs`):** O motor de execução. Recebe os nós da AST, verifica as permissões através do Contexto/Policy, e delega a execução para os módulos específicos (Crypto, Network, Scanner). Registra auditoria para cada comando via `memory::audit`.
* **Policy Engine (`policy.rs` & `roles.json`):** Módulo de RBAC que verifica de forma estrita se a role atual no `Context` tem permissão para executar um nó da AST específico.
* **Submódulos de Domínio:**
    * `crypto.rs`: Implementa criptografia simétrica (AES256) com padding PKCS7, além de hashing (SHA256). Manipula IO de arquivos diretos.
    * `network.rs`: Estabelece e gerencia túneis TLS e mTLS usando `tokio-rustls`, suportando validação de certificados locais.
    * `scanner.rs`: Implementa operações de reconhecimento de rede de alta performance. Utiliza a runtime assíncrona do `tokio` para despachar dezenas de threads simultâneas (`tokio::spawn`), testando sockets TCP com um timeout estrito de 200ms para garantir eficiência na varredura de sub-redes e portas.

## Fluxo de Execução (Pipeline)

1.  **Entrada:** O usuário digita uma string no REPL (`main.rs`).
2.  **Tokenização:** `lexer::tokenize` converte a string em `Vec<Token>`.
3.  **Parsing:** `parser::parse` transforma os tokens em um `Command` (AST).
4.  **Autorização:** `interpreter::interpret` extrai o nome do comando e consulta `ctx.can_execute()`. Se negado, emite log e aborta.
5.  **Avaliação:** O comando é roteado para a função de domínio correspondente.
6.  **Auditoria:** O resultado (sucesso ou falha) é registrado em log.
