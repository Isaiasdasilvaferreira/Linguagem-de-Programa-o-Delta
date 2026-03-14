#[derive(Debug, Clone)]
pub enum Command {
    Print(String),
    Calc(String),
    ScanNetwork { prefix: String, port: u16 },
    ScanPorts(String),
    EncryptFile { file: String, algo: String },
    DecryptFile { file: String, algo: String },
    HashFile { file: String, algo: String },
    VerifyHash { file: String, algo: String },
    Alert(String),
    Log(String),
    Assign { var: String, value: String },    

    SecureData { name: String },
    FunctionWithRole { name: String, role: String },

    SecureChannel {
        name: String,
        host: String,
        port: u16,
        cert: Option<String>, 
        mutual_tls: bool,      
    },

    SendData {
        channel: String,
        data: String,
    },

    BatchEncrypt { 
        path: String, 
        extension: String, 
        algo: String 
    },

    BatchDecrypt { 
        path: String, 
        extension: String, 
        algo: String 
    },
}