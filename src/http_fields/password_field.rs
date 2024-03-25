use rust_extensions::StrOrString;

service_sdk::macros::use_my_http_server!();

#[http_input_field]
pub struct PasswordField(String);

fn process_value(src: &str) -> Result<StrOrString, HttpFailResult> {
    if src.len() < 8 {
        return Err(HttpFailResult::as_validation_error(format!(
            "Password must have at least {} chars",
            src.len()
        )));
    }

    let mut uppercase_amount = 0;
    let mut lowercase_amount = 0;
    let mut digits = 0;

    let mut special_symbol = 0;

    for c in src.chars() {
        if c.is_numeric() {
            digits += 1;
        } else if c.is_uppercase() {
            uppercase_amount += 1;
        } else if c.is_lowercase() {
            lowercase_amount += 1;
        } else {
            special_symbol += 1;
        }
    }

    if uppercase_amount == 0 {
        return Err(HttpFailResult::as_validation_error(
            "Password must have at least one uppercase letter".to_string(),
        ));
    }

    if lowercase_amount == 0 {
        return Err(HttpFailResult::as_validation_error(
            "Password must have at least one lowercase letter".to_string(),
        ));
    }

    if digits == 0 {
        return Err(HttpFailResult::as_validation_error(
            "Password must have at least one digit".to_string(),
        ));
    }

    if special_symbol == 0 {
        return Err(HttpFailResult::as_validation_error(
            "Password must have at least one special symbol".to_string(),
        ));
    }

    Ok(StrOrString::create_as_str(src))
}
