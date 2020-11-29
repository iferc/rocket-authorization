#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "ok"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}
fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn root_available() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("ok".into()));
    }
}
