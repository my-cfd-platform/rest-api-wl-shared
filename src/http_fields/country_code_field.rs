use rust_common::country_code::CountryCode;
use rust_extensions::StrOrString;

service_sdk::macros::use_my_http_server!();

#[http_input_field]
pub struct CountryCodeField(String);

fn process_value(src: &str) -> Result<StrOrString, HttpFailResult> {
    let value = src.trim().to_uppercase();

    let parsed = CountryCode::parse(value.as_str());

    if parsed.is_err() {
        return Err(HttpFailResult::as_validation_error(
            "Country code is not valid".to_string(),
        ));
    }

    Ok(StrOrString::create_as_string(value))
}

impl Into<CountryCode> for CountryCodeField {
    fn into(self) -> CountryCode {
        CountryCode::parse(self.0.as_str()).unwrap()
    }
}
