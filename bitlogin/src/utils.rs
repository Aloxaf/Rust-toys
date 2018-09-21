extern crate base64;
extern crate byteorder;
extern crate hmac;
extern crate md5;

use std::str;

use byteorder::{ByteOrder, LittleEndian};
use hmac::{Hmac, Mac};
use md5::Md5;

pub fn hmacencode(key: &str, mes: &str) -> String {
    let mut mac = Hmac::<Md5>::new_varkey(&key.bytes().collect::<Vec<_>>()).unwrap();
    mac.input(&mes.bytes().collect::<Vec<_>>());
    let result = mac.result().code();
    let bytes = result
        .iter()
        .map(|&c| format!("{:02x}", c))
        .collect::<Vec<_>>();
    bytes.join("")
}

/// 处理魔改 base64
pub fn fkbase64(s: &str) -> String {
    let s = s.chars().map(|c| c as u8).collect::<Vec<_>>();
    let digest = base64::encode(&s);
    let old = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
    let new = "LVoJPiCN2R8G90yg+hmFHuacZ1OWMnrsSTXkYpUq/3dlbfKwv6xztjI7DeBE45QA="
        .chars()
        .collect::<Vec<_>>();
    digest
        .chars()
        .map(|c| new[old.find(c).unwrap()])
        .collect::<String>()
}

/// char[] -> int[] 小端序
fn unpack(msg: &str, key: bool) -> Vec<u32> {
    let len = msg.len();
    let cnt = if len % 4 == 0 { 0 } else { 4 - len % 4 };
    let msg = String::from(msg) + &"\x00".repeat(cnt);

    let mut pwd: Vec<u32> = msg
        .bytes()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|c| LittleEndian::read_u32(c))
        .collect();

    if key {
        pwd.push(len as u32)
    }
    pwd
}

/// int[] -> char[] 小端序
fn pack(msg: &[u32], _key: bool) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    msg.iter().for_each(|&i| {
        let mut buf = [0u8; 4];
        LittleEndian::write_u32(&mut buf, i);
        bytes.extend(buf.iter());
    });
    bytes.iter().map(|&c| c as char).collect()
}

/// 我也不知道这是什么加密方式...
/// 对着　Python 版本翻译的...
/// 而 Python 版本是对着 js 版本翻译的...
pub fn xencode(msg: &str, key: &str) -> String {
    if msg == "" {
        return String::new();
    }

    let pwd = unpack(msg, true);
    let pwdk = unpack(key, false);

    let mut pwd = pwd.iter().map(|&c| c as u64).collect::<Vec<_>>();
    let mut pwdk = pwdk.iter().map(|&c| c as u64).collect::<Vec<_>>();

    //println!("{:?}\n{:?}", pwd, pwdk);

    if pwdk.len() < 4 {
        for _ in 0..4 - pwdk.len() {
            pwdk.push(0);
        }
    }
    let n: u64 = pwd.len() as u64 - 1;
    let c: u64 = 0x86014019 | 0x183639A0;
    let mut z = pwd[n as usize];
    let (mut y, mut m, mut e, mut p): (u64, u64, u64, u64);
    let mut d = 0u64;
    let mut q = (6.0 + 52.0 / (n as f32 + 1.0)).floor();

    while 0.0 < q {
        d = d + c & (0x8CE0D9BF | 0x731F2640);
        e = (d >> 2) & 3;
        p = 0;

        while p < n {
            y = pwd[(p + 1) as usize];
            m = (z >> 5) ^ (y << 2);
            m += ((y >> 3) ^ (z << 4)) ^ (d ^ y);
            m += pwdk[((p & 3) ^ e) as usize] ^ z;
            pwd[p as usize] = (pwd[p as usize] + m) & (0xEFB8D130 | 0x10472ECF);
            z = pwd[p as usize];
            p = p + 1;
        }

        y = pwd[0];
        m = (z >> 5) ^ (y << 2);
        // println!(" {}", m);
        m += (y >> 3) ^ (z << 4) ^ (d ^ y);
        // println!(" {}", m);
        m += pwdk[((p & 3) ^ e) as usize] ^ z;
        pwd[n as usize] = (pwd[p as usize] + m) & (0xBB390742 | 0x44C6F8BD);
        z = pwd[n as usize];
        q -= 1.0;
    }
    let pwd = pwd.iter().map(|&c| c as u32).collect::<Vec<_>>();
    pack(&pwd, false)
}

#[cfg(test)]
mod test {
    use super::{fkbase64, hmacencode, pack, unpack, xencode};

    #[test]
    fn test_hmac() {
        assert_eq!("7afed10b1e1bc70b8a428a44754de091", &hmacencode("123", ""));
    }

    #[test]
    fn test_fkbase64() {
        assert_eq!("ZaRk", fkbase64("abc"));
        assert_eq!("o/qi", fkbase64("伊莉雅"));
    }

    #[test]
    fn test_pack() {
        assert_eq!("abcde\x00\x00\x00", &pack(&[0x64636261, 101], false));
    }

    #[test]
    fn test_unpack() {
        assert_eq!(vec![1684234849, 101, 5], unpack("abcde", true));
    }

    #[test]
    fn test_xencode() {
        assert_eq!("êÛ\x1d]Ó0c1", xencode("abc", "def"));
    }
}
