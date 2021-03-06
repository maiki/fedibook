#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate failure;
extern crate rocket_contrib;
extern crate serde;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate ring;
extern crate diesel;

extern crate _fedibook as fedibook;

use ring::rand::SystemRandom;
use rocket::Rocket;
use rocket_contrib::Template;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn db_pool(rocket: &Rocket) -> Pool {
    let database_url = rocket.config().get_str("database_url").expect("Must set DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Could not get DB connection pool")
}

fn app() -> Rocket {
    let r = rocket::ignite()
        .mount("/api/v1", routes![
            fedibook::routes::applications::register_application
        ])
        .mount("/", routes![
            fedibook::routes::auth::sign_up_form,
            fedibook::routes::auth::sign_in_form,
            fedibook::routes::auth::sign_up,
            fedibook::routes::auth::sign_in,
            fedibook::routes::auth::confirm,
            fedibook::routes::auth::sign_out,

            fedibook::routes::app::home,
            fedibook::routes::app::home_redirect,
        ])
        .attach(Template::fairing())
        .manage(SystemRandom::new());

    // we need an instance of the app to access the config values in Rocket.toml,
    // so we pass it to the db_pool function, get the pool, and _then_ return the instance
    let pool = db_pool(&r);
    r.manage(pool)
}

fn main() {
    app().launch();
}

