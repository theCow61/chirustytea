//use async_compat::{Compat, CompatExt};
use super::Commandment;
use serde_json::{json /*to_string_pretty*/, Value as JsonValue};
//use std::collections::HashMap;

const WORD_TOKEN_INC: u64 = 1;


pub struct Bank {
    json_object: JsonValue,
    db_path: String
}

impl Bank {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        dotenv::dotenv()?;
        let contents = async_std::fs::read_to_string(std::env::var("STAT_PATH").expect("Env not set...")).await?;
        let res = serde_json::from_str(&contents)?;
        Ok(Self {
            json_object: res,
            db_path: std::env::var("STAT_PATH")?,
        })
    }
    pub async fn get_balance(&mut self, commandment: &Commandment<'_>) -> Result<u64, async_std::io::Error> {
        let user = commandment.user.unwrap();
        self.null_nullifier(user).await?;

        Ok(self.json_object["CowSheckles"][user].as_u64().unwrap())
    }

    async fn null_nullifier(&mut self, test_if_null: &str) -> Result<(), async_std::io::Error> {
        if self.json_object["CowSheckles"][test_if_null] == JsonValue::Null {
            self.json_object["CowSheckles"][test_if_null] = json!(0);
            let new_contents = serde_json::to_string_pretty(&self.json_object)?;
            async_std::fs::write(&self.db_path, new_contents).await?;
        }

        Ok(())
    }
}

//#[derive(serde::Serialize, serde::Deserialize)]
//pub struct Bank {
//    // username: String,
//    // balance: f64,
//    // tele_user_executer: String,
//    json_object: JsonValue,
//    db_path: String,
//    //pub msg_info: HashMap<String, String>,
//}

//impl Bank {
//    pub async fn new(/*tele_user_executer: String*/) -> Result<Bank, Box<dyn std::error::Error>> {
//        // let bruha = tele_user_executer.to_owned();
//        dotenv::dotenv()?;
//        let contents =
//            async_std::fs::read_to_string(std::env::var("STAT_PATH").expect("Env not set...")).await?;
//        let res = serde_json::from_str(&contents)?;
//        //let msg_info = HashMap::new();
//        Ok(Bank {
//            // tele_user_executer: bruha,
//            json_object: res,
//            db_path: std::env::var("STAT_PATH")?,
//            //msg_info,
//        })
//        // match res {
//        //     Ok(resa) => {
//        //         let p: Bank = resa;
//        //         Ok(p)
//        //     }
//        //     Err(e) => {
//        //         println!("Could not parse json...");
//        //         Err(Box::new(e))
//        //     }
//        // }
//        // Ok(
//        //     Bank {
//        //         username: "lul".to_owned(),
//        //         balance: 3f64,
//    }
//    pub async fn get_balance(
//        &mut self,
//        commandent: &Commandment<'_>,
//    ) -> Result<u64, async_std::io::Error> {
//        // if self.json_object["CowSheckles"][commandent.cap_map["Executer"]] == JsonValue::Null {
//        //     self.json_object["CowSheckles"][commandent.cap_map["Executer"]] = json!(0);
//        //     let new_contents = serde_json::to_string_pretty(&self.json_object)?;
//        //     tokio::fs::write(&self.db_path, new_contents).await?;
//        //     return Ok(0)
//        // }

//        let user = format!("@{}", commandent.cap_map["Executer"]);
//        let user = user.as_str();
//        self.null_nullifier(user).await?;
//        let balance: u64 = self.json_object["CowSheckles"][user].as_u64().unwrap();
//        //.to_owned();
//        Ok(balance)
//    }
//    pub async fn transfer(
//        &mut self,
//        commandent: &Commandment<'_>,
//        amount: u64,
//    ) -> Result<Option<()>, async_std::io::Error> {
//        // if self.json_object["CowSheckles"][&commandent.cap_map["Executer"]] == JsonValue::Null {
//        //     self.json_object["CowSheckles"][&commandent.cap_map["Executer"]] = json!(0);
//        //     let new_contents = serde_json::to_string_pretty(&self.json_object)?;
//        //     tokio::fs::write(&self.db_path, new_contents).await?;
//        //     return Ok(None)
//        // }
//        // if self.json_object["CowSheckles"][&commandent.cap_map[""]]
//        let user = format!("@{}", commandent.cap_map["Executer"]);
//        let user = user.as_str();
//        self.null_nullifier(user).await?;
//        self.null_nullifier(commandent.cap_map["Recep"]).await?;
//        if self.json_object["CowSheckles"][user].as_u64().unwrap() < amount {
//            return Ok(None);
//        }
//        if self.json_object["CowSheckles"][user].as_u64().unwrap() >= amount {
//            let to_inc = self.json_object["CowSheckles"][commandent.cap_map["Recep"]]
//                .as_u64()
//                .unwrap()
//                + amount;
//            let to_dec = self.json_object["CowSheckles"][user].as_u64().unwrap() - amount;
//            self.json_object["CowSheckles"][commandent.cap_map["Recep"]] = json!(to_inc);
//            self.json_object["CowSheckles"][user] = json!(to_dec);
//            let new_contents = serde_json::to_string_pretty(&self.json_object)?;
//            //tokio::fs::write(&self.db_path, new_contents).await?;
//            /*async_std::fs::write(&self.db_path, new_contents).await?;*/
//            async_std::fs::write(&self.db_path, new_contents).await?;
//            return Ok(Some(()));
//        }
//        Ok(None)
//    }
//    pub async fn word_detected(&mut self, commandent: &Commandment<'_>, _word: &str) -> Result<(), async_std::io::Error> {
//        let user = format!("@{}", commandent.cap_map["Executer"]);
//        let user = user.as_str();
//        self.null_nullifier(user).await?;
//        let to_inc = self.json_object["CowSheckles"][user].as_u64().unwrap() + WORD_TOKEN_INC;
//        self.json_object["CowSheckles"][user] = json!(to_inc);
//        let new_contents = serde_json::to_string_pretty(&self.json_object)?;
//        async_std::fs::write(&self.db_path, new_contents).await?;
//        // MAKE WORD GET RECORDED THE COUNT OF USE OF WORD STATISTICS.
//        Ok(())
//    }
//    pub fn cred_global(&self) -> i64{
//        return self.json_object["CowSheckles"]["@theCow61_benuts"].as_i64().unwrap();
//    }
//    async fn null_nullifier(&mut self, test_if_null: &str) -> Result<(), async_std::io::Error> {
//        if self.json_object["CowSheckles"][test_if_null] == JsonValue::Null {
//            self.json_object["CowSheckles"][test_if_null] = json!(0);
//            let new_contents = serde_json::to_string_pretty(&self.json_object)?;
//            //async_std::fs::write(&self.db_path, new_contents).await?;
//            async_std::fs::write(&self.db_path, new_contents).await?;
//        }
//        Ok(())
//    }
//}
