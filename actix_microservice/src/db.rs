use mysql_async::Pool;

pub fn connect() -> Pool {
    Pool::new("mysql://username:password@localhost:3306/mydatabase")
}
