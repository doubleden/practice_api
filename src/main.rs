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
    exercises.push_back(
        Exercise::new(
            17,
            "chest",
            "Жим лёжа",
            "Жим лёжа — базовое физическое упражнение со свободным весом.",
            &format!("http://{IP}/images/bench_press.jpg.webp")
        )
    );

    exercises.push_back(
        Exercise::new(
            18,
            "chest",
            "Жим лёжа <30",
            "Смена угла наклона жима позволяет уделять внимание конкретной части груди, что способствует построению нужного рельефа",
            &format!("http://{IP}/images/bench_press30.jpg.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            19,
            "chest",
            "Жим лёжа гантелями",
            "Жим гантелей обеспечивает более широкий диапазон движения, так как каждая рука двигается независимо.",
            &format!("http://{IP}/images/bench_dmb.png")
        )
    );

    exercises.push_back(
        Exercise::new(
            20,
            "chest",
            "Жим лёжа гантелями <30",
            "Жим гантелей обеспечивает более широкий диапазон движения, так как каждая рука двигается независимо. Нагрузка идет на вврхнюю часть груди",
            &format!("http://{IP}/images/bench_dmb30.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            21,
            "chest",
            "Жим узким хватом",
            "Жим узким хватом — это базовое упражнение на прокачку грудных мышц и трицепсов, при выполнении которого руки на грифе находятся на расстоянии около 40 см друг от друга.",
            &format!("http://{IP}/images/bench_close.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            22,
            "chest",
            "Пуловер",
            "Пуловер — изолированное упражнение для проработки верхней части спины, при этом незначительно нагружаются грудные мышцы и трицепсы.",
            &format!("http://{IP}/images/pullover.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            23,
            "chest",
            "Разводка в тренажёре",
            "В тренажере мышца всегда находится под нагрузкой. Тяжесть упражнения возрастает по мере сведения рук",
            &format!("http://{IP}/images/chest_together.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            24,
            "chest",
            "Разводка в кроссе",
            "Эффективное изолированное упражнение для развития мышц груди.",
            &format!("http://{IP}/images/chest_together_cross.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            25,
            "chest",
            "Брусья",
            "Нагрузка приходится на трицепс и мышцы груди.",
            &format!("http://{IP}/images/bars.png")
        )
    );

    exercises.push_back(
        Exercise::new(
            26,
            "chest",
            "Отжимания",
            "Отжима́ния — базовое физическое упражнение, выполняемое в планке и представляющее собой опускание-поднятие тела с помощью рук от пола, скамьи, стула, стола, стены и т. д.",
            &format!("http://{IP}/images/pushup.jpeg")
        )
    );
    //END CHEST

    exercises.push_back(
        Exercise::new(
            27,
            "shoulders",
            "Жим стоя",
            "Базовое упражнение с классической техникой (стоя со штангой) задействует много мышечных групп, но лучше всего прокачивает передние дельты, делая плечи визуально более объемными.",
            &format!("http://{IP}/images/shoulder_press.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            28,
            "shoulders",
            "Жим сидя с гантелями",
            "Жим гантелей сидя – классическое упражнение для проработки передней и средней дельтовидных мышц и в меньшей степени трицепса.",
            &format!("http://{IP}/images/seated_press.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            29,
            "shoulders",
            "Круг вращения",
            "Гало — прекрасное упражнение для улучшения подвижности ваших плеч, предложенное мастером русского гиревого спорта Стивом Максвелом.",
            &format!("http://{IP}/images/galo.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            30,
            "shoulders",
            "Отвидения",
            "За отведение руки в стороны от туловища отвечают надостная и дельтовидная мышцы.",
            &format!("http://{IP}/images/visions.png")
        )
    );

    exercises.push_back(
        Exercise::new(
            31,
            "shoulders",
            "Батерфляй",
            "Тяга к подбородку — базовое упражнения для тренировки дельтовидных мышц (главным образом средней головки). В упражнении также участвует трапециевидные мышцы.",
            &format!("http://{IP}/images/pull_head.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            32,
            "shoulders",
            "Россомаха",
            "Упражнение на задний пучок. Скрещивание рук, затем разводка",
            &format!("http://{IP}/images/wolf.jpeg")
        )
    );
    //END SHOULDERS

    exercises.push_back(
        Exercise::new(
            33,
            "back",
            "Тяга верхнего блока",
            "Это одно из основных упражнений в фитнесе и культуризме, которое действует на внешние края широчайшей мышцы, а также на верхнюю часть грудной мышцы, в меньшей мере на дельтовидные мышцы и бицепсы рук",
            &format!("http://{IP}/images/upper_block.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            34,
            "back",
            "Тяга нижнего блока",
            "Тяга нижнего блока отлично укрепляет мышцы средней и нижней части спины, и как следствие, улучшается осанка.",
            &format!("http://{IP}/images/down_block.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            35,
            "back",
            "Рычажная тяга",
            "Тяга вертикального рычажного тренажёра широким хватом качает верх широчайших мышц спины, так же включает в работу ромбовидные и трапециевидные мышцы спины.",
            &format!("http://{IP}/images/linkage.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            36,
            "back",
            "Тяга в наклоне",
            "Это упражнение включает в работу среднюю и нижнюю часть трапециевидных, широчайшие и подостные мышцы, разгибатели позвоночника и задние дельты",
            &format!("http://{IP}/images/bent_over.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            37,
            "back",
            "Тяга гантелей одной рукой",
            "Основная работающая мышечная группа при тяге гантели в наклоне одной рукой – широчайшие мышцы спины.",
            &format!("http://{IP}/images/one_arm_dumbbell.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            38,
            "back",
            "Подтягивания",
            "Это базовое физическое упражнение, развивающее группы мышц верхней части тела: широчайшие, бицепсы, брахиалис, грудные, верхний отдел позвоночника, мышцы брюшной стенки, предплечья.",
            &format!("http://{IP}/images/pullup.webp")
        )
    );

    exercises.push_back(
        Exercise::new(
            39,
            "back",
            "Гиперэкстензия",
            "Это изолированное физическое упражнение для развития выпрямителей спины.",
            &format!("http://{IP}/images/hyperextension.webp")
        )
    );
    //END BACK

    exercises.push_back(
        Exercise::new(
            40,
            "arms",
            "Трицепс канатами",
            "Опуская кроссовер плавно вниз, зафиксируйте результат в самой нижней точке на пару секунд, Так как именно в нижней точке трицепс напряжен больше всего.",
            &format!("http://{IP}/images/ropes.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            41,
            "arms",
            "Французкий жим",
            "Французский жим – упражнение, при котором основная нагрузка приходится на разгибающие мышцы рук – трицепсы. Также в работу включаются мышцы груди, плеч и предплечья, но в меньшей степени.",
            &format!("http://{IP}/images/french.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            42,
            "arms",
            "Бицепс стоя",
            "Во время подъема штанги не двигайте локтями, держите их по бокам туловища и не сгибайте руки в запястьях.",
            &format!("http://{IP}/images/biceps_class.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            43,
            "arms",
            "Бицепс молотки",
            "Мóлот — изолирующее упражнение для развития мышц боковой части двуглавой мышцы плеча, плечевой и плечелучевой мышц.",
            &format!("http://{IP}/images/hummers.jpeg")
        )
    );

    exercises.push_back(
        Exercise::new(
            44,
            "arms",
            "Бицепс в кроссе",
            "Мóлот В этом упражнении сгибания рук на бицепс задействуют все пять компонентов бицепса.",
            &format!("http://{IP}/images/biceps_block.jpeg")
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

