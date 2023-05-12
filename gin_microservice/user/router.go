package user

import (
	"github.com/gin-gonic/gin"
	"github.com/jackc/pgx/v4"
	"github.com/streadway/amqp"
)

func UserRouter(r *gin.Engine, conn *pgx.Conn, ch *amqp.Channel) {
	r.POST("/user", func(c *gin.Context) {
		CreateUser(c, conn, ch)
	})
}
