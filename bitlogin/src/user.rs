use crate::{utils, GetAcidError};
use failure::Error;
use json::{object, JsonValue};
use regex::Regex;
use reqwest;
use sha1::{Digest, Sha1};

pub struct User<'a> {
    api: &'a str,
    client: reqwest::Client,
    param_n: &'a str,
    param_type: &'a str,
    username: &'a str,
    password: &'a str,
}

// FIXME: 乱七八糟的生命周期, clone 什么的
impl<'a> User<'a> {
    pub fn new<'b>(username: &'b str, password: &'b str) -> Result<User<'b>, Error> {
        Ok(User {
            api: "http://10.0.0.55/cgi-bin",
            client: reqwest::Client::new(),
            param_n: "200",
            param_type: "1",
            username,
            password,
        })
    }

    /// 获取 acid
    /// 用于判断 wifi 类型
    fn get_acid() -> Result<String, GetAcidError> {
        let client = reqwest::Client::new();
        let re = Regex::new(r"index_(\d)+\.html").unwrap();
        let mut res = client.get("http://detectportal.firefox.com/").send()?;
        let text = res.text()?;

        if text == "success" {
            return Err(GetAcidError::AlreadyLogin);
        }

        let caps = re.captures(&text)?;
        Ok(caps.get(1)?.as_str().to_string())
    }

    /// 获取登录 token
    fn get_token(&self, ip: &str) -> Result<String, Error> {
        let params = [
            ("callback", "jsonp"),
            ("username", self.username),
            ("ip", ip),
        ];
        let mut res = self
            .client
            .get(&format!("{}/get_challenge", self.api))
            .query(&params)
            .send()?;
        let data = res.text()?;
        // raw data: jsonp({xxxx})
        let data = json::parse(&data[6..data.len() - 1])?;
        Ok(data["challenge"].to_string())
    }

    /// 计算 sha1 返回 digest
    fn sha1(s: &str) -> String {
        let mut sh = Sha1::default();
        sh.input(&s.bytes().collect::<Vec<_>>());
        let result = sh
            .result()
            .iter()
            .map(|&c| format!("{:02x}", c))
            .collect::<Vec<_>>();
        result.join("")
    }

    /// 构造登录登出需要的参数
    fn make_params(&self, action: &str, acid: &str) -> [(&str, String); 10] {
        let ip = utils::get_host_ip().unwrap();
        let token = self.get_token(&ip).unwrap();
        let data = object!{
            "username" => self.username,
            "password" => self.password,
            "acid" => acid,
            "ip" => ip.as_str(),
            "enc_ver" => "srun_bx1",
        };
        let hmd5 = utils::hmacencode(&token, "");
        let json_data = data.dump();
        let info = String::from("{SRBX1}") + &utils::fkbase64(&utils::xencode(&json_data, &token));

        let chksum = User::sha1(&format!(
            "{0}{1}{0}{2}{0}{3}{0}{4}{0}{5}{0}{6}{0}{7}",
            token, self.username, hmd5, acid, ip, self.param_n, self.param_type, info
        ));

        let params = [
            ("callback", "jsonp".to_string()),
            ("username", self.username.to_string()),
            ("action", action.to_string()),
            ("ac_id", acid.to_string()),
            ("type", self.param_type.to_string()),
            ("ip", ip),
            ("n", self.param_n.to_string()),
            ("password", (String::from("{MD5}") + &hmd5)),
            ("chksum", chksum),
            ("info", info),
        ];

        params
    }

    pub fn login(&self) -> Result<JsonValue, Error> {
        let acid = User::get_acid()?;
        let params = self.make_params("login", &acid);
        let mut res = self
            .client
            .get(&format!("{}/srun_portal", self.api))
            .query(&params)
            .send()
            .unwrap();
        // println!("{}", res.url());
        // println!("-- {}", res.text().unwrap());
        let data = res.text().unwrap();
        // println!("-- {}", data);
        Ok(json::parse(&data[6..data.len() - 1])?)
    }

    pub fn logout(&self, acid: &str) -> Result<JsonValue, Error> {
        let params = self.make_params("logout", acid);
        let mut res = self
            .client
            .get(&format!("{}/srun_portal", self.api))
            .query(&params)
            .send()
            .unwrap();
        let data = res.text().unwrap();
        Ok(json::parse(&data[6..data.len() - 1])?)
    }
}

#[cfg(test)]
mod test {
    use crate::User;
    #[test]
    fn test_make_params() {
        let user = User::new("1120172179", "Fak3_Pa3sw07d");
    }
}
