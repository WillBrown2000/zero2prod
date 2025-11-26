use crate::domain::subscriber_email::SubscriberEmail;
use reqwest::Client;
use std::time::Duration;
use secrecy::{ExposeSecret, SecretString};

pub struct EmailClient {
    sender: SubscriberEmail,
    http_client: Client,
    base_url: String,
    authorization_token: SecretString,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail, timeout: Duration, authorization_token: SecretString) -> Self {
        let http_client = Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to build reqwest client");
        Self {
            sender,
            http_client,
            base_url,
            authorization_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_body: &str,
        text_body: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/email", self.base_url);
        let request_body = SendEmailRequest {
            from: self.sender.as_ref(),
            to: recipient.as_ref(),
            subject,
            html_body: html_body,
            text_body: text_body,
        };

        self
            .http_client
            .post(&url)
            .header(
                "X-Postmark-Server-Token",
                self.authorization_token.expose_secret().to_string(),
            )
            .json(&request_body)
            .send()
            .await?
            .error_for_status_ref()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use claims::assert_err;
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::Fake;
    use secrecy::SecretString;
    use wiremock::matchers::{any, header, header_exists, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &wiremock::Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            if let Ok(body) = result {
                body.get("From").is_some()
                && body.get("To").is_some()
                && body.get("Subject").is_some()
                && body.get("HtmlBody").is_some()
            } else {
                false
            }
        }
    }
    #[tokio::test]
    async fn send_email_succeeds_if_the_server_returns_200() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(
            mock_server.uri(),
            sender,
            Duration::new(10, 0),
            SecretString::new("secret".to_string().into_boxed_str()),
        );

        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200).set_body_string("OK"))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(10..20).fake();
        let content: String = Paragraph(1..10).fake();

        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content).await;

    }
    #[tokio::test]
    async fn send_email_fails_if_the_server_returns_500() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(
            mock_server.uri(),
            sender,
            Duration::new(10, 0),
            SecretString::new("secret".to_string().into_boxed_str()),
        );

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(10..20).fake();
        let content: String = Paragraph(1..10).fake();

        let outcome = email_client
            .send_email(subscriber_email, &subject, &content, &content).await;
        assert_err!(outcome);
    }
    #[tokio::test]
    async fn send_email_times_out_if_the_server_takes_too_long() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(
            mock_server.uri(),
            sender,
            Duration::new(10, 0),
            SecretString::new("secret".to_string().into_boxed_str()),
        );


        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(10..20).fake();
        let content: String = Paragraph(1..10).fake();
        let response = ResponseTemplate::new(200).set_delay(Duration::from_secs(180));
        Mock::given(any())
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(subscriber_email, &subject, &content, &content).await;
        assert_err!(outcome);
    }
}