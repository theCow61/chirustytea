mod aws;
mod bank;
use std::env;

use teloxide::{Bot, net::Download, prelude::{*}, types::File as TgFile};
use async_std::task;
//use async_compat::{Compat, CompatExt};
//use smol::prelude::*;
//use smol::{io, Async};
// use bank::Bank;
//use futures::StreamExt;
// use lazy_static::lazy_static;
// use telegram_bot::prelude::*;
//use telegram_bot::{Api, Error, Message, MessageKind, /*ParseMode,*/ UpdateKind};

const WORDLIST_PATH: &str = "wl.txt";
const BUCKET_NAME: &'static str = "mycowbucket";

//pub struct Commandment<'bob> {
//    api: &'bob Api,
//    message: &'bob Message,
//    pub cap_map: std::collections::HashMap<&'bob str, &'bob str>,

//    pub s3: &'bob aws::Aws<'bob>,
//}

//impl<'bob> Commandment<'bob> {
//    /*async fn new<'a>(api: Api, message: &'a telegram_bot::Message) -> Commandment<'bob> {
//        Commandment {
//            api,
//            message, cap_map: &std::collections::HashMap::new(),
//            //cap_map,
//        }
//    }*/
//    async fn _get_admins(&self) -> Result<(), Error> {
//        let admins = self
//            .api
//            .send(self.message.chat.get_administrators())
//            .await?;
//        let mut response = Vec::new();
//        for member in admins {
//            response.push(member.user.first_name.clone())
//        }
//        self.api
//            .send(
//                self.message
//                    .text_reply(format!("Admins: {}", response.join(", "))),
//            )
//            .await?;

//        Ok(())
//    }

//    async fn balance(&self) -> Result<(), Box<dyn std::error::Error>> {
//        // let from_user = self.message.from.username.as_ref().unwrap();
//        //// let bank = bank::Bank::new(self.message.from.username.clone().unwrap()).unwrap();
//        // bank.get_balance();
//        // let balance = &bank.json_object["CowSheckles"][self.message.from.username.as_ref().unwrap()];
//        // self.api.send(self.message.text_reply(format!("You got {}", balance))).await?;

//        let mut bank = bank::Bank::new().await?;
//        // bank.msg_info.insert("Executer".to_owned(), self.message.from.username.clone().unwrap());
//        let balance: u64 = bank.get_balance(&self).await?;
//        match balance {
//            0 => {
//                self.api
//                    .send(
//                        self.message
//                            .text_reply("Your poor and have no Cow Sheckles 😑."),
//                    )
//                    .await?;
//            }
//            _ => {
//                self.api
//                    .send(
//                        self.message
//                            .text_reply(format!("You got {} Cow Sheckles.", balance)),
//                    )
//                    .await?;
//            }
//        }
//        Ok(())
//    }

//    async fn transfer(&self) -> Result<(), Box<dyn std::error::Error>> {
//        // TODO MAKESET static REF into a function and get your captures from the message in self
//        // bank.msg_info.insert("Executer".to_owned(), self.message.from.username.clone().unwrap());
//        if let Ok(amount) = self.cap_map["Amount"].parse::<u64>() {
//            let mut bank = bank::Bank::new().await?;
//            match bank.transfer(self, amount).await {
//                Ok(Some(_)) => {
//                    self.api
//                        .send(self.message.text_reply("Transfer complete 😏."))
//                        .await?;
//                }
//                Ok(None) => {
//                    self.api
//                        .send(self.message.text_reply("Lol your broke 🤣."))
//                        .await?;
//                }
//                Err(_) => {}
//            }
//        }
//        Ok(())
//    }

//    async fn word_increment(&self, word: &str) -> Result<(), Box<dyn std::error::Error>> {
//        let mut bank = bank::Bank::new().await?;
//        bank.word_detected(self, word).await?;

//        Ok(())
//    }

//    async fn credits_global(&self) -> Result<(), Box<dyn std::error::Error>> {
//        let bank = bank::Bank::new().await?;
//        let credits = bank.cred_global();
//        self.api.send(self.message.text_reply(format!("{}", credits))).await?;
//        Ok(())
//    }

//    async fn aws_ls(&self) {
//        let objects_list = self.s3.ls().await;
//        let mut files = String::new();
//        for object in objects_list {
//            files.push_str(&object.key.unwrap());
//            files.push('\n');
//        }
//        self.api.send(self.message.text_reply(files)).await.unwrap();
//    }

//    async fn aws_download(&self, path: String) {
//        let bstream = self.s3.download(path).await;
//    }

