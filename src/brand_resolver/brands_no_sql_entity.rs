use serde::*;
service_sdk::macros::use_my_no_sql_entity!();

//todo!("This entity must be removed from my-nosql shared library");
#[my_no_sql_entity("brands")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrandMyNoSqlEntity {
    #[serde(rename = "brandId")]
    pub brand_id: String,

    #[serde(rename = "logoUrl")]
    pub logo_url: String,
    #[serde(rename = "policyUrl")]
    pub policy_url: String,
    #[serde(rename = "termsUrl")]
    pub terms_url: String,
    #[serde(rename = "withdrawFaqUrl")]
    pub withdraw_faq_url: String,
    #[serde(rename = "aboutUrl")]
    pub about_url: String,
    #[serde(rename = "supportUrl")]
    pub support_url: String,

    #[serde(rename = "brandName")]
    pub brand_name: String,

    #[serde(rename = "brandCopyrights")]
    pub brand_copyrights: String,
    #[serde(rename = "gaAsAccount")]
    pub ga_as_account: String,

    #[serde(rename = "mixPanelToken")]
    pub mix_panel_token: Option<String>,

    #[serde(rename = "faviconUrl")]
    pub favicon_url: String,

    #[serde(rename = "androidAppId")]
    pub android_app_id: Option<String>,
    #[serde(rename = "androidAppLink")]
    pub android_app_link: Option<String>,

    #[serde(rename = "iosAppId")]
    pub ios_app_id: Option<String>,
    #[serde(rename = "iosAppLink")]
    pub ios_app_link: Option<String>,

    #[serde(rename = "mobileAppLogo")]
    pub mobile_app_logo: Option<String>,

    #[serde(rename = "logInUrl")]
    pub login_url: Option<String>,

    #[serde(rename = "hideSecurityTab")]
    pub hide_security_tab: Option<bool>,

    #[serde(rename = "hideAccountInfo")]
    pub hide_account_info: Option<bool>,

    #[serde(rename = "hideVolumeAmountInCurrency")]
    pub hide_volume_amount_in_currency: Option<bool>,

    #[serde(rename = "hideKycContent")]
    pub hide_kyc_content: Option<bool>,

    #[serde(rename = "hideDeposit")]
    pub hide_deposit: Option<bool>,

    #[serde(rename = "hideSignUp")]
    pub hide_sign_up: Option<bool>,

    #[serde(rename = "hideAboutPageUrl")]
    pub hide_about_page_url: Option<bool>,

    #[serde(rename = "RecaptchaPublicKey")]
    pub recaptcha_public_key: Option<String>,

    #[serde(rename = "RecaptchaPrivateKey")]
    pub recaptcha_private_key: Option<String>,

    #[serde(rename = "RecaptchaScoreToVerify")]
    pub recaptcha_score_to_verify: Option<f64>,

    #[serde(rename = "defaultChartTimeFrame")]
    pub default_chart_time_frame: Option<String>,
}

impl BrandMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "b";

    pub fn get_domain(&self) -> &str {
        self.row_key.as_str()
    }
}
