use actix_web::web;

// Route: /users
mod users;
pub fn users(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/users")
        .service(users::get_users) // GET /
        .service(users::delete_user) // DELETE /{id}
        .service(users::get_user_me) // GET /@me
    );
}