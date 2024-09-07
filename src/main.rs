mod api;

use crate::api::IMAGE_IP;
use actix_files as afs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

enum Category {
    Legs,
    Shoulders,
    Arms,
    Chest,
    Back,
}

impl Category {
    fn as_str(&self) -> &str {
        match self {
            Category::Arms => "Arms",
            Category::Back => "Back",
            Category::Shoulders => "Shoulders",
            Category::Legs => "Legs",
            Category::Chest => "Chest",
        }
    }
}

const IP: &str = "0.0.0.0:8081";
#[derive(Serialize, Deserialize)]
struct Exercise {
    id: Uuid,
    category: String,
    name: String,
    description: String,
    image: String,
}

impl Exercise {
    fn new(category: &str, name: &str, description: &str, photo_url: &str) -> Self {
        Exercise {
            id: Uuid::new_v4(),
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
            Category::Legs.as_str(),
            "Leg Press",
            "A leg exercise performed in a special machine, either horizontal or vertical (inclined)",
            &format!("{IMAGE_IP}/leg_press.png"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Squat with Barbell",
            "Place the barbell on your shoulders, step one foot forward, and the other back. Inhale as you lower your hips until your thigh is parallel to the floor. Exhale as you return to the starting position. This exercise effectively targets the muscles of the thighs and glutes.",
            &format!("{IMAGE_IP}/squat_with_barbell.jpg"),
        ),
    );

    exercises.push_back(Exercise::new(
        Category::Legs.as_str(),
        "Dumbbell Squat",
        "An alternative exercise to the barbell squat.",
        &format!("{IMAGE_IP}/squat_with_dumbbells.jpeg"),
    ));

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Reverse Lunges",
            "Reverse lunges are an exercise aimed at developing leg muscles such as the glutes, thighs, and calves.",
            &format!("{IMAGE_IP}/back_lunges.jpg.webp"),
        ),
    );

    exercises.push_back(Exercise::new(
        Category::Legs.as_str(),
        "Bulgarian Split Squat",
        "An exercise for training the lower body, especially the glutes and thigh muscles.",
        &format!("{IMAGE_IP}/lunges_with_bench.jpg"),
    ));

    exercises.push_back(Exercise::new(
        Category::Legs.as_str(),
        "Forward Lunges",
        "A classic exercise.",
        &format!("{IMAGE_IP}/lunge_steps.jpeg.webp"),
    ));

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Leg Curl",
            "Leg curls in a machine — an exercise for developing strength and increasing muscle volume in the hamstrings.",
            &format!("{IMAGE_IP}/leg_curl.jpg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Leg Extensions",
            "Leg extensions in a machine — an exercise for developing strength and increasing muscle volume in the quadriceps.",
            &format!("{IMAGE_IP}/leg_extensions.jpg"),
        ),
    );

    exercises.push_back(Exercise::new(
        Category::Legs.as_str(),
        "Conventional Deadlift",
        "Most of the load is placed on the back muscles — extensors and trapezius muscles.",
        &format!("{IMAGE_IP}/traction_classic.jpeg"),
    ));

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Sumo Deadlift",
            "Sumo squats are a compound exercise that involves many muscles. Main muscles: quadriceps, glutes, hamstrings, calves, and inner thigh muscles, among others.",
            &format!("{IMAGE_IP}/sumo_deadlift.jpg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Pistol Squat",
            "The pistol squat is a single-leg squat without support, classified as a strength exercise. Its biomechanics are similar to regular squats, but performing the pistol squat is significantly more difficult due to the increased load on the working leg.",
            &format!("{IMAGE_IP}/gun_squat.jpg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Kettlebell Throws",
            "Kettlebell throws, one way or another, tone all muscles, but mostly involve the muscles of the thighs, glutes, back, and abs.",
            &format!("{IMAGE_IP}/throws_weights.jpeg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Side-Lying Leg Raises",
            "These leg raises target the inner thighs and lower abdominal muscles.",
            &format!("{IMAGE_IP}/swing_lying.png"),
        ),
    );

    exercises.push_back(Exercise::new(
        Category::Legs.as_str(),
        "Glute Bridge",
        "The glute bridge is one of the best exercises for targeting the gluteal muscles.",
        &format!("{IMAGE_IP}/gluteal_bridge.jpg"),
    ));

    exercises.push_back(Exercise::new(
        Category::Legs.as_str(),
        "Leg Adduction",
        "Leg adduction is a popular exercise for strengthening the lower body.",
        &format!("{IMAGE_IP}/date_legs.jpg"),
    ));

    exercises.push_back(Exercise::new(
        Category::Legs.as_str(),
        "Leg Abduction",
        "Leg abduction allows you to train the outer thigh muscles and glutes.",
        &format!("{IMAGE_IP}/unraveling_legs.jpg"),
    ));

    // LEGS END
    exercises.push_back(Exercise::new(
        Category::Chest.as_str(),
        "Bench Press",
        "The bench press is a basic exercise with free weights.",
        &format!("{IMAGE_IP}/bench_press.jpg.webp"),
    ));

    exercises.push_back(
        Exercise::new(
            Category::Chest.as_str(),
            "Incline Bench Press",
            "Changing the angle of the press allows you to focus on specific parts of the chest, contributing to the desired muscle definition.",
            &format!("{IMAGE_IP}/bench_press30.jpg.jpeg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Chest.as_str(),
            "Dumbbell Bench Press",
            "The dumbbell bench press provides a greater range of motion because each arm moves independently.",
            &format!("{IMAGE_IP}/bench_dmb.png"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Chest.as_str(),
            "Incline Dumbbell Bench Press",
            "The dumbbell bench press provides a greater range of motion because each arm moves independently. The load targets the upper chest.",
            &format!("{IMAGE_IP}/bench_dmb30.jpeg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Chest.as_str(),
            "Close-Grip Bench Press",
            "The close-grip bench press is a basic exercise for the chest muscles and triceps, where the hands are about 40 cm apart on the bar.",
            &format!("{IMAGE_IP}/bench_close.jpeg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Chest.as_str(),
            "Pullover",
            "The pullover is an isolated exercise for targeting the upper back, with some involvement of the chest muscles and triceps.",
            &format!("{IMAGE_IP}/pullover.jpeg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Chest.as_str(),
            "Machine Fly",
            "In the machine, the muscle is always under load. The exercise becomes harder as the arms come together.",
            &format!("{IMAGE_IP}/chest_together.jpeg"),
        ),
    );

    exercises.push_back(Exercise::new(
        Category::Chest.as_str(),
        "Cable Fly",
        "An effective isolated exercise for developing chest muscles.",
        &format!("{IMAGE_IP}/chest_together_cross.jpeg"),
    ));

    exercises.push_back(Exercise::new(
        Category::Chest.as_str(),
        "Dips",
        "The load is focused on the triceps and chest muscles.",
        &format!("{IMAGE_IP}/bars.png"),
    ));

    exercises.push_back(
        Exercise::new(
            Category::Chest.as_str(),
            "Push-Ups",
            "Push-ups are a basic exercise performed in a plank position, involving lowering and raising the body using the arms from the floor, bench, chair, table, wall, etc.",
            &format!("{IMAGE_IP}/pushup.jpeg"),
        ),
    );
    //END CHEST

    exercises.push_back(
        Exercise::new(
            Category::Shoulders.as_str(),
            "Overhead Press",
            "A basic exercise with classic technique (standing with a barbell) engages many muscle groups but primarily targets the front delts, making the shoulders visually larger.",
            &format!("{IMAGE_IP}/shoulder_press.jpeg"),
        ),
    );
    exercises.push_back(
        Exercise::new(
            Category::Shoulders.as_str(),
            "Seated Dumbbell Press",
            "The seated dumbbell press is a classic exercise for targeting the front and middle deltoid muscles, with some involvement of the triceps.",
            &format!("{IMAGE_IP}/seated_press.jpeg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Shoulders.as_str(),
            "Halo",
            "The halo is a great exercise for improving shoulder mobility, introduced by Russian kettlebell sport master Steve Maxwell.",
            &format!("{IMAGE_IP}/galo.jpeg"),
        ),
    );

    exercises.push_back(Exercise::new(
        Category::Shoulders.as_str(),
        "Lateral Raise",
        "The supraspinatus and deltoid muscles are responsible for raising the arm to the side of the body.",
        &format!("{IMAGE_IP}/visions.png"),
    ));

    exercises.push_back(
        Exercise::new(
            Category::Shoulders.as_str(),
            "Butterfly",
            "The upright row is a basic exercise for training the deltoid muscles (mainly the middle head). The trapezius muscles are also involved.",
            &format!("{IMAGE_IP}/pull_head.jpeg"),
        ),
    );

    exercises.push_back(Exercise::new(
        Category::Shoulders.as_str(),
        "Wolverine",
        "An exercise for the rear delts. Cross your arms, then perform a fly.",
        &format!("{IMAGE_IP}/wolf.jpeg"),
    ));
    //END SHOULDERS

    exercises.push_back(
        Exercise::new(
            Category::Back.as_str(),
            "Lat Pulldown",
            "This is one of the basic exercises in fitness and bodybuilding that targets the outer edges of the lats, as well as the upper part of the chest, to a lesser extent the deltoid muscles and biceps.",
            &format!("{IMAGE_IP}/upper_block.jpeg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Back.as_str(),
            "Low Row",
            "The low row effectively strengthens the muscles of the mid and lower back, thereby improving posture.",
            &format!("{IMAGE_IP}/down_block.jpeg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Back.as_str(),
            "Hammer Strength Row",
            "The vertical hammer strength row with a wide grip targets the upper lats, and also involves the rhomboid and trapezius muscles.",
            &format!("{IMAGE_IP}/linkage.jpeg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Back.as_str(),
            "Bent-Over Row",
            "This exercise engages the middle and lower trapezius, lats, and infraspinatus muscles, spinal extensors, and rear delts.",
            &format!("{IMAGE_IP}/bent_over.jpeg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Back.as_str(),
            "One-Arm Dumbbell Row",
            "The main working muscle group in the one-arm dumbbell row is the lats.",
            &format!("{IMAGE_IP}/one_arm_dumbbell.jpeg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Back.as_str(),
            "Pull-Ups",
            "Pull-ups are a basic exercise that develops the muscle groups of the upper body: lats, biceps, brachialis, chest, upper back, core muscles, and forearms.",
            &format!("{IMAGE_IP}/pullup.webp"),
        ),
    );

    exercises.push_back(Exercise::new(
        Category::Back.as_str(),
        "Hyperextension",
        "This is an isolated exercise for developing the spinal extensors.",
        &format!("{IMAGE_IP}/hyperextension.webp"),
    ));
    //END BACK

    exercises.push_back(
        Exercise::new(
            Category::Arms.as_str(),
            "Triceps Rope Pushdown",
            "Lower the cable machine smoothly, hold the result in the lowest position for a couple of seconds, as this is the point where the triceps are under the most tension.",
            &format!("{IMAGE_IP}/ropes.jpeg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Arms.as_str(),
            "French Press",
            "The French press is an exercise where the main load is on the triceps. The chest, shoulders, and forearms are also involved, but to a lesser extent.",
            &format!("{IMAGE_IP}/french.jpeg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Arms.as_str(),
            "Standing Barbell Curl",
            "While lifting the barbell, do not move your elbows, keep them by your sides, and do not bend your wrists.",
            &format!("{IMAGE_IP}/biceps_class.jpeg"),
        ),
    );

    exercises.push_back(
        Exercise::new(
            Category::Arms.as_str(),
            "Hammer Curl",
            "The hammer curl is an isolating exercise for developing the lateral part of the biceps, brachialis, and brachioradialis muscles.",
            &format!("{IMAGE_IP}/hummers.jpeg"),
        ),
    );

    exercises.push_back(Exercise::new(
        Category::Arms.as_str(),
        "Cable Biceps Curl",
        "This exercise targets all five components of the biceps during the curl.",
        &format!("{IMAGE_IP}/biceps_block.jpeg"),
    ));

    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(exercises)
}
# [actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(afs::Files::new("/images", "static/images").show_files_listing())
            .route("/trainings_en", web::get().to(get_exercises))
    })
        .bind(IP)?
        .run()
        .await
}