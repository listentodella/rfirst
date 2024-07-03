//use anyhow::Result;
use clap::Parser;
use colored::*;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Stdout, Write},
    ops::Range,
    path::Path,
};

mod error;
pub use error::RgrepError;

/// 定义类型, 这样, 在使用时可以简化复杂类型的书写
/// - R: 实现了 Read trait 的类型, 如File,Stdin,并且BufReader是具有内部缓冲区的,性能更好
/// - W: 实现了 Write trait 的类型, 如Stdout
pub type StrategyFn<W, R> = fn(&Path, BufReader<R>, &Regex, &mut W) -> Result<(), RgrepError>;

#[derive(Parser, Debug)]
#[command(name = "rgrep", version, author, about, long_about = None)]
//#[(setting = clap::AppSettings::ColoredHelp)]
pub struct RgrepConfig {
    /// 用于查找的正则表达式
    pattern: String,
    /// 文件通配符,借助 glob crate 可以枚举出符合的所有文件名
    glob: String,
}

impl RgrepConfig {
    /// 使用默认策略来查找匹配
    pub fn match_with_default_strategy(&self) -> Result<(), RgrepError> {
        self.match_with(default_strategy)
    }

    /// 使用某个策略来查找匹配
    pub fn match_with(&self, strategy: StrategyFn<Stdout, File>) -> Result<(), RgrepError> {
        let regex = Regex::new(&self.pattern)?;
        // 生成所有符合通配符的文件列表
        // 需要注意的是, glob 返回的是 Result<PathBuf, GlobError>
        let files = glob::glob(&self.glob)?.collect::<Vec<_>>();
        // 并行处理文件
        // 这里的 for_each 来自于 rayon crate, 它接收一个闭包, 并会并行执行
        files.into_par_iter().for_each(|v| {
            // 仅匹配Ok的v, 不匹配的v会被忽略(不过也是执行了,只是没有else分支)
            if let Ok(filename) = v {
                // 仅匹配Ok的File::open,失败的open并不是没有执行,而是走不到if里面的逻辑
                if let Ok(file) = File::open(&filename) {
                    // BufReader 是带有缓冲的
                    let reader = BufReader::new(file);
                    let mut stdout = io::stdout();
                    // 调用策略函数
                    if let Err(e) = strategy(filename.as_path(), reader, &regex, &mut stdout) {
                        println!("internal error: {:?}", e);
                    }
                }
            }
        });
        Ok(())
    }
}

/// 缺省策略，从头到尾串行查找,最后输出到writer
pub fn default_strategy<W: Write, R: Read>(
    path: &Path,
    reader: BufReader<R>,
    pattern: &Regex,
    writer: &mut W,
) -> Result<(), RgrepError> {
    let matches = reader
        //返回此 reader 的各行上的迭代器。
        //从这个函数返回的迭代器将产生 io::Result<String> 的实例
        //返回的每个字符串末尾都不会有换行符字节 (0xA 字节) 或 CRLF (0xD，0xA 字节)。
        .lines()
        //创建一个迭代器，该迭代器给出当前迭代次数以及下一个值。
        //返回的迭代器产生对 (i, val)，其中 i 是当前迭代索引，val 是迭代器返回的值。
        //针对这个用例, i 就是行号, val 是每行的内容
        .enumerate()
        .map(|(line_num, line)| {
            // 对于每一个正确含有内容的行, 调用map
            // ok()可以将Result<T, E>转换为Option<T>
            line.ok()
                // map闭包, 对于传入的行, 调用 Regex 的find方法
                // 对于匹配的行, 调用 format_line 函数, 格式化输出
                .map(|line| {
                    pattern
                        .find(&line)
                        .map(|m| format_line(&line, line_num + 1, m.range()))
                })
                //对于 Option<Option<T>> 这种嵌套的 Option
                //可以用flatten来消除多余的 Option
                //不过一次flatten只能消除一层Option
                .flatten()
        })
        //将消除到只剩一层Option的iterator,使用 filter_map 过滤出ok的项
        // ok_or() 将 Option<T> 转换为 Result<T, E>
        // 再用ok()将 Result<T, E> 转换为 Option<T>
        // filter_map 的闭包需要返回 Option<T>, 而filter_map产生的迭代器直接产生T
        // 如果直接拿迭代器取next(), 将得到Option<T>, 而如果直接用collect()或join()等聚合操作, 将得到T
        .filter_map(|v| v.ok_or(()).ok())
        .join("\n");

    if !matches.is_empty() {
        writer.write(path.display().to_string().green().as_bytes())?;
        writer.write(b"\n")?;
        writer.write(matches.as_bytes())?;
        writer.write(b"\n")?;
    }

    Ok(())
}

/// 格式化输出匹配的行，包含行号、列号和带有高亮的第一个匹配项
pub fn format_line(line: &str, line_num: usize, range: Range<usize>) -> String {
    let Range { start, end } = range;
    let prefix = &line[..start];
    format!(
        "{0: >6}:{1: <3} {2}{3}{4}",
        line_num.to_string().blue(),
        // 找到匹配项的起始位置,注意对汉字等非ascii字符,我们不能使用 prefix.len()
        // 这是一个O(n)的操作,会拖累效率
        (prefix.chars().count() + 1).to_string().cyan(),
        prefix,
        &line[start..end].red(),
        &line[end..]
    )
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn format_line_should_work() {
        let ret = format_line("Hello, Leo~", 1000, 7..10);
        let expected = format!(
            "{0: >6}:{1: <3} Hello, {2}~",
            "1000".blue(),
            "8".cyan(),
            "Leo".red()
        );
        assert_eq!(ret, expected);
    }

    #[test]
    fn default_strategy_should_work() {
        let path = Path::new("src/main.rs");
        let input = b"hello world!\nhey Leo!";
        let reader = BufReader::new(&input[..]);
        println!("{:?}", reader);
        let pattern = Regex::new(r"he\w+").unwrap();
        let mut writer = Vec::new();
        default_strategy(path, reader, &pattern, &mut writer).unwrap();
        println!("{:?}", writer);
        let ret = String::from_utf8(writer).unwrap();
        let expected = [
            String::from("src/main.rs"),
            format_line("hello world!", 1, 0..5),
            format_line("hey Leo!\n", 2, 0..3),
        ];

        assert_eq!(ret, expected.join("\n"));
    }
}
