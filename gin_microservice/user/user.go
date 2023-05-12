package user

import (
	"context"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/jackc/pgx/v4"
	"github.com/streadway/amqp"
)

type User struct {
	FirstName       string `json:"first_name"`
	MiddleName      string `json:"middle_name"`
	LastName        string `json:"last_name"`
	PersonalAddress string `json:"personal_address"`
}

func CreateUser(c *gin.Context, conn *pgx.Conn, ch *amqp.Channel) {
	var newUser User
	if err := c.ShouldBindJSON(&newUser); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	// Insert user into Postgres database
	_, err := conn.Exec(context.Background(), `
	INSERT INTO users (first_name, middle_name, last_name, personal_address) VALUES ($1, $2, $3, $4)
	`, newUser.FirstName, newUser.MiddleName, newUser.LastName, newUser.PersonalAddress)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	// Publish user creation event to RabbitMQ
	err = ch.Publish("", "userCreated", false, false, amqp.Publishing{
		ContentType: "text/plain",
		Body:        []byte(newUser.FirstName + " " + newUser.MiddleName + " " + newUser.LastName),
	})
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(200, gin.H{"status": "User created successfully"})
}
