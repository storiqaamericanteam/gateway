use failure::Error as FailureError;
use reqwest::Client;

pub struct UsersMicroservice {
    pub client: Client,
    pub config: gateway_lib::config::Config,
}

impl UsersMicroservice {
    pub fn clear_all_data(&self) -> Result<(), FailureError> {
        let url = format!("{}/clear_database", self.config.users_microservice.url);
        let _ = self.client.post(&url).send()?;
        Ok(())
    }
}
