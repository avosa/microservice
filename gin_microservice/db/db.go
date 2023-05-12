package db

import (
	"context"

	"github.com/jackc/pgx/v4"
)

func Connect() (*pgx.Conn, error) {
	return pgx.Connect(context.Background(), "postgres://username:password@localhost:5432/usersdatabase")
}
