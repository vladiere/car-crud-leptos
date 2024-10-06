mod car_server;
mod category_server;
mod color_server;

pub use color_server::*;


#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::ServerFnError;
    use sqlx::{Connection, MySqlConnection};
    
    pub async fn db() -> Result<MySqlConnection, ServerFnError> {
        Ok(MySqlConnection::connect("mariadb://root:31N$t31n@$@localhost:3306/car_crud").await?)
    }
}
