use crate::domain::subscriber_email::SubscriberEmail;
use reqwest::Client;
use std::time::Duration;

pub struct EmailClient {
    sender: SubscriberEmail,
    http_client: Client,
    base_url: String,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail, timeout: Duration) -> Self {
        let http_client = Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to build reqwest client");
        Self {
            sender,
            http_client,
            base_url,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        // Implementation will be added later
        let _ = (recipient, subject, html_content, text_content);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use wiremock::matchers::any;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(mock_server.uri(), sender, Duration::new(u64::MAX, 0));

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(10..20).fake();
        let content: String = Paragraph(1..10).fake();

        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content).await;

    }
}