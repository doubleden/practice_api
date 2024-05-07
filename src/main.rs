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
            "Присяд со штангой",
            "Положите штангу на плечи, выставьте одну ногу вперед, вторую отведите назад. На вдохе опустите таз до параллели бедра с полом. На выдохе вернитесь в исходное положение. Упражнение эффективно нагружает мышцы бедер и ягодиц.",
            &format!("http://{IP}/images/squat_with_barbell.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            3,
            "legs",
            "Присяд с гантелей",
            "Альтернатива упражнения - присяд со штангой",
            &format!("http://{IP}/images/squat_with_dumbbells.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            4,
            "legs",
            "Выпады назад",
            "Упражнение выпадов назад 一 это упражнение, которое направлено на развитие мышц ног, таких как ягодицы, бедра и икры.",
            &format!("http://{IP}/images/back_lunges.jpg.webp")
        )
    );

    exercises.push_back(
        Exercise::new(
            5,
            "legs",
            "Болгарский выпад",
            "упражнение для тренировки нижней части тела, особенно ягодичных и бедренных мышц.",
            &format!("http://{IP}/images/lunges_with_bench.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            6,
            "legs",
            "Выпады вперед",
            "Классика",
            &format!("http://{IP}/images/lunge_steps.jpeg.webp")
        )
    );

    exercises.push_back(
        Exercise::new(
            7,
            "legs",
            "Сгибание ног",
            "Сгибание ног в тренажёре — упражнение для развития силы и наращивания объёма мышц на задней стороне бедра.",
            &format!("http://{IP}/images/leg_curl.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            8,
            "legs",
            "Разгибания ног",
            "Разгибания ног в тренажёре — упражнение для развития силы и наращивания объёма мышц на передней стороне бедра.",
            &format!("http://{IP}/images/leg_extensions.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            9,
            "legs",
            "Тяга классика",
            "Большая часть нагрузки приходится на спину — разгибатели и трапециевидные мышцы.",
            &format!("http://{IP}/images/traction_classic.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            10,
            "legs",
            "Тяга сумо",
            "Приседания сумо — это комплексное упражнение, при выполнении которого работает много мышц. Основные мышцы: квадрицепсы, ягодичные мышцы, бицепс бедра, икры и внутренняя поверхность бедер, и другие, более мелкие, внутренние мышцы бедра.",
            &format!("http://{IP}/images/sumo_deadlift.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            11,
            "legs",
            "Пистолетик",
            "«Пистолетик» — приседание на одной ноге без опоры, относится к силовым упражнениям. Его биомеханика аналогична обычным приседаниям. Но выполнять «пистолетик» значительно тяжелее, так как рабочая нога получает непривычную нагрузку.",
            &format!("http://{IP}/images/gun_squat.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            12,
            "legs",
            "Броски гири",
            "Броски гири, так или иначе, приводит в тонус все мышцы, но в большей степени участвуют мышцы бедра, ягодиц, спины и живота",
            &format!("http://{IP}/images/throws_weights.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            13,
            "legs",
            "Махи лёжа на боку",
            "Такие махи направлены на укрепление внутренней поверхности бедра и нижней части брюшной мускулатуры.",
            &format!("http://{IP}/images/swing_lying.png")
        )
    );

    exercises.push_back(
        Exercise::new(
            14,
            "legs",
            "Ягодичный мостик",
            "Ягодичный мостик – это наилучшее упражнения для проработки ягодичной мышцы.",
            &format!("http://{IP}/images/gluteal_bridge.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            15,
            "legs",
            "Свидение ног",
            "Сведение ног – востребованное физическое упражнение для укрепления нижнего пояса.",
            &format!("http://{IP}/images/date_legs.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            16,
            "legs",
            "Развидение ног",
            "Разведение ног - позволяет тренировать внешние (отводящие) мышцы бёдер и ягодичные мышцы.",
            &format!("http://{IP}/images/unraveling_legs.jpg")
        )
    );
    // LEGS END



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

