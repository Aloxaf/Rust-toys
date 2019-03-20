use fxhash::hash;
use hashbrown::HashMap;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// 从字符串中提取出"字符串"
pub fn get_keywords(text: &str) -> Vec<&str> {
    #[inline]
    fn type_of(b: u8) -> u8 {
        if b.is_ascii_alphabetic() {
            1
        } else if b.is_ascii_digit() {
            2
        } else {
            3
        }
    }

    let bytes = text.bytes().map(type_of).collect::<Vec<_>>();
    let mut ret = Vec::with_capacity(10);
    let mut pos = 0;

    if bytes.len() > 1 {
        for i in 0..bytes.len() - 1 {
            if bytes[i] != 3 && bytes[i] != bytes[i + 1] {
                ret.push(&text[pos..=i]);
                pos = i + 1;
            } else if bytes[i] == 3 {
                pos = i + 1;
            }
        }
        if bytes[pos] != 3 && bytes[pos] == bytes[bytes.len() - 1] {
            ret.push(&text[pos..]);
        }
    }
    ret
}

/// 读取文件, 确定每个"字符串"出现的次数
pub fn make_keyword_map(path: &str) -> HashMap<usize, u32> {
    let file = BufReader::new(File::open(path).expect("无法打开文件"));
    let mut ret = HashMap::with_capacity(1000);

    for line in file.lines() {
        let line = line.unwrap();
        for keyword in get_keywords(&line) {
            *ret.entry(hash(keyword)).or_insert(0) += 1;
        }
    }

    ret
}

/// 根据每行的字符串出现次数生成一个"特征值"
pub fn make_line_map(
    path: &str,
    keyword_map: &HashMap<usize, u32>,
) -> (usize, HashMap<Vec<u32>, Vec<u32>>) {
    let file = BufReader::new(File::open(path).expect("无法打开文件"));
    let mut line_cnt = 0;
    let mut ret = HashMap::with_capacity(1000);

    for (idx, line) in file.lines().enumerate() {
        let line = line.unwrap();
        let keywords = get_keywords(&line);

        let mut occurs_cnt = keywords
            .iter()
            .map(|&s| *keyword_map.get(&hash(s)).unwrap())
            .collect::<Vec<_>>();
        occurs_cnt.sort_unstable_by(|a, b| b.cmp(a));

        ret.entry(occurs_cnt).or_insert(vec![]).push(idx as u32);
        line_cnt = idx;
    }

    (line_cnt, ret)
}

pub fn make_line_order(line_cnt: usize, line_map: &HashMap<Vec<u32>, Vec<u32>>) -> Vec<u32> {
    // 排序
    let mut keys = line_map.keys().collect::<Vec<_>>();
    keys.par_sort_unstable_by(|a, b| b.cmp(a));

    // 确定每行的顺序
    let mut line_order = vec![0; line_cnt + 1]; // TODO: 我没记错的话当 line_cnt 很大时会出问题
    let mut order = 0 as u32;
    for key in keys {
        let idxs = line_map.get(key).unwrap();
        for &idx in idxs {
            line_order[idx as usize] = order;
            order += 1;
        }
    }
    line_order
}
