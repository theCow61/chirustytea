use serde_json::{Value as JsonValue, json};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Bank {
    // username: String,
    // balance: f64,
    // tele_user_executer: String,
    pub json_object: JsonValue,
    db_path: String,
}

impl Bank {
    pub fn new(/*tele_user_executer: String*/) -> Result<Bank, Box<dyn std::error::Error>> {
        // let bruha = tele_user_executer.to_owned();
        dotenv::dotenv().unwrap();
        let contents = std::fs::read_to_string(std::env::var("STAT_PATH").expect("Env not set..."))?;
        let res = serde_json::from_str(&contents)?;
        Ok(
            Bank {
                // tele_user_executer: bruha,
                json_object: res,
                db_path: std::env::var("STAT_PATH").unwrap(),
            }
        )
        // match res {
        //     Ok(resa) => {
        //         let p: Bank = resa;
        //         Ok(p)
        //     }
        //     Err(e) => {
        //         println!("Could not parse json...");
        //         Err(Box::new(e))
        //     }
        // }
        // Ok(
        //     Bank {
        //         username: "lul".to_owned(),
        //         balance: 3f64,

    }
    pub async fn get_balance(&mut self, executer: &str) -> Result<u64, std::io::Error> {
        if self.json_object["CowSheckles"][executer] == JsonValue::Null {
            self.json_object["CowSheckles"][executer] = json!(0);
            let new_contents = serde_json::to_string_pretty(&self.json_object)?;
            tokio::fs::write(self.db_path.clone(), new_contents).await?;
            return Ok(0)
        }
        let balance: u64 = self.json_object["CowSheckles"][executer].as_u64().unwrap().to_owned();
        Ok(balance)
    }
}
