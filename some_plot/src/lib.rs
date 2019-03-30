use std::f32::consts::PI;
use std::fmt::Debug;
use std::io::{self, prelude::*};
use std::str::FromStr;
use svg::node::element::{Circle, Line, Path, Text};
use svg::node::{self, element::path::Data, Node};
use svg::Document;

pub fn read<T>() -> T
where
    T: FromStr,
    T::Err: Debug,
{
    let str = io::stdin()
        .bytes()
        .map(|b| b.unwrap() as char)
        .take_while(|&c| c != ' ' && c != '\n')
        .collect::<String>();
    str.trim().parse::<T>().unwrap()
}

/// 统计图
pub struct Plot {
    /// svg 文档
    document: Document,
    /// 中心 x 坐标
    x: f32,
    /// 中心 y 坐标
    y: f32,
    /// 统计图半径
    radius: f32,
    /// 坐标轴个数 (数据项数
    axis_cnt: f32,
}

impl Plot {
    pub fn new(x: f32, y: f32, radius: f32, axis_cnt: i32) -> Self {
        // 创建外层圆环
        let circle = Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", radius)
            .set("stroke", "black")
            .set("stroke-width", 0.5)
            .set("fill", "none");

        Self {
            document: Document::new()
                .set("viewBox", (0, 0, x * 2.0, y * 2.0))
                .add(circle),
            x,
            y,
            radius,
            axis_cnt: axis_cnt as f32,
        }
    }

    /// 添加节点
    pub fn add<T: Node>(&mut self, node: T) {
        let mut document = std::mem::replace(&mut self.document, Document::new());
        document = document.add(node);
        std::mem::replace(&mut self.document, document);
    }

    /// 在坐标轴上根据阈值增加分隔线
    pub fn add_threshold(&mut self, info: &[([f32; 2], [f32; 2], [&str; 3])]) {
        for (idx, info) in info.iter().enumerate() {
            // 计算坐标轴起点相对原点偏移量
            let x = self.radius * (2.0 * PI / self.axis_cnt * idx as f32).sin();
            let y = self.radius * (2.0 * PI / self.axis_cnt * idx as f32).cos();

            // 添加阈值点
            let ([min, max], [threshod1, threshod2], _) = info;
            let size1 = (threshod1 - min) / (max - min);
            let size2 = (threshod2 - min) / (max - min);

            for size in [size1, size2].iter() {
                let flag = Line::new()
                    .set("x1", self.x + x * size - 2.0)
                    .set("y1", self.y - y * size)
                    .set("x2", self.x + x * size + 2.0)
                    .set("y2", self.y - y * size)
                    .set("stroke", "black")
                    .set("stroke-width", 0.7)
                    .set(
                        "transform",
                        format!(
                            "rotate({}, {}, {})",
                            360 / self.axis_cnt as usize * idx,
                            self.x + x * size,
                            self.y - y * size
                        ),
                    );
                self.add(flag);
            }
        }
    }

    /// 根据给定的标签添加坐标轴
    pub fn add_axis(&mut self, labels: &[String]) {
        for (idx, label) in labels.iter().enumerate() {
            // 计算坐标轴起点相对原点偏移量
            let x = self.radius * (2.0 * PI / self.axis_cnt * idx as f32).sin();
            let y = self.radius * (2.0 * PI / self.axis_cnt * idx as f32).cos();

            // 添加坐标轴
            let line = Line::new()
                .set("x1", self.x + x)
                .set("y1", self.y + y)
                .set("x2", self.x)
                .set("y2", self.y)
                .set("stroke", "black")
                .set("stroke-width", 0.5);
            self.add(line);

            // 添加标签文字
            let text = Text::new()
                .set("x", self.x + x * 1.02 - 5.0)
                .set("y", self.y - y * 1.02)
                .set("font-size", 9)
                .add(node::Text::new(label.to_owned()));
            self.add(text);
        }
    }

    /// 添加一个扇形
    pub fn add_sector(&mut self, size: f32, nth: i32, color: &str) {
        let radius = self.radius * size;
        let (delta_x, delta_y) = (
            radius * (PI / self.axis_cnt).sin(),
            radius * (PI / self.axis_cnt).cos(),
        );

        let data = Data::new()
            .move_to((self.x, self.y))
            .line_to((self.x + delta_x, self.y - delta_y))
            .elliptical_arc_to((radius, radius, 0, 0, 0, self.x - delta_x, self.y - delta_y))
            .close();

        let path = Path::new()
            .set("d", data)
            .set(
                "transform",
                format!(
                    "rotate({}, {}, {})",
                    360 / self.axis_cnt as i32 * nth,
                    self.x,
                    self.y
                ),
            )
            .set("fill", color);

        self.add(path);
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        Ok(svg::save(path, &self.document)?)
    }
}
