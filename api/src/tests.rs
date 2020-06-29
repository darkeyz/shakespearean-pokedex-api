use super::rocket;
use rocket::local::Client;
use rocket::http::Status;


#[test]
fn existing_pokemon() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/pokemon/pikachu").dispatch();
    assert!(response.status() == Status::Ok || response.status() == Status::TooManyRequests);
}

#[test]
fn unexisting_pokemon() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/pokemon/skrfhkrw").dispatch();
    assert!(response.status() == Status::BadRequest);
}
