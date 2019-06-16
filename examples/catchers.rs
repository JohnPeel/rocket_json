use rocket_json::catchers;

fn main() {
    rocket::ignite().register(catchers::All()).launch();
}
