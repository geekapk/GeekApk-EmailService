use std;
use std::fmt::Display;
use std::error::Error;
use config::Config;
use redis;
use redis::Commands;
use sendgrid;
use mail::MailInfo;
use serde_json;

pub struct Worker {
    config: Config
}

impl Worker {
    pub fn new(config: Config) -> Worker {
        Worker {
            config: config
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            match self.run_once() {
                Err(e) => println!("Error in worker: {}", e.description())
            };
            std::thread::sleep(
                std::time::Duration::from_millis(1000)
            );
        }
    }

    pub fn run_once(&mut self) -> Result<!, Box<Error>> {
        let redis_client = redis::Client::open(self.config.redis_url.as_str())?;
        let redis_conn = redis_client.get_connection()?;
        let sg_sender = sendgrid::v3::V3Sender::new(self.config.sendgrid_api_key.clone());

        loop {
            let mail_info_s: (String, String) = redis_conn.brpop(self.config.redis_prefix.clone() + "email_queue", 0)?;
            let mail_info_s = mail_info_s.1;

            let mail_info: MailInfo = match serde_json::from_str(mail_info_s.as_str()) {
                Ok(v) => v,
                Err(e) => {
                    println!("Error while parsing mail info: {}", e.description());
                    continue;
                }
            };
            println!("New mail to {}", mail_info.receiver);
            match send_mail(&self.config, &sg_sender, &mail_info) {
                Ok(_) => println!("Done"),
                Err(e) => println!("Error while sending mail: {}", e.description())
            }
        }
    }
}

fn send_mail(
    config: &Config,
    sender: &sendgrid::v3::V3Sender,
    mail_info: &MailInfo
) -> Result<(), Box<Error>> {
    let mut mail = sendgrid::v3::SGMailV3::new();
    mail.set_subject(mail_info.title.as_str());

    let mut from_email = sendgrid::v3::Email::new();
    from_email.set_email(config.email_from_address.as_str());
    mail.set_from(from_email);

    let mut content = sendgrid::v3::Content::new();
    content.set_value(mail_info.content.as_str());
    content.set_content_type("text/html");

    mail.add_content(content);

    let mut to_email = sendgrid::v3::Email::new();
    to_email.set_email(mail_info.receiver.as_str());

    let mut psn = sendgrid::v3::Personalization::new();
    psn.add_to(to_email);

    mail.add_personalization(psn);

    let status = sender.send(&mail)?;
    if status.is_success() {
        Ok(())
    } else {
        Err(SendMailError::with_message(
            format!(
                "SendGrid API call failed: {:?}",
                status
            )
        ).into())
    }
}

#[derive(Debug)]
pub struct SendMailError {
    msg: String
}

impl SendMailError {
    pub fn with_message(msg: String) -> SendMailError {
        SendMailError {
            msg: msg
        }
    }
}

impl Error for SendMailError {
    fn description(&self) -> &str {
        self.msg.as_str()
    }
}

impl Display for SendMailError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.msg.as_str(), f)
    }
}
