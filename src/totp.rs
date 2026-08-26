use crate::config::{DIGITS, SKEW, STEP};
use qrcode::QrCode;
use std::time::{SystemTime, UNIX_EPOCH};
use totp_rs::{Algorithm, Builder, Secret, Totp};

#[derive(Debug, PartialEq)]
pub enum TotpError {
    InvalidSecret,
    InvalidCode,
    SystemTime,
}

pub fn build_totp(secret_b32: &str, issuer: &str, account: &str) -> Result<Totp, TotpError> {
    let secret = Secret::try_from_base32(secret_b32).map_err(|_| TotpError::InvalidSecret)?;
    Builder::new()
        .with_secret(secret.as_bytes().to_vec())
        .with_algorithm(Algorithm::SHA1)
        .with_digits(DIGITS as u8)
        .with_skew(SKEW)
        .with_step_duration(STEP)
        .with_issuer(Some(issuer.to_string()))
        .with_account_name(account.to_string())
        .build()
        .map_err(|_| TotpError::InvalidSecret)
}

pub fn generate_secret() -> String {
    Secret::generate().to_base32()
}

pub fn verify_code(secret_b32: &str, code: &str) -> Result<bool, TotpError> {
    if code.len() != DIGITS || !code.chars().all(|c| c.is_ascii_digit()) {
        return Err(TotpError::InvalidCode);
    }

    let totp = build_totp(secret_b32, "totp-cli", "user")?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| TotpError::SystemTime)?
        .as_secs();

    Ok(totp.check(code, now).is_some())
}

pub fn run_verify(secret: &str, code: &str) -> i32 {
    match verify_code(secret, code) {
        Ok(true) => {
            println!("Успешно (код валиден)");
            0
        }
        Ok(false) => {
            println!("Ошибка (код неверен или просрочен)");
            1
        }
        Err(TotpError::InvalidSecret) => {
            eprintln!("Ошибка: секрет не является валидным Base32");
            2
        }
        Err(TotpError::InvalidCode) => {
            eprintln!("Ошибка: код должен состоять ровно из 6 цифр");
            2
        }
        Err(TotpError::SystemTime) => {
            eprintln!("Ошибка: не удалось получить системное время");
            2
        }
    }
}

pub fn run_generate(issuer: &str, account: &str) -> i32 {
    let secret_b32 = generate_secret();
    let totp = match build_totp(&secret_b32, issuer, account) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("Ошибка: не удалось создать TOTP из сгенерированного секрета");
            return 2;
        }
    };

    let url = match totp.to_url() {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Ошибка генерации URL: {e}");
            return 2;
        }
    };

    println!("Секрет (Base32): {secret_b32}");
    println!("URI: {url}");

    match QrCode::new(url.as_bytes()) {
        Ok(code) => {
            let image = code
                .render::<qrcode::render::unicode::Dense1x2>()
                .quiet_zone(true)
                .build();
            println!("{image}");
        }
        Err(e) => {
            eprintln!("Ошибка генерации QR-кода: {e}");
            return 2;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn valid_secret() -> String {
        generate_secret()
    }

    /// Тестовые векторы из спецификации RFC 6238 (Appendix B)
    /// Секрет ASCII: "12345678901234567890", 8 цифр, step=30, SHA1
    #[rstest]
    #[case(59, "94287082")]
    #[case(1111111109, "07081804")]
    #[case(1111111111, "14050471")]
    #[case(1234567890, "89005924")]
    #[case(2000000000, "69279037")]
    fn rfc6238_sha1_vectors(#[case] timestamp: u64, #[case] expected: &str) {
        let totp = Builder::new()
            .with_secret(b"12345678901234567890".to_vec())
            .with_algorithm(Algorithm::SHA1)
            .with_digits(8)
            .with_skew(1)
            .with_step_duration(30)
            .with_account_name("user".to_string())
            .build()
            .unwrap();
        assert_eq!(totp.generate(timestamp).to_string(), expected);
    }

    #[rstest]
    fn generated_secret_is_20_bytes_base32(valid_secret: String) {
        let bytes = Secret::try_from_base32(&valid_secret).unwrap();
        assert_eq!(bytes.as_bytes().len(), 20);
    }

    #[rstest]
    #[case("!!!not-base32!!!")]
    #[case("12345678901234567890")] // Содержит недопустимые для Base32 символы (8, 9, 0, 1)
    #[case(" ")]
    #[case("")]
    fn build_totp_rejects_invalid_base32(#[case] invalid_secret: &str) {
        assert_eq!(
            build_totp(invalid_secret, "issuer", "user"),
            Err(TotpError::InvalidSecret)
        );
    }

    #[rstest]
    fn build_totp_accepts_valid_secret(valid_secret: String) {
        assert!(build_totp(&valid_secret, "issuer", "user").is_ok());
    }

    #[rstest]
    #[case("GitHub", "user@example.com")]
    #[case("Google", "alice@gmail.com")]
    #[case("CustomService", "admin_root")]
    fn run_generate_succeeds(#[case] issuer: &str, #[case] account: &str) {
        assert_eq!(run_generate(issuer, account), 0);
    }

    #[rstest]
    fn verify_accepts_current_code(valid_secret: String) {
        let totp = build_totp(&valid_secret, "issuer", "user").unwrap();
        let code = totp.generate_current().to_string();
        assert_eq!(verify_code(&valid_secret, &code), Ok(true));
    }

    #[rstest]
    #[case("000000")]
    #[case("999999")]
    #[case("123456")]
    fn verify_rejects_wrong_code(valid_secret: String, #[case] arbitrary_code: &str) {
        let totp = build_totp(&valid_secret, "issuer", "user").unwrap();
        let code = totp.generate_current().to_string();
        let wrong = if code == arbitrary_code {
            "111111"
        } else {
            arbitrary_code
        };
        assert_eq!(verify_code(&valid_secret, wrong), Ok(false));
    }

    #[rstest]
    #[case("12345")] // 5 цифр (слишком короткий)
    #[case("1234567")] // 7 цифр (слишком длинный)
    #[case("12a456")] // содержит буквы
    #[case("abcdef")] // только буквы
    #[case("12 456")] // содержит пробел
    #[case("")] // пустой
    #[case("-12345")] // отрицательный/спецсимволы
    #[case("12.345")] // с точкой
    fn verify_rejects_bad_code_format(valid_secret: String, #[case] bad_code: &str) {
        assert_eq!(
            verify_code(&valid_secret, bad_code),
            Err(TotpError::InvalidCode)
        );
    }

    #[rstest]
    #[case("!!!not-base32!!!")]
    #[case("12345678901234567890")]
    #[case("invalid!base32?")]
    fn verify_rejects_invalid_secret(#[case] invalid_secret: &str) {
        assert_eq!(
            verify_code(invalid_secret, "123456"),
            Err(TotpError::InvalidSecret)
        );
    }
}
