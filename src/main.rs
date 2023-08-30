use async_std::net::TcpStream;
use tiberius::{AuthMethod, Client, Config, Query};
use uuid::Uuid;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let mut config = Config::new();

    config.host("DESKTOP-UCEVIKS");
    config.port(1433);
    config.database("temp");

    config.authentication(AuthMethod::sql_server("sa", "sqlpwd"));

    config.trust_cert();

    println!("Connecting to database...");

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;

    println!("Connected to database!");

    let _query = "SELECT TOP (1000) UserId, UserName, Email FROM Users";

    let query = Query::new(_query);

    let mut stream = query.query(&mut client).await?;

    println!("*** Query results ***");

    while let Some(row) = stream.into_row().await? {
        let user_id: Option<Uuid> = row.get(0);
        let username: Option<&str> = row.get(1);
        let email: Option<&str> = row.get(2);

        println!(
            "User ID: {:?}, Username: {:?}, Email: {:?}",
            user_id, username, email
        );

        let query = Query::new(_query);
        stream = query.query(&mut client).await?;
    }

    println!("Query execution completed!");
    Ok(())
}
