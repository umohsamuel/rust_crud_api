use crate::db;
use crate::jwt::create_access_token;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
pub struct UpdateTaskRequest {
    title: String,
    completed: bool,
}

#[get("/tasks")]
pub async fn get_tasks(db: web::Data<Mutex<db::Database>>) -> impl Responder {
    let db = db.lock().unwrap();
    let tasks = db.get_tasks().unwrap();
    HttpResponse::Ok().json(tasks)
}

#[post("/tasks")]
pub async fn create_task(
    req: web::Json<CreateTaskRequest>,
    db: web::Data<Mutex<db::Database>>,
) -> impl Responder {
    let db = db.lock().unwrap();
    let task = db.create_task(&req.title, req.completed).unwrap();
    HttpResponse::Created().json(task)
}

#[put("/tasks/{id}")]
pub async fn update_task(
    path: web::Path<Uuid>,
    req: web::Json<UpdateTaskRequest>,
    db: web::Data<Mutex<db::Database>>,
) -> impl Responder {
    let id = path.into_inner();
    let req_data = req.into_inner();
    let db = db.lock().unwrap();

    match db.update_task(id, req_data.completed, &req_data.title) {
        Ok(updated_task) => HttpResponse::Ok().json(updated_task),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/tasks/{id}")]
pub async fn delete_task(
    path: web::Path<Uuid>,
    db: web::Data<Mutex<db::Database>>,
) -> impl Responder {
    let id = path.into_inner();
    let db = db.lock().unwrap();
    match db.delete_task(id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[post("/login")]
pub async fn login(
    req: web::Json<LoginRequest>,
    data: web::Data<Mutex<db::Database>>,
) -> impl Responder {
    let db = data.lock().unwrap();

    let user = match db.get_user_by_username(&req.username) {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid username or password"),
    };

    // Verify the password.
    // (You should use a proper password hashing library here; this example does a plain string compare.)
    if req.password != user.password_hash {
        return HttpResponse::Unauthorized().body("Invalid username or password");
    }

    // Instead of using an environment variable for JWT_SECRET, you can load it from the settings table.
    let secret = match db.get_setting("JWT_SECRET").unwrap() {
        Some(s) => s,
        None => return HttpResponse::InternalServerError().body("JWT secret not configured"),
    };

    let secret_bytes = secret.as_bytes();
    let access_token = match crate::jwt::create_access_token(&req.username, secret_bytes) {
        Ok(token) => token,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Token error: {}", e)),
    };

    let refresh_token = match crate::jwt::create_refresh_token(&req.username, secret_bytes) {
        Ok(token) => token,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Token error: {}", e)),
    };

    HttpResponse::Ok().json(json!({
        "status": "success",
        "data": {
            "access_token": access_token,
            "refresh_token": refresh_token,
        }}
    ))
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: Uuid,
    pub username: String,
    pub created_at: Option<NaiveDateTime>,
}

#[post("/register")]
pub async fn register(
    req: web::Json<RegisterRequest>,
    data: web::Data<Mutex<db::Database>>,
) -> impl Responder {
    let db = data.lock().unwrap();

    // Check if user already exists.
    if db.get_user_by_username(&req.username).is_ok() {
        return HttpResponse::BadRequest().body("User already exists");
    }

    // For demonstration, we simply store the raw password.
    // In production, hash the password using a secure algorithm.
    let user = match db.create_user(&req.username, &req.password) {
        Ok(user) => user,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    };

    HttpResponse::Created().json(RegisterResponse {
        id: user.id,
        username: user.username,
        created_at: user.created_at,
    })
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
}

#[post("/refresh")]
pub async fn refresh(
    req: web::Json<RefreshRequest>,
    data: web::Data<Mutex<db::Database>>,
) -> impl Responder {
    let db = data.lock().unwrap();
    let yo = db.get_setting("JWT_SECRET").unwrap().unwrap();
    let secret = yo.as_bytes();
    match verify_token(&req.refresh_token, secret) {
        Ok(token_data) => {
            let user_id = token_data.claims.sub;
            match create_access_token(&user_id, secret) {
                Ok(new_access) => HttpResponse::Ok().json(RefreshResponse {
                    access_token: new_access,
                }),
                Err(e) => HttpResponse::InternalServerError().body(format!("Token error: {}", e)),
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid refresh token"),
    }
}

use crate::jwt::verify_token;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct AuthMiddleware {
    pub jwt_secret: String,
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareInner {
            service: Rc::new(service),
            jwt_secret: self.jwt_secret.clone(),
        })
    }
}

pub struct AuthMiddlewareInner<S> {
    service: Rc<S>,
    jwt_secret: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let jwt_secret = self.jwt_secret.clone();
        // Check for Authorization header.
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|hv| hv.to_str().ok());
        if let Some(header_value) = auth_header {
            if header_value.starts_with("Bearer ") {
                let token = &header_value[7..];
                if verify_token(token, jwt_secret.as_bytes()).is_ok() {
                    // Optionally, you can store claims in extensions:
                    // req.extensions_mut().insert(claims);
                    let fut = self.service.call(req);
                    return Box::pin(async move { fut.await });
                }
            }
        }
        let fut = async { Err(actix_web::error::ErrorUnauthorized("Unauthorized")) };
        Box::pin(fut)
    }
}
