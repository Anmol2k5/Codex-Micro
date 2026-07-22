mod adapter;
mod locator;
pub mod action_confirmation;
pub mod action_executor;
pub mod element_query;
pub mod input_fallback;
pub mod selector_profile;
pub mod uia_client;
pub mod uia_worker;

pub use adapter::WindowsAutomationAdapter;
#[allow(unused_imports)]
pub use locator::{TargetAppLocator, TargetWindow};
