use std::fs::File;

use serde::{Deserialize, Serialize};
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub dbserver: DbServerConfig,
    pub labelfmt: LabelFmtConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LabelFmtConfig {
    pub desc: String,
    pub supplier: String,
    pub remark: String,
    pub coo: String,
    pub envirattr: String,
}

impl Default for LabelFmtConfig {
    fn default() -> Self {
        Self {
            desc: "贴片普通晶体振荡器".to_string(),
            supplier: "鸿星科技（集团）股份有限公司".to_string(),
            remark: "".to_string(),
            coo: "CN".to_string(),
            envirattr: "HSF-S".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DbServerConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let ret = match (
            File::open("./app.yml"),
            File::open("/etc/config/app.yml"),
            std::env::var("APP_CONFIG"),
        ) {
            (Ok(f), _, _) => serde_yaml::from_reader(f),
            (_, Ok(f), _) => serde_yaml::from_reader(f),
            (_, _, Ok(f)) => serde_yaml::from_reader(File::open(f)?),
            _ => {
                let new_config = AppConfig::default();
                new_config.save()?;
                return Ok(new_config);
            }
        };
        Ok(ret?)
    }
    pub fn save(&self) -> anyhow::Result<()> {
        let f = File::create("./app.yml")?;
        serde_yaml::to_writer(f, self)?;
        Ok(())
    }
    pub async fn connect_db(&self) -> anyhow::Result<Client<Compat<TcpStream>>> {
        let mut config = Config::new();
        config.host(&self.dbserver.host);
        config.port(self.dbserver.port);
        config.authentication(AuthMethod::sql_server(
            &self.dbserver.user,
            &self.dbserver.password,
        ));
        config.trust_cert();

        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;
        let client = Client::connect(config, tcp.compat_write()).await?;
        Ok(client)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn load_save_should_work() -> anyhow::Result<()> {
        // fs::remove_file("./app.yml")?;
        // let mut config = AppConfig::load()?;
        // assert_eq!(config.dbserver.host, "");
        // assert_eq!(config.dbserver.port, 0);
        // assert_eq!(config.dbserver.user, "");
        // assert_eq!(config.dbserver.password, "");
        // assert_eq!(config.dbserver.database, "");

        // config.dbserver.host = "101.95.95.58".to_string();
        // config.dbserver.port = 1433;
        // config.dbserver.user = "sa".to_string();
        // config.dbserver.password = "Ibm123".to_string();
        // config.dbserver.database = "TTC".to_string();

        // config.save()?;
        let config = AppConfig::load()?;
        assert_eq!(config.dbserver.host, "101.95.95.58");
        assert_eq!(config.dbserver.port, 1433);
        assert_eq!(config.dbserver.user, "sa");
        assert_eq!(config.dbserver.password, "Ibm123");
        assert_eq!(config.dbserver.database, "TTC");
        Ok(())
    }
}
