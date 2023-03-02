use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn project_detail(prjid: &str) {
    let home_url = format!("https://www.njhouse.com.cn/spf/sales?prjid={prjid}");
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

    let file_name = format!("{title}.csv");
    let csv_title =
        "楼盘名称, 入网总套数, 未售总套数, 总成交套数, 今日成交套数, 总认购套数, 今日认购套数";
    let inserted_line =  format!("{title}, {total}, {total_unsold}, {total_deal}, {deal_today}, {total_subscription}, {subscribe_today}");

    insert_or_create_csv(&file_name, csv_title, &inserted_line);
}

fn statistics() {
    let home_url = "https://www.njhouse.com.cn/data/index";

    let res = reqwest::blocking::get(home_url).unwrap();

    let html_text = res.text().unwrap();

    let html = scraper::Html::parse_document(&html_text);

    let csv_title = "区域, 年未售, 年认购, 年成交套数, 年成交面积, 日认购, 日成交套数, 日成交面积";

    let table_body = html.select(&selector("body > div.main > div.business_centers.datas_center > div.datas_main > div.datas_block.datas_block_first > div:nth-child(1) > table > tbody")).next().unwrap();
    let tr_selector = selector("tr");
    let items = table_body.select(&tr_selector);
    for item in items {
        let line = item
            .text()
            .map(|it| it.trim())
            .filter(|it| !it.is_empty())
            .collect::<Vec<_>>();

        let file_name = format!("{}.csv", line[0]);

        let line = line.join(", ");

        insert_or_create_csv(&file_name, csv_title, &line);
    }
}

fn insert_or_create_csv(file_name: &str, title: &str, line: &str) {
    let result_file_path = Path::new(&file_name);
    if !result_file_path.exists() {
        std::fs::write(result_file_path, format!("日期, {}", title)).unwrap();
    }

    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(result_file_path)
        .unwrap();

    let reader = BufReader::new(&file);
    let mut lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let today = chrono::Local::now().date_naive();
    let formatted_date = today.format("%Y-%m-%d").to_string();

    let line = format!("{}, {}", formatted_date, line);
    if lines.len() > 1 && lines[1].starts_with(&formatted_date) {
        lines[1] = line;
    } else {
        lines.insert(1, line);
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

fn main() {
    let args = std::env::args().skip(1);
    for home_url in args {
        project_detail(&home_url);
    }

    statistics();
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
