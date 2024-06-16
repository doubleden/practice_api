use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files as afs;
use uuid::Uuid;
const IP: &str = "0.0.0.0:8080";
const IMAGE_IP: &str = "192.168.18.234:8080";
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
            "legs",
            "Жим ног",
            "Упражнение для прокачки ног, которое выполняется в специальном тренажёре, горизонтальном или вертикальном (наклонном)",
            &format!("http://{IMAGE_IP}/images/leg_press.png")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Присяд со штангой",
            "Положите штангу на плечи, выставьте одну ногу вперед, вторую отведите назад. На вдохе опустите таз до параллели бедра с полом. На выдохе вернитесь в исходное положение. Упражнение эффективно нагружает мышцы бедер и ягодиц.",
            &format!("http://{IMAGE_IP}/images/squat_with_barbell.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Присяд с гантелей",
            "Альтернатива упражнения - присяд со штангой",
            &format!("http://{IMAGE_IP}/images/squat_with_dumbbells.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Выпады назад",
            "Упражнение выпадов назад 一 это упражнение, которое направлено на развитие мышц ног, таких как ягодицы, бедра и икры.",
            &format!("http://{IMAGE_IP}/images/back_lunges.jpg.webp")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Болгарский выпад",
            "упражнение для тренировки нижней части тела, особенно ягодичных и бедренных мышц.",
            &format!("http://{IMAGE_IP}/images/lunges_with_bench.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Выпады вперед",
            "Классика",
            &format!("http://{IMAGE_IP}/images/lunge_steps.jpeg.webp")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Сгибание ног",
            "Сгибание ног в тренажёре — упражнение для развития силы и наращивания объёма мышц на задней стороне бедра.",
            &format!("http://{IMAGE_IP}/images/leg_curl.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Разгибания ног",
            "Разгибания ног в тренажёре — упражнение для развития силы и наращивания объёма мышц на передней стороне бедра.",
            &format!("http://{IMAGE_IP}/images/leg_extensions.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Тяга классика",
            "Большая часть нагрузки приходится на спину — разгибатели и трапециевидные мышцы.",
            &format!("http://{IMAGE_IP}/images/traction_classic.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Тяга сумо",
            "Приседания сумо — это комплексное упражнение, при выполнении которого работает много мышц. Основные мышцы: квадрицепсы, ягодичные мышцы, бицепс бедра, икры и внутренняя поверхность бедер, и другие, более мелкие, внутренние мышцы бедра.",
            &format!("http://{IMAGE_IP}/images/sumo_deadlift.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Пистолетик",
            "«Пистолетик» — приседание на одной ноге без опоры, относится к силовым упражнениям. Его биомеханика аналогична обычным приседаниям. Но выполнять «пистолетик» значительно тяжелее, так как рабочая нога получает непривычную нагрузку.",
            &format!("http://{IMAGE_IP}/images/gun_squat.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Броски гири",
            "Броски гири, так или иначе, приводит в тонус все мышцы, но в большей степени участвуют мышцы бедра, ягодиц, спины и живота",
            &format!("http://{IMAGE_IP}/images/throws_weights.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Махи лёжа на боку",
            "Такие махи направлены на укрепление внутренней поверхности бедра и нижней части брюшной мускулатуры.",
            &format!("http://{IMAGE_IP}/images/swing_lying.png")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Ягодичный мостик",
            "Ягодичный мостик – это наилучшее упражнения для проработки ягодичной мышцы.",
            &format!("http://{IMAGE_IP}/images/gluteal_bridge.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Свидение ног",
            "Сведение ног – востребованное физическое упражнение для укрепления нижнего пояса.",
            &format!("http://{IMAGE_IP}/images/date_legs.jpg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "legs",
            "Развидение ног",
            "Разведение ног - позволяет тренировать внешние (отводящие) мышцы бёдер и ягодичные мышцы.",
            &format!("http://{IMAGE_IP}/images/unraveling_legs.jpg")
        )
    );
    // LEGS END
    exercises.push_back(
        Exercise::new(
            "chest",
            "Жим лёжа",
            "Жим лёжа — базовое физическое упражнение со свободным весом.",
            &format!("http://{IMAGE_IP}/images/bench_press.jpg.webp")
        )
    );

    exercises.push_back(
        Exercise::new(
            "chest",
            "Жим лёжа <30",
            "Смена угла наклона жима позволяет уделять внимание конкретной части груди, что способствует построению нужного рельефа",
            &format!("http://{IMAGE_IP}/images/bench_press30.jpg.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "chest",
            "Жим лёжа гантелями",
            "Жим гантелей обеспечивает более широкий диапазон движения, так как каждая рука двигается независимо.",
            &format!("http://{IMAGE_IP}/images/bench_dmb.png")
        )
    );

    exercises.push_back(
        Exercise::new(
            "chest",
            "Жим лёжа гантелями <30",
            "Жим гантелей обеспечивает более широкий диапазон движения, так как каждая рука двигается независимо. Нагрузка идет на вврхнюю часть груди",
            &format!("http://{IMAGE_IP}/images/bench_dmb30.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "chest",
            "Жим узким хватом",
            "Жим узким хватом — это базовое упражнение на прокачку грудных мышц и трицепсов, при выполнении которого руки на грифе находятся на расстоянии около 40 см друг от друга.",
            &format!("http://{IMAGE_IP}/images/bench_close.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "chest",
            "Пуловер",
            "Пуловер — изолированное упражнение для проработки верхней части спины, при этом незначительно нагружаются грудные мышцы и трицепсы.",
            &format!("http://{IMAGE_IP}/images/pullover.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "chest",
            "Разводка в тренажёре",
            "В тренажере мышца всегда находится под нагрузкой. Тяжесть упражнения возрастает по мере сведения рук",
            &format!("http://{IMAGE_IP}/images/chest_together.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "chest",
            "Разводка в кроссе",
            "Эффективное изолированное упражнение для развития мышц груди.",
            &format!("http://{IMAGE_IP}/images/chest_together_cross.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "chest",
            "Брусья",
            "Нагрузка приходится на трицепс и мышцы груди.",
            &format!("http://{IMAGE_IP}/images/bars.png")
        )
    );

    exercises.push_back(
        Exercise::new(
            "chest",
            "Отжимания",
            "Отжима́ния — базовое физическое упражнение, выполняемое в планке и представляющее собой опускание-поднятие тела с помощью рук от пола, скамьи, стула, стола, стены и т. д.",
            &format!("http://{IMAGE_IP}/images/pushup.jpeg")
        )
    );
    //END CHEST

    exercises.push_back(
        Exercise::new(
            "shoulders",
            "Жим стоя",
            "Базовое упражнение с классической техникой (стоя со штангой) задействует много мышечных групп, но лучше всего прокачивает передние дельты, делая плечи визуально более объемными.",
            &format!("http://{IMAGE_IP}/images/shoulder_press.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "shoulders",
            "Жим сидя с гантелями",
            "Жим гантелей сидя – классическое упражнение для проработки передней и средней дельтовидных мышц и в меньшей степени трицепса.",
            &format!("http://{IMAGE_IP}/images/seated_press.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "shoulders",
            "Круг вращения",
            "Гало — прекрасное упражнение для улучшения подвижности ваших плеч, предложенное мастером русского гиревого спорта Стивом Максвелом.",
            &format!("http://{IMAGE_IP}/images/galo.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "shoulders",
            "Отвидения",
            "За отведение руки в стороны от туловища отвечают надостная и дельтовидная мышцы.",
            &format!("http://{IMAGE_IP}/images/visions.png")
        )
    );

    exercises.push_back(
        Exercise::new(
            "shoulders",
            "Батерфляй",
            "Тяга к подбородку — базовое упражнения для тренировки дельтовидных мышц (главным образом средней головки). В упражнении также участвует трапециевидные мышцы.",
            &format!("http://{IMAGE_IP}/images/pull_head.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "shoulders",
            "Россомаха",
            "Упражнение на задний пучок. Скрещивание рук, затем разводка",
            &format!("http://{IMAGE_IP}/images/wolf.jpeg")
        )
    );
    //END SHOULDERS

    exercises.push_back(
        Exercise::new(
            "back",
            "Тяга верхнего блока",
            "Это одно из основных упражнений в фитнесе и культуризме, которое действует на внешние края широчайшей мышцы, а также на верхнюю часть грудной мышцы, в меньшей мере на дельтовидные мышцы и бицепсы рук",
            &format!("http://{IMAGE_IP}/images/upper_block.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "back",
            "Тяга нижнего блока",
            "Тяга нижнего блока отлично укрепляет мышцы средней и нижней части спины, и как следствие, улучшается осанка.",
            &format!("http://{IMAGE_IP}/images/down_block.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "back",
            "Рычажная тяга",
            "Тяга вертикального рычажного тренажёра широким хватом качает верх широчайших мышц спины, так же включает в работу ромбовидные и трапециевидные мышцы спины.",
            &format!("http://{IMAGE_IP}/images/linkage.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "back",
            "Тяга в наклоне",
            "Это упражнение включает в работу среднюю и нижнюю часть трапециевидных, широчайшие и подостные мышцы, разгибатели позвоночника и задние дельты",
            &format!("http://{IMAGE_IP}/images/bent_over.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "back",
            "Тяга гантелей одной рукой",
            "Основная работающая мышечная группа при тяге гантели в наклоне одной рукой – широчайшие мышцы спины.",
            &format!("http://{IMAGE_IP}/images/one_arm_dumbbell.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "back",
            "Подтягивания",
            "Это базовое физическое упражнение, развивающее группы мышц верхней части тела: широчайшие, бицепсы, брахиалис, грудные, верхний отдел позвоночника, мышцы брюшной стенки, предплечья.",
            &format!("http://{IMAGE_IP}/images/pullup.webp")
        )
    );

    exercises.push_back(
        Exercise::new(
            "back",
            "Гиперэкстензия",
            "Это изолированное физическое упражнение для развития выпрямителей спины.",
            &format!("http://{IMAGE_IP}/images/hyperextension.webp")
        )
    );
    //END BACK

    exercises.push_back(
        Exercise::new(
            "arms",
            "Трицепс канатами",
            "Опуская кроссовер плавно вниз, зафиксируйте результат в самой нижней точке на пару секунд, Так как именно в нижней точке трицепс напряжен больше всего.",
            &format!("http://{IMAGE_IP}/images/ropes.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "arms",
            "Французкий жим",
            "Французский жим – упражнение, при котором основная нагрузка приходится на разгибающие мышцы рук – трицепсы. Также в работу включаются мышцы груди, плеч и предплечья, но в меньшей степени.",
            &format!("http://{IMAGE_IP}/images/french.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "arms",
            "Бицепс стоя",
            "Во время подъема штанги не двигайте локтями, держите их по бокам туловища и не сгибайте руки в запястьях.",
            &format!("http://{IMAGE_IP}/images/biceps_class.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "arms",
            "Бицепс молотки",
            "Мóлот — изолирующее упражнение для развития мышц боковой части двуглавой мышцы плеча, плечевой и плечелучевой мышц.",
            &format!("http://{IMAGE_IP}/images/hummers.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            "arms",
            "Бицепс в кроссе",
            "Мóлот В этом упражнении сгибания рук на бицепс задействуют все пять компонентов бицепса.",
            &format!("http://{IMAGE_IP}/images/biceps_block.jpeg")
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

