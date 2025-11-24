use crate::domain::subscriber_email::SubscriberEmail;
use crate::domain::SubscriberName;

pub struct NewSubscriber {
    pub name: SubscriberName,
    pub email: SubscriberEmail,
}
impl NewSubscriber {
    pub fn into_parts(self) -> (String, String) {
        let email = self.email.as_ref().to_string();
        let name = self.name.as_ref().to_string();
        (email, name)
    }
}