//    async fn test(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//        match self.message.kind {
//            MessageKind::Text { ref data, .. } => {
//                // match data.as_str() {
//                //     "/get_ball" => self.balance().await?,
//                //     _ => {},
//                // }
//                lazy_static::lazy_static! {
//                    // static ref SET: regex::RegexSet = regex::RegexSet::new(&[
//                    //     r"(^/transfer)( +)(\d+)( +)(@\w+)", r"/get_ball",
//                    // ]).unwrap();
//                    static ref SET: [regex::Regex; 5] = [
//                        regex::Regex::new(r"/get_ball").unwrap(),
//                        regex::Regex::new(r"(^/transfer)( +)(\d+)( +)(@\w+)").unwrap(),
//                        regex::Regex::new(r"/credits_global").unwrap(),
//                        regex::Regex::new(r"/ls").unwrap(),
//                        regex::Regex::new(r"(^/download)( +)(\w+)").unwrap(),
//                    ];

//                    //static ref WORDLIST_VALS: String = async_std::fs::read_to_string(WORDLIST_PATH).await.unwrap();
//                    static ref WORDLIST_VALS: String = std::fs::read_to_string(WORDLIST_PATH).unwrap();
//                }
//                // if(SET.is_match(data.as_str())) {
//                //     let matches = SET.matches(data.as_str());
//                //     if(matches.matched(0)) {
//                //         let caps =
//                //     }
//                // }
//                let msg = data.as_str();
//                let user = self.message.from.username.as_ref().unwrap();
//                if SET[0].is_match(msg) { // get_ball
//                    println!("Found match");
//                    let bbbb = &self.message.chat.id();
//                    println!("{}", bbbb);
//                    //let user = format!("@{}", self.message.from.username.unwrap());
//                    let user = self.message.from.username.as_ref().unwrap();
//                    self.cap_map.insert("Executer", user);
//                    self.balance().await?;
//                }
//                if let Some(caps) = SET[1].captures(msg) { // transfer
//                    println!(
//                        "Found match caps: {} {}",
//                        caps.get(3).unwrap().as_str(),
//                        caps.get(5).unwrap().as_str()
//                    );
//               //     let user = self.message.from.username.as_ref().unwrap();
//                    self.cap_map.insert("Executer", user); //*    What if you were to have Bank methods use the cap_map from this struct instaid?    *//
//                    self.cap_map.insert("Amount", caps.get(3).unwrap().as_str()); //*  in transfer method have everything in a iflet block to make sure it parses or does nothing for int  *//
//                    self.cap_map.insert("Recep", caps.get(5).unwrap().as_str());

//                    self.transfer().await?;
//                }

//                if SET[2].is_match(msg) {
//                    self.credits_global().await?;
//                }

//                if SET[3].is_match(msg) {
//                    self.aws_ls().await;
//                }

//                if let Some(caps) = SET[4].captures(msg) {
//                    // println!("{}", caps.get(3).unwrap().as_str());
//                    self.aws_download(caps.get(3).unwrap().as_str().to_owned()).await;
//                }

//                for word in msg.to_lowercase().split(" ").collect::<Vec<&str>>() {
//                    // Make sure words in wordlist are all lowercase in wl.txt, lowercasing the
//                    // file everytime will create unnecesary overhead.
//                    if WORDLIST_VALS.contains(word) {
//                        println!("😏");
//                        self.cap_map.insert("Executer", user);
//                        self.word_increment(word).await?;
//                    }
//                }

//            }
//            _ => {}
//        }

//        Ok(())
//    }
//}

//async fn _test_leave(api: Api, message: Message) -> Result<(), Error> {
//    api.send(message.chat.leave()).await?;
//    Ok(())
//}

//*async fn test(api: Api, message: Message) -> Result<(), Box<dyn std::error::Error>> {
//    let comm = Commandment::new(api.clone(), message.clone()).await;
//    match message.kind {
//        MessageKind::Text { ref data, .. } => match data.as_str() {
//           // "/get_chat_administrators" => test_get_chat_administrators(api, message).await?,
//            "/get_admins" => comm.get_admins().await?,
//            "/leave" => test_leave(api, message).await?,
//            "/get_ball" => comm.balance().await?,
//            _ => {
//        // if (/*re.is_match(data.as_str())*/) {
//        // }
//        },
//        },
//        _ => (),
//    };

//    Ok(())
//}*/
////#[async_std::main]
//async fn old_async_main() -> Result<(), Box<dyn std::error::Error>> {
//    dotenv::dotenv().expect(".env file not found...");
//    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set...");
//    let api = Api::new(token);
//    let mut stream = api.stream();
//    let cloned = api.clone();
//    task::spawn(async move {
//        they_prog(cloned).await.unwrap();
//    });
//    /*smol::spawn(async move {
//        they_prog(cloned).await.unwrap();
//    }).detach();*/
//    let s3 = aws::Aws::new("mycowbucket", Region::UsEast2);

