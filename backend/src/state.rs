use std::sync::Mutex;

// Application State, shared with all routes and resources, Accessed via `web::Data<AppState>`
// Shared Mutable State
pub struct AppState {
    pub web_socket_subscription_id: Mutex<Option<String>>,
}

impl AppState {
    pub fn set_subscription_id(&self, subscription_id: String) {
        let mut guard = self.web_socket_subscription_id.lock().unwrap();
        *guard = Some(subscription_id);
    }
}
