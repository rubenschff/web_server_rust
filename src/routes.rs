use rocket::{get, post};
use rocket::serde::{Deserialize, Serialize, json::Json};
use crate::controllers::user::index;

#[derive(Serialize, Deserialize)]
#[serde(crate  = "rocket::serde")]
pub struct JsonData<'r> {
    description: &'r str,
    complete: bool,
}

#[post("/json_data", data = "<data>")]
pub fn json_data(data: Json<JsonData<'_>>) -> String {
    format!("{} is {}", data.description, data.complete)
}

#[get("/")]
pub fn home() -> &'static str {
    "Hello Rocket"
}

#[get("/name/<name>")]
pub fn hello(name: &str) -> String {
    index::return_name(&name)
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![home, hello, json_data]
}
