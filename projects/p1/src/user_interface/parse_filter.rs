use std::process::exit;

use super::super::FilterSubcommand;
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use polars::lazy::dsl::Expr;
use regex::Regex;

fn fn_parse_filter(filter: FilterSubcommand, default_tz: i32) -> Vec<Expr> {
    let masks: Vec<Expr> = vec![];
    match filter.id {
        Some(x) => masks.push(parse_int_filter(x)),
        None => (),
    }
    match filter.name {
        Some(x) => masks.push(parse_str_filter(x)),
        None => (),
    }
    match filter.value {
        Some(x) => masks.push(parse_float_filter(x)),
        None => (),
    }
    match filter.amount {
        Some(x) => masks.push(parse_int_filter(x)),
        None => (),
    }
    match filter.datetime {
        Some(x) => masks.push(parse_datetime_filter(x, default_tz)),
        None => (),
    }
    match filter.currency {
        Some(x) => masks.push(parse_str_filter(x)),
        None => (),
    }
    match filter.recipient {
        Some(x) => masks.push(parse_str_filter(x)),
        None => (),
    }
    match filter.situation {
        Some(x) => masks.push(parse_str_filter(x)),
        None => (),
    }
    masks
}

fn parse_int_filter(int_filter: String) -> Expr {
    let int_filter = int_filter.to_uppercase().replace(" ", "");
    match int_filter {
        s if Regex::new(r"[\d]+").unwrap().is_match(&s) => int_pattern_parser(int_filter),
        s if Regex::new(r"[!=+><]{1,2}[\d]+").unwrap().is_match(&s) => {
            sint_pattern_parser(int_filter)
        }
        _ => {
            println!("Invalid int filter");
            exit(0);
        }
    }
}

fn parse_str_filter(str_filter: String) -> Expr {
    let str_filter = str_filter.to_uppercase().replace(" ", "");
    match str_filter {
        s if Regex::new(r"\w+").unwrap().is_match(&s) => str_pattern_parser(str_filter),
        s if Regex::new(r"[=~!]{1,2}\w+").unwrap().is_match(&s) => sstr_pattern_parser(str_filter),
        _ => {
            println!("Invalid str filter");
            exit(0);
        }
    }
}

fn int_pattern_parser(int_filter: String) -> Expr {
    let int_filter = int_filter.parse::<i64>().unwrap();
    Expr::eq(col("id"), lit(int_filter))
}

fn sint_pattern_parser(int_filter: String) -> Expr {
    let re = Regex::new(r"([!=+><]{1,2})(\d+)").unwrap();
    let captures = re.captures(&int_filter).unwrap();
    let sign = captures.get(1).unwrap().as_str();
    let int_filter = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
    match sign {
        "=" => Expr::eq(col("id"), lit(int_filter)),
        "!=" => Expr::neq(col("id"), lit(int_filter)),
        ">" => Expr::gt(col("id"), lit(int_filter)),
        ">=" => Expr::gt_eq(col("id"), lit(int_filter)),
        "<" => Expr::lt(col("id"), lit(int_filter)),
        "<=" => Expr::lt_eq(col("id"), lit(int_filter)),
    }
}

fn str_pattern_parser(str_filter: String) -> Expr {
    Expr::eq(col("name"), lit(str_filter))
}

fn sstr_pattern_parser(str_filter: String) -> Expr {
    let re = Regex::new(r"([=~!]{1,2})([\w]+)").unwrap();
    let captures = re.captures(&str_filter).unwrap();
    let sign = captures.get(1).unwrap().as_str();
    let str_filter = captures.get(2).unwrap().as_str();
    match sign {
        "=" => Expr::eq(col("name"), lit(str_filter)),
        "!=" => Expr::neq(col("name"), lit(str_filter)),
        "~" => Expr::like(col("name"), lit(str_filter)),
        "!~" => Expr::not_like(col("name"), lit(str_filter)),
    }
}

fn parse_float_filter(float_filter: String) -> Expr {
    let float_filter = float_filter.to_uppercase().replace(" ", "");
    match float_filter {
        s if Regex::new(r"\d+\.\d+").unwrap().is_match(&s) => float_pattern_parser(float_filter),
        s if Regex::new(r"[!=+><]{1,2}\d+\.\d+").unwrap().is_match(&s) => {
            sfloat_pattern_parser(float_filter)
        }
        _ => {
            println!("Invalid float filter");
            exit(0);
        }
    }
}