//    while let Some(update) = stream.next().await {
//        let update = match update {
//            Ok(udp) => udp,
//            Err(e) => {
//                println!("Error: {}", e);
//                continue;
//            }
//        };
//        if let UpdateKind::Message(message) = update.kind {
//            // let mut command = Commandment::new(api.clone(), message).await;
//            //let mut command = Commandment::new(api.clone(), &message).await;
//            let mut command = Commandment {
//                api: &api,
//                message: &message,
//                cap_map: std::collections::HashMap::new(),
//                s3: &s3,
//            };
//            match command.test().await {
//                Err(e) => {
//                    println!("Error: {}", e);
//                },
//                _ => {},
//            }
//        }
//    }

//    Ok(())
//}

//                lazy_static::lazy_static! {
//                    // static ref SET: regex::RegexSet = regex::RegexSet::new(&[
//                    //     r"(^/transfer)( +)(\d+)( +)(@\w+)", r"/get_ball",
//                    // ]).unwrap();
//                    static ref SET: [regex::Regex; 5] = [
//                        regex::Regex::new(r"/get_ball").unwrap(),
//                        regex::Regex::new(r"(^/transfer)( +)(\d+)( +)(@\w+)").unwrap(),
//                        regex::Regex::new(r"/credits_global").unwrap(),
//                        regex::Regex::new(r"/ls").unwrap(),
//                        regex::Regex::new(r"(^/download)( +)(\w+)").unwrap(),
//                    ];

//                    //static ref WORDLIST_VALS: String = async_std::fs::read_to_string(WORDLIST_PATH).await.unwrap();
//                    static ref WORDLIST_VALS: String = std::fs::read_to_string(WORDLIST_PATH).unwrap();
//                }

pub struct Commandment<'a> {
    message: &'a Message,
    ap: &'a UpdateWithCx<AutoSend<Bot>, Message>,
    s3: &'a aws::Aws,
}

pub struct BankInfo<'bruh> {
    from_user: Option<&'bruh str>,
    amount: Option<&'bruh u64>,
    to_user: Option<&'bruh str>,
}

// pub struct AwsInfo<'burger> {
//     file_name: Option<&'burger str>,
//     file_contents: Option<&'burger Vec<u8>>,
// }

impl Default for BankInfo<'_> {
    fn default() -> Self {
        Self {
            from_user: None,
            amount: None,
            to_user: None,
        }
    }
}

impl<'a> Commandment<'a> {
    fn new(
        message: &'a Message,
        ap: &'a UpdateWithCx<AutoSend<Bot>, Message>,
        // ap: &'a Bot,
        s3: &'a aws::Aws,
    ) -> Self {
        Self { message, ap, s3 }
    }

    async fn balance(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // self.user = Some(self.message.from().unwrap().username.as_ref().unwrap());
        let bank_info = BankInfo {
            // TODO make default values so you don't have to define `None` for every field.
            from_user: Some(self.message.from().unwrap().username.as_ref().unwrap()),
            amount: None,
            to_user: None,
        };
        let mut bank = bank::Bank::new().await?;
        match bank.get_balance(&bank_info).await? {
            0 => {
                self.ap
                    .answer("Your poor and have no Cow Sheckles 😑.")
                    .send()
                    .await?;
            }
            bal => {
                self.ap
                    .answer(format!("You got {} Cow Sheckles.", bal))
                    .send()
                    .await?;
            }
        }

        Ok(())
    }

    async fn transfer(
        &self,
        amount: &u64,
        to_user: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let bank_info = BankInfo {
            from_user: Some(self.message.from().unwrap().username.as_ref().unwrap()),
            amount: Some(amount),
            to_user: Some(to_user),
        };

        let mut bank = bank::Bank::new().await?;
        match bank.transfer(&bank_info).await {
            Ok(Some(_)) => {
                self.ap.answer("Transfer complete 😃.").send().await?;
            }
            Ok(None) => {
                self.ap
                    .answer("Transfer failed: Your broke 🤣.")
                    .send()
                    .await?;
            }
            Err(_) => {}
        }

        Ok(())
    }

    async fn s3_ls(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let objects_list = self.s3.ls().await;
        let mut files = String::new();
        for object in objects_list {
            files.push_str(&object.key.unwrap());
            files.push('\n');
        }
        self.ap.answer(files).send().await?;

        Ok(())
    }

