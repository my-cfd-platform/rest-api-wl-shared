use std::sync::Arc;

use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpServerMiddleware, HttpServerRequestFlow,
};
use my_no_sql_tcp_reader::MyNoSqlDataReader;

use crate::{my_no_sql::SessionEntity, ApiResultStatus};

use super::{AuthenticationFailedApiResponse, TradingPlatformRequestCredentials};

const AUTH_HEADER: &str = "authorization";

pub struct AuthMiddleware {
    sessions_reader: Arc<MyNoSqlDataReader<SessionEntity>>,
}

impl AuthMiddleware {
    pub fn new(sessions_reader: Arc<MyNoSqlDataReader<SessionEntity>>) -> Self {
        Self { sessions_reader }
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for AuthMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        let auth_header = ctx.request.get_headers().get(AUTH_HEADER);

        if auth_header.is_none() {
            return Err(AuthenticationFailedApiResponse::new(
                ApiResultStatus::AccessTokenInvalid,
                "AccessToken not found".to_string(),
            ));
        }

        let auth_header = auth_header.unwrap();

        let token = extract_token(auth_header.as_bytes());

        let token = match std::str::from_utf8(token) {
            Ok(result) => result,
            Err(_) => {
                return Err(AuthenticationFailedApiResponse::new(
                    ApiResultStatus::AccessTokenInvalid,
                    "AccessToken is broken".to_string(),
                ))
            }
        };

        let token_entity = self
            .sessions_reader
            .get_entity(crate::my_no_sql::PARTITION_KEY_VALUE, token)
            .await;

        if token_entity.is_none() {
            return Err(AuthenticationFailedApiResponse::new(
                ApiResultStatus::AccessTokenInvalid,
                "Invalid or expired AccessToken".to_string(),
            ));
        }

        ctx.credentials = Some(Box::new(TradingPlatformRequestCredentials::new(
            token_entity.unwrap(),
        )));

        get_next.next(ctx).await
    }
}

fn extract_token(src: &[u8]) -> &[u8] {
    if src[6] == b' ' {
        return &src[7..];
    }
    src
}
