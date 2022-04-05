pub mod bot {
    use std::collections::HashMap;
    use reqwest::Error;
    use serde::{Deserialize};

    // static LANGUAGE: &str = "Rust";
    // const THRESHOLD: i32 = 10;
    const API_URL: &str = "https://api.line.me/";

    fn get_block_client() -> reqwest::blocking::Client
    {
        reqwest::blocking::Client::new()
    }

    pub fn url() -> &'static str {
        API_URL
    }

    #[derive(Deserialize, Debug)]
    #[allow(unused_macros)]
    #[warn(dead_code)]
    pub struct AccessTokenResp {
        access_token: String,
        expires_in: u32,
        token_type: String
    }

    pub fn get_access_token(channel_id: u32, channel_secret: &str) -> Result<String, Error>
    {
        let channel_id = channel_id.to_string();
        println!("channel_id: {} channel_secret: {}", channel_id, channel_secret);
        let url = format!("{API_URL}/v2/oauth/accessToken");
        let mut params = HashMap::new();

        params.insert("grant_type", "client_credentials");
        params.insert("client_id", &channel_id);
        params.insert("client_secret", channel_secret);
        
        let result = get_block_client().post(url).form(&params).send()?.text()?;
        // let result: AccessTokenResp = get_block_client().post(url).form(&params).send()?.json::<AccessTokenResp>()?;
        Ok(result)
    }

    pub fn get_bot_info() -> Result<String, Error> 
    {
        let url = format!("{}/v2/bot/info", API_URL);
        let result = reqwest::blocking::get(url)?.text()?;
        Ok(result)
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
