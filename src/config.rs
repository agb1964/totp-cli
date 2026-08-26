use clap::{Parser, Subcommand};

/// Количество цифр в коде.
pub const DIGITS: usize = 6;
/// Шаг времени в секундах.
pub const STEP: u64 = 30;
/// Допустимый дрейф в интервалах (±1).
pub const SKEW: u16 = 1;

#[derive(Parser, Debug)]
#[command(name = "totp-cli", version, about = "Генерация и проверка TOTP-кодов")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Сгенерировать новый секрет и QR-код
    Generate {
        /// Название сервиса (Issuer)
        #[arg(long, default_value = "totp-cli")]
        issuer: String,
        /// Имя аккаунта/пользователя
        #[arg(long, default_value = "user")]
        account: String,
    },
    /// Проверить 6-значный код
    Verify {
        /// Секрет в Base32
        #[arg(long)]
        secret: String,
        /// 6-значный код из приложения
        #[arg(long)]
        code: String,
    },
}
