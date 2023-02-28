use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let Some(home_url) = args.get(1) else {
        panic!("Please provide home url");
    };

    let res = reqwest::blocking::get(home_url).unwrap();

    let html_text = res.text().unwrap();

    let html = scraper::Html::parse_document(&html_text);

    let title_path =
        "body > div.main > div.business_centers > div > div.spf_del_title.clearfix > h2";
    let total_path = "body > div.main > div.business_centers > div > div.spf_del_block > div > div > table > tbody > tr:nth-child(1) > td:nth-child(2)";
    let subscribe_today_path = "body > div.main > div.business_centers > div > div.spf_del_block > div > div > table > tbody > tr:nth-child(1) > td:nth-child(4)";
    let deal_today_path = "body > div.main > div.business_centers > div > div.spf_del_block > div > div > table > tbody > tr:nth-child(2) > td:nth-child(4)";
    let total_unsold_path = "body > div.main > div.business_centers > div > div.spf_del_block > div > div > table > tbody > tr:nth-child(3) > td:nth-child(2)";
    let total_subscription_path = "body > div.main > div.business_centers > div > div.spf_del_block > div > div > table > tbody > tr:nth-child(3) > td:nth-child(4)";
    let total_deal_path = "body > div.main > div.business_centers > div > div.spf_del_block > div > div > table > tbody > tr:nth-child(5) > td:nth-child(4)";

    let title = parse_path_text(&html, title_path);
    let total = parse_path_num(&html, total_path);
    let subscribe_today = parse_path_num(&html, subscribe_today_path);
    let deal_today = parse_path_num(&html, deal_today_path);
    let total_unsold = parse_path_num(&html, total_unsold_path);
    let total_subscription = parse_path_num(&html, total_subscription_path);
    let total_deal = parse_path_num(&html, total_deal_path);
    let today = chrono::Local::now().date_naive();
    let formatted_date = today.format("%Y-%m-%d").to_string();

    let file_name = format!("{title}.csv");
    let result_file_path = Path::new(&file_name);
    if !result_file_path.exists() {
        std::fs::write(
            result_file_path,
            "日期, 楼盘名称, 入网总套数, 未售总套数, 总成交套数, 今日成交套数, 总认购套数, 今日认购套数",
        )
        .unwrap();
    }

    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(result_file_path)
        .unwrap();

    let reader = BufReader::new(&file);
    let mut lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let inserted_line =  format!("{formatted_date}, {title}, {total}, {total_unsold}, {total_deal}, {deal_today}, {total_subscription}, {subscribe_today}");

    if lines.len() > 1 && lines[1].starts_with(&formatted_date) {
        lines[1] = inserted_line;
    } else {
        lines.insert(1, inserted_line);
    }

    let file = std::fs::OpenOptions::new()
        .write(true)
        .open(result_file_path)
        .unwrap();
    let mut writer = BufWriter::new(&file);
    for line in lines {
        writeln!(writer, "{}", line).unwrap();
    }

    writer.flush().unwrap();
}

fn parse_path_num(html: &scraper::Html, path: &str) -> f32 {
    parse_selector_num(html, &selector(path))
}

fn parse_path_text(html: &scraper::Html, path: &str) -> String {
    parse_selector_text(html, &selector(path))
}

fn selector(path: &str) -> scraper::Selector {
    scraper::Selector::parse(path).unwrap()
}

fn parse_selector_num(html: &scraper::Html, selector: &scraper::Selector) -> f32 {
    parse_selector_text(html, selector)
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<f32>()
        .unwrap()
}

fn parse_selector_text(html: &scraper::Html, selector: &scraper::Selector) -> String {
    html.select(selector)
        .next()
        .unwrap()
        .text()
        .collect::<Vec<_>>()
        .join("")
}
