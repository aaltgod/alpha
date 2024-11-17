use std::collections::HashMap;

use tokio::sync::Mutex;

lazy_static! {
    pub static ref PORTS_TO_SNIFF: Mutex<HashMap<i32, ()>> = Mutex::new(HashMap::new());
}
