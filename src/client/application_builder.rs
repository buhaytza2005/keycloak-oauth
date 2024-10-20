use std::fmt::Debug;

use super::{AppConfig, Credential};

#[derive(Debug)]
pub struct PublicApplication<C: Credential> {
    pub config: AppConfig<C>,
}
