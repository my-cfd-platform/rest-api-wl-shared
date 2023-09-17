use service_sdk::{
    my_http_server::controllers::{ControllersAuthorization, RequiredClaims},
    HttpServerBuilder,
};

use crate::middlewares::AuthFailResponseFactory;

pub fn configure_rest_api_server(http_server_builder: &mut HttpServerBuilder) {
    http_server_builder.set_authorization(ControllersAuthorization::BearerAuthentication {
        global: true,
        global_claims: RequiredClaims::no_claims(),
    });

    http_server_builder.set_auth_error_factory(AuthFailResponseFactory);
}
