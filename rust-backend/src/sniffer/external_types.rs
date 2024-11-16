use std::collections::HashMap;

use tokio::sync::Mutex;

lazy_static! {
    pub static ref PORTS_TO_SNIFF: Mutex<HashMap<i16, ()>> = Mutex::new(HashMap::new());
}
