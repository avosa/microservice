package main

import (
	"github.com/avosa/microservice/gin_microservice/db"
	"github.com/avosa/microservice/gin_microservice/rabbitmq"

	"github.com/avosa/microservice/gin_microservice/user"

	"github.com/gin-gonic/gin"
)

func main() {
	conn, _ := db.Connect()
	ch, _ := rabbitmq.Connect()

	r := gin.Default()

	user.UserRouter(r, conn, ch)

	r.Run()
}
