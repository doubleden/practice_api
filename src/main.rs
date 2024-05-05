use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files as afs;

#[derive(Serialize, Deserialize)]
struct Exercise {
    id: u32,
    name: String,
    description: String,
    image: String,
}

impl Exercise {
    fn new(id: u32, name: &str, description: &str, photo_url: &str) -> Self {
        Exercise {
            id,
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
            "Morning Run",
            "A quick morning run.",
            "http://127.0.0.1:8080/photos/1.png"
        )
    );

    exercises.push_back(
        Exercise::new(
            2,
            "Swimming",
            "Swimming in the pool for 45 minutes.",
            "http://127.0.0.1:8080/photos/2.webp"
        )
    );

    HttpResponse::Ok().json(exercises)
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

