# TaskTrack – Full-Stack Architecture Overview

## What Angular Components Do
Angular components are reusable UI blocks that display content and handle user interactions. Each component manages its template (HTML), logic (TypeScript), and styling. Components are the building blocks of the entire Angular application—they display tasks, forms, and buttons to users.

## What Angular Services Do :
Angular services encapsulate business logic and manage communication with the backend. Services handle API requests, share data between components, and keep code DRY (Don't Repeat Yourself). Multiple components can use the same service to access backend data.

## How Angular HttpClient Works (Request → Response)
1. Component calls Service method
2. Service uses HttpClient.post/get/put/delete() to send HTTP request
3. Request travels to backend API with JSON data
4. Backend processes and returns JSON response
5. HttpClient delivers response as an Observable stream
6. Component receives data and updates UI

## What Rust Backend APIs Do
The Rust backend receives HTTP requests from the frontend and processes them. It:
- **Validates** incoming data (ensures title isn't empty, length limits)
- **Executes business logic** (checks permissions, calculations)
- **Queries the database** to fetch or store data
- **Returns JSON responses** with status codes (200, 201, 404, etc.)

## How PostgreSQL Fits In
PostgreSQL stores all persistent data (tasks, user info, etc.) in tables. The backend queries PostgreSQL using SQL, retrieves data, and sends it back to the frontend as JSON. The database guarantees data integrity and handles concurrent requests safely.

## Full Request Lifecycle (Frontend → Backend → Database → UI)

**Example: User creates a new task**

1. User types task title in Angular form and clicks "Create"
2. Component validates input (not empty, correct length)
3. Component calls TaskService.createTask(taskData)
4. Service uses HttpClient to POST request to /api/tasks endpoint
5. **HTTP Request sent:** POST /api/tasks with JSON body {"title": "Buy milk"}
6. Rust backend receives request at handler function
7. Handler validates data again (defense in depth)
8. Handler executes SQL: INSERT INTO tasks (title, ...) VALUES ('Buy milk', ...)
9. PostgreSQL creates new row, assigns ID, returns full task object
10. Rust handler formats response as JSON: {"status": "success", "data": {...}}
11. Handler sends HTTP 201 Created response with JSON body
12. HttpClient receives response, emits as Observable
13. Component subscribes, receives new task data
14. Component updates tasks array: this.tasks.push(newTask)
15. Angular detects change, re-renders template
16. **User sees new task in the list**

## System Architecture Diagram

```
┌─────────────────────────────────────────┐
│      ANGULAR FRONTEND (Browser)         │
│  Components ←→ Services ←→ HttpClient   │
└──────────────────┬──────────────────────┘
                   │ HTTP/JSON
┌──────────────────▼──────────────────────┐
│       RUST BACKEND API (Axum)           │
│  Routes ←→ Handlers ←→ Validation       │
└──────────────────┬──────────────────────┘
                   │ SQL Queries
┌──────────────────▼──────────────────────┐
│    POSTGRESQL DATABASE (Persistent)     │
│          Tasks Table with Data          │
└─────────────────────────────────────────┘
```

![Architecture Diagram](./architecture.png)

## Key Takeaway

Data flows in a cycle: **User Action → Component → Service → HTTP Request → Rust Handler → SQL Query → Database → Response JSON → Service → Component → UI Update**. Each layer has one responsibility, making the system maintainable and scalable.
