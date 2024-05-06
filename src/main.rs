use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files as afs;

#[derive(Serialize, Deserialize)]
struct Exercise {
    id: u32,
    category: String,
    name: String,
    description: String,
    image: String,
}

impl Exercise {
    fn new(id: u32, category: &str, name: &str, description: &str, photo_url: &str) -> Self {
        Exercise {
            id,
            category: category.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            image: photo_url.to_string(),
        }
    }
}

async fn get_exercises() -> impl Responder {
    let mut exercises = VecDeque::new();
    exercises.push_back(
        Exercise::new(
            1,
            "legs",
            "Жим ног",
            "Упражнение для прокачки ног, которое выполняется в специальном тренажёре, горизонтальном или вертикальном (наклонном)",
            "http://127.0.0.1:8080/images/leg_press.png"
        )
    );

    exercises.push_back(
        Exercise::new(
            2,
            "legs",
            "Сгибания",
            "Сгибание ног в тренажёре — упражнение для развития силы и наращивания объёма мышц на задней стороне бедра.",
            "http://127.0.0.1:8080/images/leg_curl.jpg"
        )
    );

    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(exercises)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(afs::Files::new("/images", "static/images").show_files_listing())
            .route("/trainings", web::get().to(get_exercises))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

