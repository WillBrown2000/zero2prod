use crate::domain::SubscriberName;

pub struct NewSubscriber {
    pub name: SubscriberName,
    pub email: String,
}
impl NewSubscriber {
    pub fn into_parts(self) -> (String, String) {
        let email = self.email;
        let name = self.name.as_ref().to_string();
        (email, name)
    }
}
