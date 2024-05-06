use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files as afs;

const IP: &str = "127.0.0.1:8080";

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
            &format!("http://{IP}/images/leg_press.png")
        )
    );

    exercises.push_back(
        Exercise::new(
            2,
            "legs",
            "Сгибания",
            "Сгибание ног в тренажёре — упражнение для развития силы и наращивания объёма мышц на задней стороне бедра.",
            &format!("http://{IP}/images/leg_curl.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            3,
            "legs",
            "Присяд со штангой",
            "Положите штангу на плечи, выставьте одну ногу вперед, вторую отведите назад. На вдохе опустите таз до параллели бедра с полом. На выдохе вернитесь в исходное положение. Упражнение эффективно нагружает мышцы бедер и ягодиц.",
            &format!("http://{IP}/images/squat_with_barbell.jpg")
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
        .bind(IP)?
        .run()
        .await
}

