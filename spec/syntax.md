# DELTA: Guia de Sintaxe e Uso da Linguagem

A linguagem DELTA opera através de comandos diretos projetados para clareza operacional. Todos os argumentos textuais ou caminhos de arquivos devem ser envoltos em aspas duplas (`""`).

## 1. Operações de Arquivo e Criptografia
A DELTA permite criptografar, descriptografar e verificar o hash de arquivos localmente.

* **Criptografar Arquivo Único:**
    `encrypt file "dados.txt" with AES256`
* **Descriptografar Arquivo Único:**
    `decrypt file "dados.txt.enc" with AES256`
* **Gerar Hash de Arquivo:**
    `hash file "config.json" with SHA256`
* **Verificar Integridade (Hash):**
    `verify file "config.json" with SHA256`

## 2. Automação e Processamento em Lote (Batch)
Você pode aplicar criptografia ou descriptografia em múltiplos arquivos de um diretório com base em sua extensão.

* **Criptografia em Lote:**
    `scan files ".txt" in "/caminho/pasta" encrypt AES256`
* **Descriptografia em Lote:**
    `scan files ".txt.enc" in "/caminho/pasta" decrypt AES256`

## 3. Reconhecimento de Rede (Scanner)
Comandos para identificação de hosts e portas abertas.

* **Mapeamento de Rede:**
    `scan network "192.168.1.0/24"`
* **Varredura de Portas:**
    `scan ports "127.0.0.1"`

## 4. Canais Seguros (TLS/mTLS)
Estabelece conexões encriptadas via rede. Os parâmetros de configuração são passados através de strings formatadas.

* **Criar Canal Seguro Simples:**
    `secure channel "meu_canal" "host(192.168.1.10)" "port(8443)"`
* **Criar Canal com mTLS e Certificado:**
    `secure channel "canal_mtls" "host(secure.local)" "port(443)" "cert(client.pem)" "mutual_tls(true)"`

## 5. Gerenciamento de Estado e RBAC
Permite alocação de memória na sessão e validação de privilégios.

* **Declaração de Variável:**
    `let var_nome = "valor"` ou `let iterador = 10`
* **Declaração de Dado Sensível:**
    `secure data "chave_api"`
* **Executar Função com Validação de Papel:**
    `function "deletar_banco" requires role(admin)`

## 6. Utilitários Gerais
* **Exibir Mensagem:** `print "Sistema Iniciado"`
* **Calcular Expressão:** `calc "2 + 2 * 5"`
* **Registrar Alerta:** `alert "Tentativa de invasão detectada!"`
* **Registrar Log Local:** `log "Acesso autorizado."`
