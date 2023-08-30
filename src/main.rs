use async_std::net::TcpStream;
use tiberius::{AuthMethod, Client, Config, Query};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let mut config = Config::new();

    config.host("localhost"); 
    config.port(1433);
 
    config.authentication(AuthMethod::sql_server("USER ID", "PASSWORD"));

    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;

    let mut select = Query::new("SELECT @P1");
    select.bind(-4i32);

    let stream = select.query(&mut client).await?;
    let row = stream.into_row().await?;

    assert_eq!(Some(-4i32), row.unwrap().get(0));


    Ok(())
}
