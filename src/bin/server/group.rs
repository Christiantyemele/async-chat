use crate::connection::Outbound;
use async_chat::FromServer;
use async_std::task;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct Group {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}

impl Group {
    pub fn new(name: Arc<String>) -> Group {
        let (sender, _receiver) = broadcast::channel(1000);
        Group { name, sender }
    }
    pub fn join(&self, outbound: Arc<Outbound>) {
        let receiver = self.sender.subscribe();
        task::spawn(handle_subscriber(self.name.clone(), receiver, outbound));
    }
    pub fn post(&self, message: Arc<String>) {
        let _ignored = self.sender.send(message);
    }
}

async fn handle_subscriber(
    group_name: Arc<String>,
    mut receiver: broadcast::Receiver<Arc<String>>,
    outbound: Arc<Outbound>,
) {
    while let Ok(message) = receiver.recv().await {
        let response = FromServer::Message {
            group_name: group_name.clone(),
            message: message.clone(),
        };
        
        if let Err(e) = outbound.send(response).await {
            eprintln!("Error sending message to client: {}", e);
            // If we can't send to this client, we should probably remove them from the group
            break;
        }
    }
}
