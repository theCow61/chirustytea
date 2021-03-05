mod bank;
use std::env;

// use bank::Bank;
use futures::StreamExt;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, Message, MessageKind, /*ParseMode,*/ UpdateKind};

struct Commandment {
    api: Api,
    message: Message,
}

impl Commandment {
    async fn new(api: Api, message: Message) -> Commandment {
        Commandment {
            api,
            message,
        }
    }
    async fn get_admins(&self) -> Result<(), Error> {
        let admins = self.api.send(self.message.chat.get_administrators()).await?;
        let mut response = Vec::new();
        for member in admins {
            response.push(member.user.first_name.clone())
        }
        self.api.send(self.message.text_reply(format!("Admins: {}", response.join(", "))))
            .await?;

        Ok(())
    }

    async fn get_balance(&self) -> Result<(), Box<dyn std::error::Error>> {
        // let from_user = self.message.from.username.as_ref().unwrap();
        //// let bank = bank::Bank::new(self.message.from.username.clone().unwrap()).unwrap();
        // bank.get_balance();
        // let balance = &bank.json_object["CowSheckles"][self.message.from.username.as_ref().unwrap()];
        // self.api.send(self.message.text_reply(format!("You got {}", balance))).await?;

        let mut bank = bank::Bank::new()?;
        let balance = bank.get_balance(&self.message.from.username.as_ref().unwrap()).await?;
        match balance {
            0 => {
                self.api.send(self.message.text_reply("Your poor and have no Cow Sheckles 😑.")).await?;
            },
            _ => {
                self.api.send(self.message.text_reply(format!("You got {} Cow Sheckles.", balance))).await?;
            },
        }
        Ok(())
    }
}


async fn test_leave(api: Api, message: Message) -> Result<(), Error> {
    api.send(message.chat.leave()).await?;
    Ok(())
}

async fn test(api: Api, message: Message) -> Result<(), Box<dyn std::error::Error>> {
    let comm = Commandment::new(api.clone(), message.clone()).await;
    match message.kind {
        MessageKind::Text { ref data, .. } => match data.as_str() {
           // "/get_chat_administrators" => test_get_chat_administrators(api, message).await?,
            "/get_admins" => comm.get_admins().await?,
            "/leave" => test_leave(api, message).await?,
            "/get_ball" => comm.get_balance().await?,
            _ => (),
        },
        _ => (),
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect(".env file not found...");
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set...");
    let api = Api::new(token);
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = match update {
            Ok(udp) => udp,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        if let UpdateKind::Message(message) = update.kind {
            test(api.clone(), message).await?;
        }
    }

    Ok(())
}

