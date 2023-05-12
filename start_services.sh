#!/bin/bash

# Kill processes on ports 8080 and 8000
kill_process() {
  echo "Checking if port $1 is in use..."
  PID=$(lsof -t -i:$1)
  if [ -z "$PID" ]; then
    echo "Port $1 is free."
  else
    echo "Port $1 is in use. Killing process..."
    kill -9 $PID
  fi
}

kill_process 8080
kill_process 8000

cd gin_microservice
echo "Starting Gin microservice..."
go run main.go &

cd ../actix_microservice
echo "Starting Actix microservice..."
cargo run &

# Trap SIGINT (Ctrl+C) and kill the Go and Rust processes
trap "echo 'Stopping services...'; kill_process 8080; kill_process 8000; exit" INT

# Wait indefinitely
while true; do sleep 1; done
