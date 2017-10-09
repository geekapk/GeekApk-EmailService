use std;
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
        }
    }
}
