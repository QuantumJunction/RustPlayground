#[macro_use] extern crate rocket;

#[get("/<param>", rank = 2)]
fn test_par(param: &str) -> String {
    format!("Test par {}!", param)
}

#[get("/")]
fn test_hello() -> String {
    format!("Hello")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![test_hello])
        .mount("/test", routes![test_par])
}