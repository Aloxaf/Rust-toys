use colored::{ColoredString, Colorize};
use huang_li::HuangLi;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

// 提前折行, 这样可以防止 term_table 折行时导致颜色错乱
fn wrap_content<T, F>(data: T, width: usize, colorfn: F) -> String
where
    T: ToString,
    F: Fn(&str) -> ColoredString,
{
    let cell = TableCell::new(data);
    let lines = cell
        .wrapped_content(width)
        .iter()
        .map(|s| colorfn(s).to_string())
        .collect::<Vec<_>>();
    lines.join("\n")
}

fn count_line(s: &str) -> usize {
    s.chars().filter(|&c| c == '\n').count()
}

fn main() {
    let mut table = Table::new();
    table.max_column_width = 37;
    table.style = TableStyle::extended();

    let mut huangli = HuangLi::load_from("./src/config.toml");
    let today = huangli.pick_today_luck();

    let mut good = vec![];
    for g in &today.good {
        let mut s = format!("{}", g.0.bold().yellow());
        if !g.1.is_empty() {
            s.push_str(&wrap_content(
                format!("\n {}", g.1),
                table.max_column_width - 4,
                |s| s.bright_yellow(),
            ));
        }
        good.push(s);
    }

    let mut bad = vec![];
    for g in &today.bad {
        let mut s = format!("{}", g.0.bold().red());
        if !g.1.is_empty() {
            s.push_str(&wrap_content(
                format!("\n {}", g.1),
                table.max_column_width - 4,
                |s| s.bright_red(),
            ));
        }
        bad.push(s);
    }

    let (good, bad) = (good.join("\n\n"), bad.join("\n\n"));

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        "程序员老黄历".bold(),
        3,
        Alignment::Center,
    )]));
    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        today.time.bold(),
        3,
        Alignment::Center,
    )]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(
            format!(
                "{}{}",
                "\n".repeat(count_line(&good) / 2),
                "宜".bold().yellow()
            ),
            1,
            Alignment::Center,
        ),
        TableCell::new_with_col_span(good, 2),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(
            format!(
                "{}{}",
                "\n".repeat(count_line(&bad) / 2),
                "不宜".bold().red()
            ),
            1,
            Alignment::Center,
        ),
        TableCell::new_with_col_span(bad, 2),
    ]));
    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        format!(
            "{}: 面向{}写程序，BUG最少。",
            "座位朝向".bold(),
            today.direction.to_string().bold().green()
        ),
        3,
        Alignment::Center,
    )]));
    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        format!("{}: {}", "今日宜饮".bold(), today.drink),
        3,
        Alignment::Center,
    )]));
    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        format!(
            "{}: {}",
            "女神亲近指数".bold(),
            today.goddes.bright_red()
        ),
        3,
        Alignment::Center,
    )]));

    println!("{}", table.render());
}
