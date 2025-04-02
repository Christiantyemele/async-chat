pub mod connection;
pub mod group_table;
pub mod group;
pub mod user;

use connection::serve;


use async_std::prelude::*;
use async_std::net::TcpListener;
use async_std::task;
use std::sync::Arc;

fn main() -> anyhow::Result<()> {
    let address = std::env::args().nth(1).expect(
        "Usage: server
    ADDRESS",
    );
    let chat_group_table = Arc::new(group_table::GroupTable::new());
    let user_manager = Arc::new(user::UserManager::new());

    async_std::task::block_on(async {
        let listener = TcpListener::bind(address).await?;
        let mut new_connections = listener.incoming();
        while let Some(socket_result) = new_connections.next().await {
            let socket = socket_result?;
            let groups = chat_group_table.clone();
            let users = user_manager.clone();
            task::spawn(async {
                log_error(serve(socket, groups, users).await);
            });
        }
        Ok(())
    })
}

fn log_error(result: anyhow::Result<()>) {
    if let Err(error) = result {
        eprintln!("Error: {}", error);
    }
}
