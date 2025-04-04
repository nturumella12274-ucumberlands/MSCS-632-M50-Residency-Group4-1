// Import necessary modules
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::task;

// Define a User struct with an ID and name
#[derive(Debug, Clone)]
struct User {
    _id: u32,  // User ID, prefixed with underscore to suppress unused warning
    name: String,  // User name
}

// Define a Message struct with a timestamp, sender, receiver, and content
#[derive(Debug, Clone)]
struct Message {
    timestamp: u64,  // Time when the message was sent
    sender_id: u32,  // ID of the sender
    receiver_id: u32,  // ID of the receiver
    content: String,  // Content of the message
}

// Define a Chat struct to manage users and messages
#[derive(Clone)]
struct Chat {
    users: HashMap<u32, User>,  // Map of user IDs to User structs
    messages: Vec<Message>,  // List of messages
}

impl Chat {
    // Create a new Chat instance
    fn new() -> Self {
        Chat {
            users: HashMap::new(),
            messages: Vec::new(),
        }
    }

    // Add a user to the chat
    fn add_user(&mut self, id: u32, name: &str) {
        self.users.insert(id, User { _id: id, name: name.to_string() });
        println!("Added user: {} with id: {}", name, id);
    }

    // Send a message asynchronously
    async fn send_message_async(&mut self, sender_id: u32, receiver_id: u32, content: &str) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let message = Message {
            timestamp,
            sender_id,
            receiver_id,
            content: content.to_string(),
        };
        self.messages.push(message);
    }

    // Display messages for a specific user
    async fn display_messages_async(&self, user_id: u32) {
        for message in &self.messages {
            if message.sender_id == user_id || message.receiver_id == user_id {
                let sender_name = self.users.get(&message.sender_id).map_or("Unknown", |user| &user.name);
                let receiver_name = self.users.get(&message.receiver_id).map_or("Unknown", |user| &user.name);
                println!("[{}] {} -> {}: {}", message.timestamp, sender_name, receiver_name, message.content);
            }
        }
    }

    // Search for messages containing a specific keyword
    async fn search_messages_async(&self, keyword: &str) {
        for message in &self.messages {
            if message.content.contains(keyword) {
                println!("{:?}", message);
            }
        }
    }
}

// Main function to run the chat application
#[tokio::main]
async fn main() {
    let mut chat = Chat::new();
    chat.add_user(1, "Alice");
    chat.add_user(2, "Bob");

    // Send messages asynchronously
    let mut chat_clone = chat.clone();
    task::spawn(async move {
        chat_clone.send_message_async(1, 2, "Hello, Bob!").await;
    }).await.unwrap();

    let mut chat_clone_2 = chat.clone();
    task::spawn(async move {
        chat_clone_2.send_message_async(2, 1, "Hi, Alice!").await;
    }).await.unwrap();

    // Display messages for Alice
    println!("Messages for Alice:");
    chat.display_messages_async(1).await;

    // Search for messages containing 'Hello'
    println!("Messages containing 'Hello':");
    chat.search_messages_async("Hello").await;
}
