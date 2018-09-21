extern crate failure;
#[macro_use]
extern crate json;
extern crate nix;
extern crate reqwest;
extern crate sha1;

mod utils;

use failure::Error;
use sha1::{Digest, Sha1};

pub struct User<'a> {
    api: &'a str,
    client: reqwest::Client,
    param_n: &'a str,
    param_type: &'a str,
    username: &'a str,
    password: &'a str,
    ip: String,
    acid: &'a str,
}

// FIXME: 这方法太TM挫了
pub fn get_host_ip() -> Result<String, ()> {
    let addrs = nix::ifaddrs::getifaddrs().unwrap();
    for ifaddr in addrs {
        if let Some(address) = ifaddr.address {
            let ip = address.to_str();
            if &ip[0..2] == "10" {
                return Ok(String::from(&ip[..ip.len() - 2]));
            }
        }
    }
    Err(())
}

impl<'a> User<'a> {
    pub fn new<'b>(username: &'b str, password: &'b str) -> User<'b> {
        User {
            api: "http://10.0.0.55/cgi-bin",
            client: reqwest::Client::new(),
            param_n: "200",
            param_type: "1",
            ip: get_host_ip().unwrap(),
            acid: "8",
            username,
            password,
        }
    }

    /// 获取登录 token
    fn get_token(&self) -> Result<String, Error> {
        let params = [
            ("callback", "jsonp"),
            ("username", self.username),
            ("ip", &self.ip),
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
        let result = sh.result().iter()
            .map(|&c| format!("{:02x}", c))
            .collect::<Vec<_>>();
        result.join("")
    }

    /// 构造登录登出需要的参数
    fn make_params(&self, action: &str) -> [(&str, String); 10] {
        let token = self.get_token().unwrap();
        let data = object!{
            "username" => self.username,
            "password" => self.password,
            "acid" => self.acid,
            "ip" => self.ip.clone(),
            "enc_ver" => "srun_bx1",
        };
        let hmd5 = utils::hmacencode(&token, "");
        let json_data = data.dump();
        let info = String::from("{SRBX1}") + &utils::fkbase64(&utils::xencode(&json_data, &token));

        let chksum = User::sha1(&format!(
            "{0}{1}{0}{2}{0}{3}{0}{4}{0}{5}{0}{6}{0}{7}",
            token, self.username, hmd5, self.acid, self.ip, self.param_n, self.param_type, info
        ));


        let params = [
            ("callback", "jsonp".to_string()),
            ("username", self.username.to_string()),
            ("action", action.to_string()),
            ("ac_id", self.acid.to_string()),
            ("type", self.param_type.to_string()),
            ("ip", self.ip.clone()),
            ("n", self.param_n.to_string()),
            ("password", (String::from("{MD5}") + &hmd5)),
            ("chksum", chksum),
            ("info", info),
        ];

        params
    }

    pub fn login(&self) -> json::JsonValue {
        let params = self.make_params("login");
        let mut res = self.client
            .get(&format!("{}/srun_portal", self.api))
            .query(&params)
            .send()
            .unwrap();
        // println!("{}", res.url());
        // println!("-- {}", res.text().unwrap());
        let data = res.text().unwrap();
        // println!("-- {}", data);
        json::parse(&data[6..data.len() - 1]).unwrap()
    }

    pub fn logout(&self) -> json::JsonValue {
        let params = self.make_params("logout");
        let mut res = self.client
            .get(&format!("{}/srun_portal", self.api))
            .query(&params)
            .send()
            .unwrap();
        let data = res.text().unwrap();
        json::parse(&data[6..data.len() - 1]).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::User;
    #[test]
    fn test_make_params() {
        let user = User::new("1120172179", "Fak3_Pa3sw07d");
    }
}