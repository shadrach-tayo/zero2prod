use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

mod get;
mod post;

pub use get::login_form;
pub use post::login;
