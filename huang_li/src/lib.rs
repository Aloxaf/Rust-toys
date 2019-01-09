//! TODO: 不必要的 clone
use chrono::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    weeks: Vec<String>,
    directions: Vec<String>,
    tools: Vec<String>,
    var_names: Vec<String>,
    drinks: Vec<String>,
    activities: Vec<Activity>,
    specials: Vec<Special>,
}

#[derive(Debug, Deserialize)]
struct Special {
    data: String,
    r#type: String,
    name: String,
    description: String,
}

#[derive(Clone, Debug, Deserialize)]
struct Activity {
    name: String,
    good: String,
    bad: String,
    weekend: Option<bool>,
}

#[derive(Debug)]
pub struct TodayLuck {
    pub time: String,
    pub direction: String,
    pub drink: String,
    pub goddes: String,
    pub good: Vec<(String, String)>,
    pub bad: Vec<(String, String)>,
}

pub struct HuangLi {
    config: Config,
    today: DateTime<Local>,
    good: Vec<(String, String)>,
    bad: Vec<(String, String)>,
}

impl HuangLi {
    fn random(&self, indexseed: u32) -> u32 {
        let dayseed =
            self.today.year() as u32 * 10000 + self.today.month() * 100 + self.today.day();
        let mut n = dayseed % 11117;
        for _ in 0..indexseed + 100 {
            n *= n;
            n %= 11117;
        }
        n
    }

    fn random_choose<T: Clone>(&self, seed: u32, v: &[T]) -> T {
        v[self.random(seed) as usize % v.len()].clone()
    }

    fn pick_random_activities(&self, activities: Vec<Activity>, size: u32) -> Vec<Activity> {
        self.pick_random(&activities, size)
            .iter()
            .map(|&ac| self.parse(ac))
            .collect()
    }

    fn pick_random<'a, T>(&self, array: &'a [T], size: u32) -> Vec<&'a T> {
        let mut ret: Vec<&T> = array.iter().collect();
        for i in 0..ret.len() as u32 - size {
            ret.remove(self.random(i) as usize % ret.len());
        }
        ret
    }
}

impl HuangLi {
    pub fn load_from<P: AsRef<Path>>(path: P) -> Self {
        let mut file = File::open(path).expect("无法打开配置文件");
        let mut config_str = String::new();
        file.read_to_string(&mut config_str)
            .expect("无法读取配置文件");

        Self {
            config: toml::from_str(&config_str).unwrap(),
            today: Local::now(),
            good: vec![],
            bad: vec![],
        }
    }

    pub fn set_date(&mut self, date: DateTime<Local>) {
        self.today = date;
    }

    fn star(num: u32) -> String {
        "★".repeat(num as usize) + &"☆".repeat(5 - num as usize)
    }

    pub fn pick_today_luck(&mut self) -> TodayLuck {
        let activities = self
            .config
            .activities
            .iter()
            .filter(|ac| !self.is_weekend() || ac.weekend.is_some())
            .cloned()
            .collect::<Vec<_>>();
        let num_good = self.random(98) % 3 + 2;
        let num_bad = self.random(87) % 3 + 2;
        let event_attr = self.pick_random_activities(activities, num_good + num_bad);
        self.pick_specials();

        let (num_good, num_bad) = (num_good as usize, num_bad as usize);
        for event in &event_attr[..num_good] {
            self.good.push((event.name.clone(), event.good.clone()));
        }

        for event in &event_attr[num_good..num_good + num_bad] {
            self.bad.push((event.name.clone(), event.bad.clone()));
        }

        TodayLuck {
            time: self
                .today
                .format("今天是%Y年%m月%d日 星期")
                .to_string()
                + &self.config.weeks[self.today.weekday().num_days_from_sunday() as usize],
            direction: self.random_choose(2, &self.config.directions),
            drink: {
                let mut s = vec![];
                for i in self.pick_random(&self.config.drinks, 2) {
                    s.push(i as &str);
                }
                s.join("，")
            },
            goddes: Self::star(self.random(6) % 5 + 1),
            good: mem::replace(&mut self.good, vec![]),
            bad: mem::replace(&mut self.bad, vec![]),
        }
    }

    fn parse(&self, activity: &Activity) -> Activity {
        let mut activity = activity.clone();
        activity.name = activity
            .name
            .replace("%v", &self.random_choose(12, &self.config.var_names))
            .replace("%t", &self.random_choose(11, &self.config.tools))
            .replace("%l", &(self.random(12) % 247 + 30).to_string());
        activity
    }

    fn is_weekend(&self) -> bool {
        self.today.weekday().num_days_from_monday() >= 6
    }

    fn pick_specials(&mut self) {
        let iday = self.today.format("%Y%m%d").to_string();
        for special in &self.config.specials {
            if iday == special.data {
                if special.r#type == "good" {
                    self.good
                        .push((special.name.clone(), special.description.clone()));
                } else {
                    self.bad
                        .push((special.name.clone(), special.description.clone()));
                }
            }
        }
    }
}
