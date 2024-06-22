use sqlparser::dialect::Dialect;
use tracing::info;

#[derive(Debug, Default)]
pub struct DiyDialect;

// 创建自己的sql方言
// DiyDialect 支持 identifier 可以是简单的 url
impl Dialect for DiyDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '-'
    }

    // identifier 可以有 ':', '/', '?', '&', '='
    fn is_identifier_part(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            || ('0'..='9').contains(&ch)
            || [':', '/', '?', '&', '=', '-', '_', '.'].contains(&ch)
    }
}

pub fn example_sql() -> String {
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5",
        url
    );
    info!("get sql = {}", sql);

    sql
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlparser::parser::Parser;
    #[test]
    fn it_works() {
        assert!(Parser::parse_sql(&DiyDialect::default(), &example_sql()).is_ok());
    }
}
