#[derive(Deserialize)]
pub struct MailInfo {
    pub receiver: String,
    pub title: String,
    pub content: String
}
