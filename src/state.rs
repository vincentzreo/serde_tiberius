use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::Compat;

use crate::AppConfig;

#[derive(Debug)]
pub struct AppState {
    pub dbclient: Client<Compat<TcpStream>>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let config = AppConfig::load()?;
        let dbclient = config.connect_db().await?;
        Ok(Self { dbclient })
    }
}

#[cfg(test)]
mod tests {
    use tiberius::Query;

    use super::*;

    #[tokio::test]
    async fn appstate_new_should_work() -> anyhow::Result<()> {
        let mut state = AppState::new().await?;

        // let mut client = state.dbclient;
        let mut select = Query::new("SELECT @P1");
        select.bind(-4i32);

        let stream = select.query(&mut state.dbclient).await?;

        let row = stream.into_row().await?;
        assert_eq!(Some(-4i32), row.unwrap().get(0));
        Ok(())
    }
}