    async fn increment(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let bank_info = BankInfo {
            from_user: Some(self.message.from().unwrap().username.as_ref().unwrap()),
            ..Default::default()
        };
        let mut bank = bank::Bank::new().await?;
        bank.word_detected(&bank_info).await?;

        Ok(())
    }
    async fn bruh(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        lazy_static::lazy_static! {
            static ref SET: [regex::Regex; 5] = [
                regex::Regex::new(r"/balance").unwrap(),                            // /balance                     - 0
                regex::Regex::new(r"(^/transfer)( +)(\d+)( +)(@\w+)").unwrap(),     // /transfer <amount> <@user>   - 1
                regex::Regex::new(r"/ls").unwrap(),                                 // /ls                          - 2
                regex::Regex::new(r"(^/download)( +)(\w+)").unwrap(),               // /download <path>             - 3
                regex::Regex::new(r"/upload").unwrap(),                             // /upload                      - 4
            ];

           static ref WORDLIST_VALS: String = std::fs::read_to_string(WORDLIST_PATH).unwrap();
           // static ref WORDLIST_WORDS: Vec<&'static str> = WORDLIST_VALS.split_ascii_whitespace().collect::<Vec<&'static str>>();
           static ref WORDLIST_WORDS: Vec<&'static str> = WORDLIST_VALS.split_whitespace().collect::<Vec<&'static str>>();
        };

        // over here commandment

        //let msg = cx.update.text().unwrap();
        //let user = cx.update.from().unwrap().username.as_ref().unwrap();

        if let Some(document) = self.message.document() {
            if let Some(caption) = self.message.caption() {
                if SET[4].is_match(caption) {
                    let TgFile { file_path, .. } = self.ap.requester.get_file(&document.file_id).send().await?;
                    // let file_name = document.file_name.as_ref().unwrap();
                    let mut yod = Vec::new();
                    self.ap.requester.download_file(&file_path, &mut yod).await?;
                    self.s3.upload(document.file_name.as_ref().unwrap(), yod).await?;
                }
            }
        }
        if let Some(msg) = self.message.text() {
            if SET[0].is_match(msg) {
                // balance
                self.balance().await?;
            }

            if let Some(caps) = SET[1].captures(msg) {
                // transfer - caps.get(3) and caps.get(5)
                println!(
                    "transfer - {} - {}",
                    caps.get(3).unwrap().as_str(),
                    caps.get(5).unwrap().as_str()
                );
                self.transfer(
                    &caps.get(3).unwrap().as_str().parse::<u64>().unwrap(),
                    caps.get(5).unwrap().as_str(),
                )
                .await?;
            }

            if SET[2].is_match(msg) {
                // ls
                self.s3_ls().await?;
                println!("ls");
            }

            if let Some(caps) = SET[3].captures(msg) {
                // download - caps.get(3)
                println!("download - {}", caps.get(3).unwrap().as_str());
            }

            // for word in msg.to_lowercase().split(' ').collect::<Vec<&str>>() {
            //     if WORDLIST_VALS.contains(word) {
            //         println!("Detected {}", word);
            //     }
            // }

            for word in msg.to_lowercase().split_whitespace().collect::<Vec<&str>>() {
                if WORDLIST_WORDS.contains(&word) {
                    println!("{}", word);
                    self.increment().await?;
                }
            }
        }


        Ok(())
    }
}

async fn async_main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if cfg!(unix) {
        dotenv::dotenv().expect(".env file not found...");
    }
    teloxide::enable_logging!();

    let bot = Bot::from_env().auto_send();
    // let bot = Bot::from_env();
    lazy_static::lazy_static! {
        static ref AWSS3: aws::Aws = aws::Aws::new(rusoto_core::Region::UsEast2);
    }

    teloxide::repl(bot, |yo| async move {
        // let mut commandment = Commandment::new(&cx.update, &cx, &s3);
        let commandment = Commandment::new(&yo.update, &yo, &AWSS3);
        commandment.bruh().await
        // bruh(yo, s3).await
    })
    .await;

    // Maybe make a struct that has a reference to bank.

    // TODO
    // Pass a reference to a one time created instance of the `Bank` struct in this method.
    // Whenever a new `Commandment` struct gets created, it will have a field thats just a pointer
    // to the `Bank` struct. [Reason: constructor `Bank::new` reads a file every time,
    // reconstructing the same exact struct for every command doesn't make sense.]

    Ok(())
}

fn main() {
    // task::block_on(old_async_main()).unwrap(); // Old telegram
    task::block_on(async_main()).unwrap();
}

//async fn they_prog(api: Api) -> Result<(), async_std::io::Error> {
//    let chat = telegram_bot::ChatId::new(1232564384);
//    // let mut handle = stdin.lock();
//    let mut buf = String::new();
//    loop {
//        //std::io::stdin().read_line(&mut buf).unwrap();
//        async_std::io::stdin().read_line(&mut buf).await?;
//        /*let mut hi = smol::Unblock::new(std::io::stdin());
//        hi.read_to_string(&mut buf).compat().await?;*/
//        let _ = api.send(chat.text(&buf)).await;
//        // api.spawn(chat.text(buf.as_str())); /*  Incredibily slow send time  */
//        // match buf.as_str() {
//        //     "KILL" => {

//        //     },
//        //     _ => {},
//        // }
//        buf.clear();
//    }
//}
