use actix_web::{Responder, Error, HttpResponse, http::StatusCode, HttpRequest};
use futures::future::{ready, Ready};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Data<T> where T: Serialize {
    Str(String),
    Json(T),
    JsonVec(Vec<T>),
    None(Option<bool>)
}

#[derive(Serialize, Deserialize)]
pub struct Response<T> where T: Serialize {
    #[serde(rename = "statusCode")]
    pub status_code: i32,
    pub data: Data<T>
}

impl<T> Responder for Response<T> where T: Serialize {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::build(StatusCode::from_u16(self.status_code as u16).unwrap())
            .content_type("application/json")
            .body(body))
        )
    }
}