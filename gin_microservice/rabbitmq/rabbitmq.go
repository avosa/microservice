package rabbitmq

import (
	"github.com/streadway/amqp"
)

func Connect() (*amqp.Channel, error) {
	conn, err := amqp.Dial("amqp://guest:guest@localhost:5672/")
	if err != nil {
		return nil, err
	}

	return conn.Channel()
}
