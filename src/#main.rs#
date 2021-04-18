mod bank;
use std::env;

// use bank::Bank;
use futures::StreamExt;
// use lazy_static::lazy_static;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, Message, MessageKind, /*ParseMode,*/ UpdateKind};

pub struct Commandment {
    api: Api,
    message: Message,
    pub cap_map: std::collections::HashMap<String, String>,
}

impl Commandment {
    async fn new(api: Api, message: Message) -> Commandment {
        Commandment {
            api,
            message,
            cap_map: std::collections::HashMap::new(),
        }
    }
    async fn _get_admins(&self) -> Result<(), Error> {
        let admins = self
            .api
            .send(self.message.chat.get_administrators())
            .await?;
        let mut response = Vec::new();
        for member in admins {
            response.push(member.user.first_name.clone())
        }
        self.api
            .send(
                self.message
                    .text_reply(format!("Admins: {}", response.join(", "))),
            )
            .await?;

        Ok(())
    }

    async fn balance(&self) -> Result<(), Box<dyn std::error::Error>> {
        // let from_user = self.message.from.username.as_ref().unwrap();
        //// let bank = bank::Bank::new(self.message.from.username.clone().unwrap()).unwrap();
        // bank.get_balance();
        // let balance = &bank.json_object["CowSheckles"][self.message.from.username.as_ref().unwrap()];
        // self.api.send(self.message.text_reply(format!("You got {}", balance))).await?;

        let mut bank = bank::Bank::new()?;
        // bank.msg_info.insert("Executer".to_owned(), self.message.from.username.clone().unwrap());
        let balance: u64 = bank.get_balance(&self).await?;
        match balance {
            0 => {
                self.api
                    .send(
                        self.message
                            .text_reply("Your poor and have no Cow Sheckles 😑."),
                    )
                    .await?;
            }
            _ => {
                self.api
                    .send(
                        self.message
                            .text_reply(format!("You got {} Cow Sheckles.", balance)),
                    )
                    .await?;
            }
        }
        Ok(())
    }

    async fn transfer(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO MAKESET static REF into a function and get your captures from the message in self
        // bank.msg_info.insert("Executer".to_owned(), self.message.from.username.clone().unwrap());
        if let Ok(amount) = self.cap_map["Amount"].parse::<u64>() {
            let mut bank = bank::Bank::new()?;
            match bank.transfer(&self, amount).await {
                Ok(Some(_)) => {
                    self.api
                        .send(self.message.text_reply("Transfer complete 😏."))
                        .await?;
                }
                Ok(None) => {
                    self.api
                        .send(self.message.text_reply("Lol your broke 🤣."))
                        .await?;
                }
                Err(_) => {}
            }
        }
        Ok(())
    }

    async fn test(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.message.kind {
            MessageKind::Text { ref data, .. } => {
                // match data.as_str() {
                //     "/get_ball" => self.balance().await?,
                //     _ => {},
                // }
                lazy_static::lazy_static! {
                    // static ref SET: regex::RegexSet = regex::RegexSet::new(&[
                    //     r"(^/transfer)( +)(\d+)( +)(@\w+)",
                    //     r"/get_ball",
                    // ]).unwrap();
                    static ref SET: [regex::Regex; 2] = [
                        regex::Regex::new(r"/get_ball").unwrap(),
                        regex::Regex::new(r"(^/transfer)( +)(\d+)( +)(@\w+)").unwrap(),
                    ];
                }
                // if(SET.is_match(data.as_str())) {
                //     let matches = SET.matches(data.as_str());
                //     if(matches.matched(0)) {
                //         let caps =
                //     }
                // }
                let msg = data.as_str();
                if let Some(_) = SET[0].captures(msg) {
                    println!("Found match");
                    let bbbb = &self.message.chat.id();
                    println!("{}", bbbb);
                    self.cap_map.insert(
                        "Executer".to_owned(),
                        self.message.from.username.clone().unwrap(),
                    );
                    self.balance().await?;
                }
                if let Some(caps) = SET[1].captures(msg) {
                    println!(
                        "Found match caps: {} {}",
                        caps.get(3).unwrap().as_str(),
                        caps.get(5).unwrap().as_str()
                    );
                    self.cap_map.insert(
                        "Executer".to_owned(),
                        self.message.from.username.clone().unwrap(),
                    ); //*    What if you were to have Bank methods use the cap_map from this struct instaid?    *//
                    self.cap_map.insert(
                        "Amount".to_owned(),
                        caps.get(3).unwrap().as_str().to_owned(),
                    ); //*  in transfer method have everything in a iflet block to make sure it parses or does nothing for int  *//
                    self.cap_map
                        .insert("Recep".to_owned(), caps.get(5).unwrap().as_str().to_owned());

                    self.transfer().await?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

async fn _test_leave(api: Api, message: Message) -> Result<(), Error> {
    api.send(message.chat.leave()).await?;
    Ok(())
}

/*async fn test(api: Api, message: Message) -> Result<(), Box<dyn std::error::Error>> {
    let comm = Commandment::new(api.clone(), message.clone()).await;
    match message.kind {
        MessageKind::Text { ref data, .. } => match data.as_str() {
           // "/get_chat_administrators" => test_get_chat_administrators(api, message).await?,
            "/get_admins" => comm.get_admins().await?,
            "/leave" => test_leave(api, message).await?,
            "/get_ball" => comm.balance().await?,
            _ => {
        // if (/*re.is_match(data.as_str())*/) {
        // }
        },
        },
        _ => (),
    };

    Ok(())
}*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect(".env file not found...");
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set...");
    let api = Api::new(token);
    let mut stream = api.stream();
    let cloned = api.clone();
    tokio::task::spawn(async move {
        they_prog(cloned).await;
    });

    while let Some(update) = stream.next().await {
        let update = match update {
            Ok(udp) => udp,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        if let UpdateKind::Message(message) = update.kind {
            let mut command = Commandment::new(api.clone(), message).await;
            command.test().await?;
        }
    }

    Ok(())
}

async fn they_prog(api: Api) {
    let chat = telegram_bot::ChatId::new(1232564384);
    // let mut handle = stdin.lock();
    let mut buf = String::new();
    loop {
        std::io::stdin().read_line(&mut buf).unwrap();
        let _ = api.send(chat.text(&buf)).await;
        // api.spawn(chat.text(buf.as_str())); /*  Incredibily slow send time  */
        // match buf.as_str() {
        //     "KILL" => {

        //     },
        //     _ => {},
        // }
        buf.clear();
    }
}
