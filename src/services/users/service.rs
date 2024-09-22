use actix_web::{post, get, web, HttpResponse, Responder, put, delete};
use actix_web::web::service;
use crate::{services, AppState};
use bcrypt::{DEFAULT_COST, hash};
use sqlx::{Row};
use crate::services::users::models::{AllUsers, CreatedUser, RegisterUser};

#[get("/users")]
async fn all_users(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query("SELECT id, name, email, password FROM users")
        .fetch_all(&app_state.postgres_cliet)
        .await;

    match result {
        Ok(rows) => {
            let users: Vec<AllUsers> = rows.iter().map(|row| {
                AllUsers {
                    id: row.get("id"),
                    name: row.get("name"),
                    email: row.get("email"),
                    password: row.get("password"),
                }
            }).collect();

            HttpResponse::Ok().json(users)
        }
        Err(err) => {
            eprintln!("Failed to fetch users: {}", err);
            HttpResponse::InternalServerError().body("Error trying to get all users.")
        }
    }
}

#[post("/users")]
async fn create_user(app_state: web::Data<AppState>, user: web::Json<RegisterUser>) -> impl Responder {
    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(hp) => hp,
        Err(_) => return HttpResponse::InternalServerError().body("Error trying to hash password"),
    };

    let result = sqlx::query(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email, password"
    )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&hashed_password)
        .fetch_one(&app_state.postgres_cliet)
        .await;

    match result {
        Ok(row) => {
            let created_user = CreatedUser {
                name: row.get("name"),
                email: row.get("email"),
            };
            HttpResponse::Created().json(created_user)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error inserting user into database"),
    }
}

#[put("/users/{id}")]
async fn update_user(app_state: web::Data<AppState>, user: web::Json<RegisterUser>, id: web::Path<i32>) -> impl Responder {
    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(hp) => hp,
        Err(_) => return HttpResponse::InternalServerError().body("Error trying to hash password")
    };

    let result = sqlx::query(
        "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4 RETURNING id, name, email"
    )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&hashed_password)
        .bind(*id)
        .fetch_one(&app_state.postgres_cliet)
        .await;

    match result {
        Ok(row) => {
            let updated_user = AllUsers {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
                password: hashed_password,
            };
            HttpResponse::Ok().json(updated_user)
        }
        Err(err) => {
            eprintln!("Error updating user: {}", err); // Log do erro
            HttpResponse::InternalServerError().body("Error updating user in the database")
        }
    }
}

#[delete("/users/{id}")]
async fn delete_user(app_state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(*id)
        .execute(&app_state.postgres_cliet)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                HttpResponse::Ok().body("User deleted successfully")
            } else {
                HttpResponse::NotFound().body("User not found")
            }
        }
        Err(err) => {
            eprintln!("Error deleting user: {}", err); // Log do erro
            HttpResponse::InternalServerError().body("Error deleting user from the database")
        }
    }
}

pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(all_users)
        .service(create_user)
        .service(update_user)
        .service(delete_user);
}
