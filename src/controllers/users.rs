use actix_web::{get, delete, Responder, HttpRequest, web};

use crate::database;
use crate::models;
use crate::diesel::prelude::*;
use crate::schema;
use crate::misc;

#[get("")]
async fn get_users() -> impl Responder {
    let db = database::get_connection()
        .as_ref()
        .expect("could not get database connection");
    let users = schema::users::table
        .load::<models::User>(db)
        .expect("could not get users");

    misc::Response{
        status_code: 200,
        data: misc::Data::JsonVec(users),
    }
}

#[delete("/{id}")]
async fn delete_user(_req: HttpRequest, web::Path((id,)): web::Path<(String,)>) -> impl Responder {
    let db = database::get_connection()
        .as_ref()
        .expect("could not get database connection");
    let user_id = id.parse::<i32>().unwrap();
    diesel::delete(schema::users::table.filter(schema::users::id.eq(user_id)))
        .execute(db)
        .expect("error deleting user");
    misc::Response::<models::User>{
        status_code: 200,
        data: misc::Data::Str(format!("user {} deleted", id)),
    }
}

#[get("/@me")]
async fn get_user_me() -> impl Responder {
    misc::Response::<models::User>{
        status_code: 200,
        data: misc::Data::None(None),
    }
}