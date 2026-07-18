use std::sync::Arc;

use crate::automation::adapter::AutomationAdapter;

pub struct AppState {
    pub automation: Arc<dyn AutomationAdapter>,
}

impl AppState {
    pub fn new(automation: Arc<dyn AutomationAdapter>) -> Self {
        Self { automation }
    }
}