fn float_pattern_parser(float_filter: String) -> Expr {
    let float_filter = float_filter.parse::<f64>().unwrap();
    Expr::eq(col("value"), lit(float_filter))
}

fn sfloat_pattern_parser(float_filter: String) -> Expr {
    let re = Regex::new(r"([!=+><]{1,2})([\d]+\.[\d]+)").unwrap();
    let captures = re.captures(&float_filter).unwrap();
    let sign = captures.get(1).unwrap().as_str();
    let float_filter = captures.get(2).unwrap().as_str().parse::<f64>().unwrap();
    match sign {
        "=" => Expr::eq(col("value"), lit(float_filter)),
        "!=" => Expr::neq(col("value"), lit(float_filter)),
        ">" => Expr::gt(col("value"), lit(float_filter)),
        ">=" => Expr::gt_eq(col("value"), lit(float_filter)),
        "<" => Expr::lt(col("value"), lit(float_filter)),
        "<=" => Expr::lt_eq(col("value"), lit(float_filter)),
    }
}

fn parse_datetime_filter(datetime_filter: String, default_tz: i32) -> Expr {
    let mut has_sign: bool;
    let mut datetime_str: String;
    let mut sign: String;
    match datetime_filter {
        s if Regex::new(r"[\d\- :\.+]+").unwrap().is_match(&s) => {
            let has_sign = false;
            let datetime_str = s;
        }
        s if Regex::new(r"[!=><]{1,2} *[\d\- :\.+]+")
            .unwrap()
            .is_match(&s) =>
        {
            let has_sign = true;
            let re = Regex::new(r"([!=+><]{1,2}) *([\d \-:\.+]+)").unwrap();
            let captures = re.captures(&s).unwrap();
            let sign = captures.get(1).unwrap().as_str();
            let datetime_str = captures.get(2).unwrap().as_str();
        }
        _ => {
            println!("Invalid datetime filter");
            exit(0);
        }
    }
    let datetime: DateTime<Utc>;
    match datetime_str {
        s if Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap().is_match(&s) => {
            let datetime = naive_str_to_datetime(datetime_str, "%Y-%m-%d", default_tz);
        }
        s if Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}")
            .unwrap()
            .is_match(&s) =>
        {
            let datetime = naive_str_to_datetime(datetime_str, "%Y-%m-%d %H:%M:%S", default_tz);
        }
        s if Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3}")
            .unwrap()
            .is_match(&s) =>
        {
            let datetime = naive_str_to_datetime(datetime_str, "%Y-%m-%d %H:%M:%S.%f", default_tz);
        }
        s if Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} [+\-]\d{2}:\d{2}")
            .unwrap()
            .is_match(&s) =>
        {
            let datetime = datetime_str_to_datetime(datetime_str, "%Y-%m-%d %H:%M:%S %z");
        }
        s if Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3} [+\-]\d{2}:\d{2}")
            .unwrap()
            .is_match(&s) =>
        {
            let datetime = datetime_str_to_datetime(datetime_str, "%Y-%m-%d %H:%M:%S%.3f %z");
        }
        _ => {
            println!("Invalid datetime filter");
            exit(0);
        }
    }
    if has_sign {
        sdatetime_pattern_parser(datetime, &sign)
    } else {
        datetime_pattern_parser(datetime)
    }
}

fn datetime_pattern_parser(datetime_filter: DateTime<Utc>) -> Expr {
    Expr::eq(col("datetime"), lit(datetime_filter.timestamp_millis()))
}

fn sdatetime_pattern_parser(datetime_filter: DateTime<Utc>, sign: &str) -> Expr {
    match sign {
        "=" => Expr::eq(col("datetime"), lit(datetime_filter.timestamp_millis())),
        "!=" => Expr::neq(col("datetime"), lit(datetime_filter.timestamp_millis())),
        ">" => Expr::gt(col("datetime"), lit(datetime_filter.timestamp_millis())),
        ">=" => Expr::gt_eq(col("datetime"), lit(datetime_filter.timestamp_millis())),
        "<" => Expr::lt(col("datetime"), lit(datetime_filter.timestamp_millis())),
        "<=" => Expr::lt_eq(col("datetime"), lit(datetime_filter.timestamp_millis())),
    }
}

fn naive_str_to_datetime(naive_str: String, pattern: &str, default_tz: i32) -> DateTime<Utc> {
    let offset = FixedOffset::east_opt(0).unwrap();
    let naive_dt = NaiveDateTime::parse_from_str(&naive_str, pattern).unwrap()
        - Duration::hours(default_tz.into());
    let dt_with_offset = offset.from_utc_datetime(&naive_dt);
    dt_with_offset.with_timezone(&Utc)
}

