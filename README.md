# Sample Microservice

This is RESTful microservice architecture example that demonstrates the integration of various technologies: Golang's Gin Framework to handle user-related operations (such as creating user data including first name, middle name, last name, and personal address), Rust's Actix Framework to manage properties owned by users (including property address, owner details, etc.) and RabbitMQ. In this example, Gin should is configured to use Postgres as the database while Actix is configured to use MySQL. The implementation showcases the communication and data linkage between these components.

## Overall project structure

```swift
Client
 |
 | HTTP
 |
Gin Microservice (Golang)
 |    | AMQP
 |    v
 | RabbitMQ
 |    | AMQP
 |    v
Actix Microservice (Rust)
```

### Gin Microservice (Golang)

```go
gin_microservice
├── main.go
├── db
│   └── db.go
├── rabbitmq
│   └── rabbitmq.go
├── user
│   ├── user.go
│   └── router.go
└── go.mod
```

### Actix Microservice (Rust)

```rust
actix_microservice
├── src
│   ├── main.rs
│   ├── db.rs
│   ├── rabbitmq.rs
│   ├── property.rs
│   └── router.rs
├── Cargo.toml
└── Cargo.lock
```

## Getting started

Here are scripts in Shell, Ruby, and Python that you can use to start both services simultaneously.

### Using Shell Script (start_services.sh):

To use this script, you'd give it execute permissions with:

```bash
chmod +x start_services.sh
```

and then run it with:

```bash
./start_services.sh
```

### Using Ruby Script (start_services.rb):

```bash
ruby start_services.rb
```

### using Python Script (start_services.py):

```bash
python start_services.py
```
