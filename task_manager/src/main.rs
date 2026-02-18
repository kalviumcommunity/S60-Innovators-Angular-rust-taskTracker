// #[actix_web::main]
// async fn main() -> std::io::Result<()> {

//     let database_url = "postgres://postgres:1234@localhost/task_manager";

//     let pool = PgPool::connect(database_url)
//         .await
//         .expect("Failed to connect to database");

//     HttpServer::new(move || {
//         App::new()
//             .app_data(web::Data::new(pool.clone()))
//             .route("/tasks", web::post().to(create_task))
//             .route("/tasks", web::get().to(get_tasks))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use actix_cors::Cors;

#[derive(Serialize)]
struct Task {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
}

async fn create_task(
    pool: web::Data<PgPool>,
    task: web::Json<CreateTask>,
) -> impl Responder {

    let result = sqlx::query(
        "INSERT INTO tasks (title) VALUES ($1)"
    )
    .bind(&task.title)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Task created successfully"),
        Err(e) => {
            println!("Insert error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_tasks(pool: web::Data<PgPool>) -> impl Responder {

    let rows = sqlx::query(
        "SELECT id, title, completed FROM tasks"
    )
    .fetch_all(pool.get_ref())
    .await;

    match rows {
        Ok(data) => {
            let tasks: Vec<Task> = data.into_iter().map(|row| {
                Task {
                    id: row.get("id"),
                    title: row.get("title"),
                    completed: row.get("completed"),
                }
            }).collect();

            HttpResponse::Ok().json(tasks)
        }
        Err(e) => {
            println!("Fetch error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let database_url = "postgres://postgres:1234@localhost/task_manager";
    // replace 1234 with your actual password

    let pool = PgPool::connect(database_url)
        .await
        .expect("Failed to connect to database");

    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(web::Data::new(pool.clone()))
    //         .route("/tasks", web::post().to(create_task))
    //         .route("/tasks", web::get().to(get_tasks))
    // })
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
            .max_age(3600);
    
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .route("/tasks", web::post().to(create_task))
            .route("/tasks", web::get().to(get_tasks))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