fn datetime_str_to_datetime(datetime_str: String, pattern: &str) -> DateTime<Utc> {
    let datetime = DateTime::parse_from_str(&datetime_str, pattern).unwrap();
    let utc_dt = datetime.with_timezone(&Utc);
    utc_dt
}

fn parse_currency_filter(currency_filter: String) -> Expr {
    let currency_filter = currency_filter.to_uppercase().replace(" ", "");
    match currency_filter {
        s if Regex::new(r"\w+").unwrap().is_match(&s) => currency_pattern_parser(currency_filter),
        s if Regex::new(r"[=~!]{1,2}\w+").unwrap().is_match(&s) => {
            scurrency_pattern_parser(currency_filter)
        }
        _ => {
            println!("Invalid currency filter");
            exit(0);
        }
    }
}

fn currency_pattern_parser(currency_filter: String) -> Expr {
    Expr::eq(col("currency"), lit(currency_filter))
}

fn scurrency_pattern_parser(currency_filter: String) -> Expr {
    let re = Regex::new(r"([=~!]{1,2})(\w+)").unwrap();
    let captures = re.captures(&currency_filter).unwrap();
    let sign = captures.get(1).unwrap().as_str();
    let currency_filter = captures.get(2).unwrap().as_str();
    match sign {
        "=" => Expr::eq(col("currency"), lit(currency_filter)),
        "!=" => Expr::neq(col("currency"), lit(currency_filter)),
        "~" => Expr::like(col("currency"), lit(currency_filter)),
        "!~" => Expr::not_like(col("currency"), lit(currency_filter)),
    }
}

fn parse_recipient_filter(recipient_filter: String) -> Expr {
    let recipient_filter = recipient_filter.to_uppercase().replace(" ", "");
    match recipient_filter {
        s if Regex::new(r"\w+").unwrap().is_match(&s) => recipient_pattern_parser(recipient_filter),
        s if Regex::new(r"[=~!]{1,2}\w+").unwrap().is_match(&s) => {
            srecipient_pattern_parser(recipient_filter)
        }
        _ => {
            println!("Invalid recipient filter");
            exit(0);
        }
    }
}

fn recipient_pattern_parser(recipient_filter: String) -> Expr {
    Expr::eq(col("recipient"), lit(recipient_filter))
}

fn srecipient_pattern_parser(recipient_filter: String) -> Expr {
    let re = Regex::new(r"([=~!]{1,2})(\w+)").unwrap();
    let captures = re.captures(&recipient_filter).unwrap();
    let sign = captures.get(1).unwrap().as_str();
    let recipient_filter = captures.get(2).unwrap().as_str();
    match sign {
        "=" => Expr::eq(col("recipient"), lit(recipient_filter)),
        "!=" => Expr::neq(col("recipient"), lit(recipient_filter)),
        "~" => Expr::like(col("recipient"), lit(recipient_filter)),
        "!~" => Expr::not_like(col("recipient"), lit(recipient_filter)),
    }
}

fn parse_situation_filter(situation_filter: String) -> Expr {
    let situation_filter = situation_filter.to_uppercase().replace(" ", "");
    match situation_filter {
        s if Regex::new(r"\w+").unwrap().is_match(&s) => situation_pattern_parser(situation_filter),
        s if Regex::new(r"[=~!]{1,2}\w+").unwrap().is_match(&s) => {
            ssituation_pattern_parser(situation_filter)
        }
        _ => {
            println!("Invalid situation filter");
            exit(0);
        }
    }
}

fn situation_pattern_parser(situation_filter: String) -> Expr {
    Expr::eq(col("situation"), lit(situation_filter))
}

fn ssituation_pattern_parser(situation_filter: String) -> Expr {
    let re = Regex::new(r"([=~!]{1,2})(\w+)").unwrap();
    let captures = re.captures(&situation_filter).unwrap();
    let sign = captures.get(1).unwrap().as_str();
    let situation_filter = captures.get(2).unwrap().as_str();
    match sign {
        "=" => Expr::eq(col("situation"), lit(situation_filter)),
        "!=" => Expr::neq(col("situation"), lit(situation_filter)),
        "~" => Expr::like(col("situation"), lit(situation_filter)),
        "!~" => Expr::not_like(col("situation"), lit(situation_filter)),
    }
}
