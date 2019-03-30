use std::env;
use std::io::{self, prelude::*};
use some_plot::{read, Plot};

// ([最小值, 最大值], [阈值1, 阈值2], [颜色1, 颜色2, 颜色3)
// (最小值, 阈值1] -> 颜色1
// (阈值1, 阈值2] -> 颜色2
// (阈值2, 最大值] -> 颜色3
const DATA_INFO: [([f32; 2], [f32; 2], [&str; 3]); 10] = [
    // SLR
    ([3.0, 13.0], [4.99, 11.01], ["red", "green", "red"]),
    // TMOS-S
    ([3.0, 4.2], [3.8, 4.0], ["red", "yellow", "green"]),
    // RLR
    ([-7.0, -1.0], [-5.0, -3.0], ["red", "green", "yellow"]),
    // TMOS-RCV
    ([2.8, 4.2], [3.4, 3.8], ["red", "yellow", "green"]),
    // G-MOS
    ([3.6, 4.1], [3.8, 3.95], ["red", "yellow", "green"]),
    // S-MOS
    ([3.6, 4.1], [3.8, 3.95], ["red", "yellow", "green"]),
    // N-MOS
    ([3.8, 4.5], [4.0, 4.2], ["red", "yellow", "green"]),
    // DT_SEND
    ([1.0, 9.0], [2.99, 5.99], ["green", "yellow", "red"]),
    // TCLW
    ([50.0, 80.0], [65.0, 70.0], ["red", "yellow", "green"]),
    // delay
    ([0.0, 222.0], [149.99, 179.99], ["green", "yellow", "red"]),
];

fn main() {
    let labels = [
        "SLR", "TMOS-S", "RLR", "TMOS-RCV", "G-MOS", "S-MOS", "N-MOS", "DT_SEND", "TCLW", "delay",
    ];
    let mut argv = env::args()
        .skip(1)
        .map(|s| s.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();

    if argv.len() != 10 {
        argv = labels
            .iter()
            .map(|label| {
                print!("请输入 {}: ", label);
                io::stdout().flush().unwrap();
                read::<f32>()
            })
            .collect();
    }

    let mut plot = Plot::new(300.0, 300.0, 280.0, 10);

    for (idx, (value, info)) in argv.iter().zip(DATA_INFO.iter()).enumerate() {
        let ([min, max], [threshold1, threshold2], [color1, color2, color3]) = info;

        // 计算扇形大小
        assert_eq!(
            min <= value && value <= max,
            true,
            "不满足条件: {} <= {} <= {}",
            min,
            value,
            max
        );
        let size = (value - min) / (max - min);

        // 判断颜色
        let color = if value <= threshold1 {
            color1
        } else if value <= threshold2 {
            color2
        } else {
            color3
        };

        /* println!(
            "{:10} value:{:7.2} size:{:.2}  color:{}",
            labels[idx], value, size, color
        ); */
        plot.add_sector(size, idx as i32, color);
    }

    plot.add_axis(
        &labels
            .iter()
            .zip(argv.iter())
            .map(|(label, value)| format!("{}={}", label, value))
            .collect::<Vec<_>>(),
    );
    plot.add_threshold(&DATA_INFO);

    plot.save("image.svg").unwrap();
    println!("图片已保存为 image.svg");
}
