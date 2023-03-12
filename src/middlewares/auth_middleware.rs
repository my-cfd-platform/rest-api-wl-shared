use std::sync::Arc;

use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpServerMiddleware, HttpServerRequestFlow,
};
use my_no_sql_tcp_reader::MyNoSqlDataReader;

use super::{SessionEntity, TradingPlatformRequestCredentials};

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
            return get_next.next(ctx).await;
        }

        let auth_header = auth_header.unwrap();

        let token = extract_token(auth_header.as_bytes());

        if token.is_none() {
            return get_next.next(ctx).await;
        }

        let token = match std::str::from_utf8(token.unwrap()) {
            Ok(result) => result,
            Err(_) => {
                return get_next.next(ctx).await;
            }
        };

        let token_entity = self
            .sessions_reader
            .get_entity(&SessionEntity::get_pk(), token)
            .await;

        if token_entity.is_none() {
            return get_next.next(ctx).await;
        }

        ctx.credentials = Some(Box::new(TradingPlatformRequestCredentials::new(
            token_entity.unwrap(),
        )));

        get_next.next(ctx).await
    }
}

fn extract_token(src: &[u8]) -> Option<&[u8]> {
    if src.len() == 0 {
        return None;
    }
    if src[6] == b' ' {
        return Some(&src[7..]);
    }
    Some(src)
}
