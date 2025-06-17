# 📝 Rust Task Manager API

A lightweight and performant **CRUD API** built with **Rust**, using `actix-web`, `rusqlite`, and `jsonwebtoken`.  
This project provides a simple task manager backend with JWT-based authentication, SQLite storage, and full RESTful API support.

🔗 [GitHub Repo](https://github.com/umohsamuel/rust_crud_api)

---

## 🚀 Features

- ✅ Full CRUD operations for task management
- 🔐 JWT authentication (register & login)
- 💾 SQLite database with `rusqlite` (bundled)
- 🌐 CORS support for safe cross-origin API access
- 🧩 UUID-based unique task & user IDs
- ⏱️ Timestamp support using `chrono`
- ⚙️ Config via `.env` using `dotenv`
- 🧪 Easy to test with Postman or cURL

---

## 🛠️ Tech Stack

| Tool         | Purpose                              |
|--------------|--------------------------------------|
| **Rust**     | Language                             |
| **actix-web**| Web framework                        |
| **rusqlite** | SQLite database integration          |
| **serde**    | JSON serialization/deserialization   |
| **jsonwebtoken** | JWT token auth                   |
| **dotenv**   | Environment variable loading         |
| **uuid**     | Unique ID generation                 |
| **chrono**   | Timestamps and date-time handling    |
| **futures**  | Async compatibility                  |

---

## 📁 Project Structure

```bash
rust_crud_api/
├── src/
│ ├── main.rs # App entry point
│ ├── models.rs # Data models (Task, User)
│ ├── handlers.rs # Route logic and endpoints
│ ├── auth.rs # JWT encoding/decoding helpers
│ ├── db.rs # Database initialization and queries
│ └── utils.rs # Utility functions (e.g., token handling)
├── .env # Environment variables (JWT secret, DB path)
├── Cargo.toml # Project dependencies and metadata
```

---

## 📦 Installation & Setup

> **Pre-requisite:** [Rust & Cargo](https://www.rust-lang.org/tools/install)

### 1. Clone the repo

```bash
git clone https://github.com/umohsamuel/rust_crud_api.git
cd rust_crud_api
```

### 2. Create your .env file
```bash
DATABASE_URL=task_manager.db
JWT_SECRET=your_jwt_secret_here
```

### 3. Run the server
```bash
cargo run
Server starts at http://127.0.0.1:8080
```

## 🔐 Authentication
This API uses JWT-based authentication. After registering and logging in, include the token in the request header:

```bash
Authorization: Bearer <your_token>
```

## 🧪 API Endpoints

| Method | Endpoint         | Description         |
| ------ | ---------------- | ------------------- |
| POST   | `/auth/register` | Register new user   |
| POST   | `/auth/login`    | Login and get token |

## 🔒 Protected Routes (require JWT)

| Method | Endpoint      | Description       |
| ------ | ------------- | ----------------- |
| GET    | `/tasks`      | Get all tasks     |
| GET    | `/tasks/{id}` | Get a single task |
| POST   | `/tasks`      | Create a new task |
| PUT    | `/tasks/{id}` | Update a task     |
| DELETE | `/tasks/{id}` | Delete a task     |


## 📬 Sample Request (cURL)

```bash
# Register
curl -X POST http://localhost:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"samuel","password":"securepass"}'

# Login
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"samuel","password":"securepass"}'

# Create Task (requires JWT)
curl -X POST http://localhost:8080/tasks \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"title":"Build API","description":"Learn actix-web with SQLite"}'
```
## 🧾 Example .env

```bash
DATABASE_URL=task_manager.db
JWT_SECRET=supersecretkey123
```

## ✅ To-Do / Improvements
 Token expiration and refresh

 Password hashing (e.g., argon2)

 Pagination for task list

 Swagger/OpenAPI documentation

 Dockerfile for containerization


## 🧪 Running Tests (optional)
You can write tests using Rust’s built-in test framework:

```bash
cargo test
```
