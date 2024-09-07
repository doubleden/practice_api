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
    Back
}

impl Category {
    fn as_str(&self) -> &str {
        match self {
            Category::Arms => "Руки",
            Category::Back => "Спина",
            Category::Shoulders => "Плечи",
            Category::Legs => "Ноги",
            Category::Chest => "Грудь"
        }
    }
}

const IP: &str = "0.0.0.0:8080";
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
            "Жим ног",
            "Упражнение для прокачки ног, которое выполняется в специальном тренажёре, горизонтальном или вертикальном (наклонном)",
            &format!("{IMAGE_IP}/leg_press.png")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Присяд со штангой",
            "Положите штангу на плечи, выставьте одну ногу вперед, вторую отведите назад. На вдохе опустите таз до параллели бедра с полом. На выдохе вернитесь в исходное положение. Упражнение эффективно нагружает мышцы бедер и ягодиц.",
            &format!("{IMAGE_IP}/squat_with_barbell.jpg")
        )
    );

    exercises.push_back(Exercise::new(
        Category::Legs.as_str(),
        "Присяд с гантелей",
        "Альтернатива упражнения - присяд со штангой",
        &format!("{IMAGE_IP}/squat_with_dumbbells.jpeg"),
    ));

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Выпады назад",
            "Упражнение выпадов назад 一 это упражнение, которое направлено на развитие мышц ног, таких как ягодицы, бедра и икры.",
            &format!("{IMAGE_IP}/back_lunges.jpg.webp")
        )
    );

    exercises.push_back(Exercise::new(
        Category::Legs.as_str(),
        "Болгарский выпад",
        "упражнение для тренировки нижней части тела, особенно ягодичных и бедренных мышц.",
        &format!("{IMAGE_IP}/lunges_with_bench.jpg"),
    ));

    exercises.push_back(Exercise::new(
        Category::Legs.as_str(),
        "Выпады вперед",
        "Классика",
        &format!("{IMAGE_IP}/lunge_steps.jpeg.webp"),
    ));

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Сгибание ног",
            "Сгибание ног в тренажёре — упражнение для развития силы и наращивания объёма мышц на задней стороне бедра.",
            &format!("{IMAGE_IP}/leg_curl.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Разгибания ног",
            "Разгибания ног в тренажёре — упражнение для развития силы и наращивания объёма мышц на передней стороне бедра.",
            &format!("{IMAGE_IP}/leg_extensions.jpg")
        )
    );

    exercises.push_back(Exercise::new(
        Category::Legs.as_str(),
        "Тяга классика",
        "Большая часть нагрузки приходится на спину — разгибатели и трапециевидные мышцы.",
        &format!("{IMAGE_IP}/traction_classic.jpeg"),
    ));

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Тяга сумо",
            "Приседания сумо — это комплексное упражнение, при выполнении которого работает много мышц. Основные мышцы: квадрицепсы, ягодичные мышцы, бицепс бедра, икры и внутренняя поверхность бедер, и другие, более мелкие, внутренние мышцы бедра.",
            &format!("{IMAGE_IP}/sumo_deadlift.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Пистолетик",
            "«Пистолетик» — приседание на одной ноге без опоры, относится к силовым упражнениям. Его биомеханика аналогична обычным приседаниям. Но выполнять «пистолетик» значительно тяжелее, так как рабочая нога получает непривычную нагрузку.",
            &format!("{IMAGE_IP}/gun_squat.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Броски гири",
            "Броски гири, так или иначе, приводит в тонус все мышцы, но в большей степени участвуют мышцы бедра, ягодиц, спины и живота",
            &format!("{IMAGE_IP}/throws_weights.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Legs.as_str(),
            "Махи лёжа на боку",
            "Такие махи направлены на укрепление внутренней поверхности бедра и нижней части брюшной мускулатуры.",
            &format!("{IMAGE_IP}/swing_lying.png")
        )
    );

    exercises.push_back(Exercise::new(
        Category::Legs.as_str(),
        "Ягодичный мостик",
        "Ягодичный мостик – это наилучшее упражнения для проработки ягодичной мышцы.",
        &format!("{IMAGE_IP}/gluteal_bridge.jpg"),
    ));

    exercises.push_back(Exercise::new(
        Category::Legs.as_str(),
        "Свидение ног",
        "Сведение ног – востребованное физическое упражнение для укрепления нижнего пояса.",
        &format!("{IMAGE_IP}/date_legs.jpg"),
    ));

    exercises.push_back(Exercise::new(
        Category::Legs.as_str(),
        "Развидение ног",
        "Разведение ног - позволяет тренировать внешние (отводящие) мышцы бёдер и ягодичные мышцы.",
        &format!("{IMAGE_IP}/unraveling_legs.jpg"),
    ));
    // LEGS END
    exercises.push_back(Exercise::new(
        Category::Chest.as_str(),
        "Жим лёжа",
        "Жим лёжа — базовое физическое упражнение со свободным весом.",
        &format!("{IMAGE_IP}/bench_press.jpg.webp"),
    ));

    exercises.push_back(
        Exercise::new(
            Category::Chest.as_str(),
            "Жим лёжа <30",
            "Смена угла наклона жима позволяет уделять внимание конкретной части груди, что способствует построению нужного рельефа",
            &format!("{IMAGE_IP}/bench_press30.jpg.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Chest.as_str(),
            "Жим лёжа гантелями",
            "Жим гантелей обеспечивает более широкий диапазон движения, так как каждая рука двигается независимо.",
            &format!("{IMAGE_IP}/bench_dmb.png")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Chest.as_str(),
            "Жим лёжа гантелями <30",
            "Жим гантелей обеспечивает более широкий диапазон движения, так как каждая рука двигается независимо. Нагрузка идет на вврхнюю часть груди",
            &format!("{IMAGE_IP}/bench_dmb30.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Chest.as_str(),
            "Жим узким хватом",
            "Жим узким хватом — это базовое упражнение на прокачку грудных мышц и трицепсов, при выполнении которого руки на грифе находятся на расстоянии около 40 см друг от друга.",
            &format!("{IMAGE_IP}/bench_close.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Chest.as_str(),
            "Пуловер",
            "Пуловер — изолированное упражнение для проработки верхней части спины, при этом незначительно нагружаются грудные мышцы и трицепсы.",
            &format!("{IMAGE_IP}/pullover.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Chest.as_str(),
            "Разводка в тренажёре",
            "В тренажере мышца всегда находится под нагрузкой. Тяжесть упражнения возрастает по мере сведения рук",
            &format!("{IMAGE_IP}/chest_together.jpeg")
        )
    );

    exercises.push_back(Exercise::new(
        Category::Chest.as_str(),
        "Разводка в кроссе",
        "Эффективное изолированное упражнение для развития мышц груди.",
        &format!("{IMAGE_IP}/chest_together_cross.jpeg"),
    ));

    exercises.push_back(Exercise::new(
        Category::Chest.as_str(),
        "Брусья",
        "Нагрузка приходится на трицепс и мышцы груди.",
        &format!("{IMAGE_IP}/bars.png"),
    ));

    exercises.push_back(
        Exercise::new(
            Category::Chest.as_str(),
            "Отжимания",
            "Отжима́ния — базовое физическое упражнение, выполняемое в планке и представляющее собой опускание-поднятие тела с помощью рук от пола, скамьи, стула, стола, стены и т. д.",
            &format!("{IMAGE_IP}/pushup.jpeg")
        )
    );
    //END CHEST

    exercises.push_back(
        Exercise::new(
            Category::Shoulders.as_str(),
            "Жим стоя",
            "Базовое упражнение с классической техникой (стоя со штангой) задействует много мышечных групп, но лучше всего прокачивает передние дельты, делая плечи визуально более объемными.",
            &format!("{IMAGE_IP}/shoulder_press.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Shoulders.as_str(),
            "Жим сидя с гантелями",
            "Жим гантелей сидя – классическое упражнение для проработки передней и средней дельтовидных мышц и в меньшей степени трицепса.",
            &format!("{IMAGE_IP}/seated_press.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Shoulders.as_str(),
            "Круг вращения",
            "Гало — прекрасное упражнение для улучшения подвижности ваших плеч, предложенное мастером русского гиревого спорта Стивом Максвелом.",
            &format!("{IMAGE_IP}/galo.jpeg")
        )
    );

    exercises.push_back(Exercise::new(
        Category::Shoulders.as_str(),
        "Отвидения",
        "За отведение руки в стороны от туловища отвечают надостная и дельтовидная мышцы.",
        &format!("{IMAGE_IP}/visions.png"),
    ));

    exercises.push_back(
        Exercise::new(
            Category::Shoulders.as_str(),
            "Батерфляй",
            "Тяга к подбородку — базовое упражнения для тренировки дельтовидных мышц (главным образом средней головки). В упражнении также участвует трапециевидные мышцы.",
            &format!("{IMAGE_IP}/pull_head.jpeg")
        )
    );

    exercises.push_back(Exercise::new(
        Category::Shoulders.as_str(),
        "Россомаха",
        "Упражнение на задний пучок. Скрещивание рук, затем разводка",
        &format!("{IMAGE_IP}/wolf.jpeg"),
    ));
    //END SHOULDERS

    exercises.push_back(
        Exercise::new(
            Category::Back.as_str(),
            "Тяга верхнего блока",
            "Это одно из основных упражнений в фитнесе и культуризме, которое действует на внешние края широчайшей мышцы, а также на верхнюю часть грудной мышцы, в меньшей мере на дельтовидные мышцы и бицепсы рук",
            &format!("{IMAGE_IP}/upper_block.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Back.as_str(),
            "Тяга нижнего блока",
            "Тяга нижнего блока отлично укрепляет мышцы средней и нижней части спины, и как следствие, улучшается осанка.",
            &format!("{IMAGE_IP}/down_block.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Back.as_str(),
            "Рычажная тяга",
            "Тяга вертикального рычажного тренажёра широким хватом качает верх широчайших мышц спины, так же включает в работу ромбовидные и трапециевидные мышцы спины.",
            &format!("{IMAGE_IP}/linkage.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Back.as_str(),
            "Тяга в наклоне",
            "Это упражнение включает в работу среднюю и нижнюю часть трапециевидных, широчайшие и подостные мышцы, разгибатели позвоночника и задние дельты",
            &format!("{IMAGE_IP}/bent_over.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Back.as_str(),
            "Тяга гантелей одной рукой",
            "Основная работающая мышечная группа при тяге гантели в наклоне одной рукой – широчайшие мышцы спины.",
            &format!("{IMAGE_IP}/one_arm_dumbbell.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Back.as_str(),
            "Подтягивания",
            "Это базовое физическое упражнение, развивающее группы мышц верхней части тела: широчайшие, бицепсы, брахиалис, грудные, верхний отдел позвоночника, мышцы брюшной стенки, предплечья.",
            &format!("{IMAGE_IP}/pullup.webp")
        )
    );

    exercises.push_back(Exercise::new(
        Category::Back.as_str(),
        "Гиперэкстензия",
        "Это изолированное физическое упражнение для развития выпрямителей спины.",
        &format!("{IMAGE_IP}/hyperextension.webp"),
    ));
    //END BACK

    exercises.push_back(
        Exercise::new(
            Category::Arms.as_str(),
            "Трицепс канатами",
            "Опуская кроссовер плавно вниз, зафиксируйте результат в самой нижней точке на пару секунд, Так как именно в нижней точке трицепс напряжен больше всего.",
            &format!("{IMAGE_IP}/ropes.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Arms.as_str(),
            "Французкий жим",
            "Французский жим – упражнение, при котором основная нагрузка приходится на разгибающие мышцы рук – трицепсы. Также в работу включаются мышцы груди, плеч и предплечья, но в меньшей степени.",
            &format!("{IMAGE_IP}/french.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Arms.as_str(),
            "Бицепс стоя",
            "Во время подъема штанги не двигайте локтями, держите их по бокам туловища и не сгибайте руки в запястьях.",
            &format!("{IMAGE_IP}/biceps_class.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            Category::Arms.as_str(),
            "Бицепс молотки",
            "Мóлот — изолирующее упражнение для развития мышц боковой части двуглавой мышцы плеча, плечевой и плечелучевой мышц.",
            &format!("{IMAGE_IP}/hummers.jpeg")
        )
    );

    exercises.push_back(Exercise::new(
        Category::Arms.as_str(),
        "Бицепс в кроссе",
        "Мóлот В этом упражнении сгибания рук на бицепс задействуют все пять компонентов бицепса.",
        &format!("{IMAGE_IP}/biceps_block.jpeg"),
    ));

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
