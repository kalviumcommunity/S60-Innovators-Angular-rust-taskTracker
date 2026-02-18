# Angular Frontend – Task Manager UI

## Overview

This Angular application serves as the frontend for the Task Manager system.  
It communicates with a Rust backend built using Actix-Web and PostgreSQL.

The purpose of this task is to demonstrate:

- Understanding of Angular components
- Use of services for API communication
- Frontend to backend interaction
- Modular architecture design
- Clear separation of concerns

---

## 1. Explanation of Angular Components

In Angular, a component controls a part of the user interface.

Each component contains:
- A template (HTML)
- Logic (TypeScript)
- Styles (CSS)

In this project, the main component is the **TasksComponent**.

The TasksComponent is responsible for:

- Displaying the list of tasks
- Handling user input
- Triggering API calls through the service
- Automatically updating the UI when data changes

The component does not directly communicate with the backend.  
Instead, it delegates API communication to a service.

This keeps the UI logic clean and focused.

---

## 2. Explanation of Angular Services

Services in Angular are used to handle:

- API communication
- Business logic
- Shared data management

In this project, the **TaskService** is responsible for:

- Sending HTTP GET requests to fetch tasks
- Sending HTTP POST requests to create tasks
- Returning observable responses to the component

The component subscribes to the service responses and updates its internal state.

This separation improves maintainability and reusability.

---

## 3. Simple UI Feature Implemented

The implemented feature is:

### Add and View Tasks

The UI allows users to:

- View all tasks stored in the database
- Add a new task using an input field
- Automatically refresh the task list after insertion

When a user adds a task:

1. The component captures user input.
2. The service sends a POST request to the Rust backend.
3. The backend stores the task in PostgreSQL.
4. The component reloads the updated list.
5. The UI updates automatically.

This demonstrates full frontend-backend integration.

---

## 4. Angular → Rust Interaction Diagram

User Action (Click Button)\
↓\
Angular Component\
↓\
Task Service\
↓\
HTTP Request (/tasks)\
↓\
Rust Route\
↓\
Rust Handler\
↓\
PostgreSQL Database\
↓\
JSON Response\
↓\
Angular Component Updates UI


This follows a three-layer architecture:

- Presentation Layer (Angular)
- Application Layer (Rust Backend)
- Data Layer (PostgreSQL)

---

## 5. Case Study

Scenario: A user clicks “View Tasks”.

Step-by-step flow:

1. The TasksComponent initializes.
2. The component calls the TaskService.
3. The service sends a GET request to the Rust backend.
4. The Rust route matches the request.
5. The Rust handler queries PostgreSQL.
6. The database returns task records.
7. Rust sends a JSON response.
8. Angular updates the tasks array.
9. The UI re-renders automatically.

This demonstrates complete request–response lifecycle.

---

## 6. Reflection on Modular Architecture

Angular’s modular structure improves scalability because:

- Components focus only on UI logic.
- Services handle API and business logic.
- Routing manages navigation separately.
- Each feature can be expanded independently.

Benefits:

- Clear separation of concerns
- Easier debugging
- Better maintainability
- Improved scalability
- Reusable components

As the application grows (e.g., adding users, dashboards, authentication), this modular approach prevents code duplication and keeps the project organized.