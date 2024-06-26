use service_sdk::{my_http_server::HttpContext, my_no_sql_sdk::reader::MyNoSqlDataReaderTcp};

use super::BrandMyNoSqlEntity;

#[async_trait::async_trait]
pub trait BrandIdResolver {
    async fn resolve_brand_id(&self, brands: MyNoSqlDataReaderTcp<BrandMyNoSqlEntity>) -> String;
}

#[async_trait::async_trait]
impl BrandIdResolver for HttpContext {
    async fn resolve_brand_id(&self, brands: MyNoSqlDataReaderTcp<BrandMyNoSqlEntity>) -> String {
        let url = self.request.get_host().to_lowercase();

        let item = brands
            .get_entity(BrandMyNoSqlEntity::PARTITION_KEY, url.as_str())
            .await;

        match item {
            Some(item) => item.brand_id.to_string(),
            None => "default".to_string(),
        }
    }
}
