## 🚀 Why Rust is Becoming a Top Backend Choice

Rust is increasingly used in performance-critical backend systems because it provides:

- Memory Safety – Prevents null pointer crashes and memory leaks.

- High Performance – Comparable to C/C++.

- Zero-Cost Abstractions – High-level code without runtime overhead.

- Fearless Concurrency – Safe multi-threading without race conditions.

- Strong Type System – Errors are caught at compile time.

Many companies use Rust in production, including:

- Amazon

- Cloudflare

- Discord

- Dropbox

- Figma

## 🧩 How a Rust API Works

A Rust backend receives HTTP requests from a frontend (such as Angular), processes them through structured handlers, and returns JSON responses.

The core building blocks are:

### 1️⃣ Routes

Routes define which handler function should execute for a specific endpoint.

Example:

```rust
.route("/tasks", web::post().to(create_task))
.route("/tasks", web::get().to(get_tasks))
```

- POST /tasks → calls create_task

- GET /tasks → calls get_tasks

Routes act as a mapping between URL paths and backend logic.

### 2️⃣ Handlers

Handlers contain the business logic for each endpoint.

Example:

```rust
async fn create_task(
    pool: web::Data<PgPool>,
    task: web::Json<CreateTask>,
) -> impl Responder
```

The handler:

- Receives structured input

- Interacts with the database

- Returns a JSON response

Handlers are asynchronous (async) so they can serve multiple users efficiently without blocking.

### 3️⃣ Structs & Type Safety

Rust uses strongly typed structs to define request data.

Example:

```rust
#[derive(Deserialize)]
struct CreateTask {
    title: String,
}
```

This ensures:

- The request body must contain a title

- The data type must match (String)

- Invalid data is rejected automatically

Rust validates structure before executing logic, reducing runtime errors.

## 📌 Example Endpoint
### POST /tasks

Request Body:
```json
{
  "title": "Finish Rust Assignment"
}
```

Flow:

1. Frontend sends JSON request.

2. Rust route matches /tasks.

3. create_task handler receives CreateTask struct.

4. SQL query inserts data into PostgreSQL.

5. Rust returns JSON response:

```json
"Task created successfully"
```

## 🔄 Full Request–Response Flow

Below is the complete architecture flow of this system:

```java
Angular Component (User submits form)
        ↓
Angular Service (HTTP POST)
        ↓
Rust Route (/tasks)
        ↓
Rust Handler (Validates Request)
        ↓
SQL Query → PostgreSQL
        ↓
JSON Response
        ↓
Angular UI Updates
```

## 🏗 Architecture Diagram
```java
Frontend (Angular)
        ↓
Rust Backend (Actix-Web)
        ↓
PostgreSQL Database
```

This represents a typical three-tier architecture:

- Presentation Layer (Frontend)

- Application Layer (Rust Backend)

- Data Layer (PostgreSQL)

## 🧠 Why Type-Safety Helps API Development

Rust’s type system ensures correctness before the application runs.

Benefits:

- Prevents invalid data from reaching the database

- Eliminates null pointer crashes

- Catches errors at compile time

- Reduces runtime bugs

- Improves maintainability

Unlike dynamically typed backends, Rust guarantees that request formats match expected structures.

This makes APIs more predictable, secure, and reliable under heavy load.