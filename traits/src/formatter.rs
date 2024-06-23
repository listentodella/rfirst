pub trait Formatter {
    fn format(&self, input: &mut String) -> bool;
}

struct MarkdownFormatter;
impl Formatter for MarkdownFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("do format for Markdown");
        true
    }
}

struct RustFormatter;
impl Formatter for RustFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("do format for Rust");
        true
    }
}

struct HtmlFormatter;
impl Formatter for HtmlFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("do format for HTML");
        true
    }
}

fn format(input: &mut String, formmaters: Vec<&dyn Formatter>) {
    for formatter in formmaters {
        formatter.format(input);
    }
}

fn main() {
    let mut text = "hello world".to_string();
    let html = &HtmlFormatter;
    let rust = &RustFormatter;
    let markdown = &MarkdownFormatter;

    // 这里要么在定义vec的时候强行转换成trait对象
    // 要么在定义变量本身的时候或标注的时候,用trait对象表示
    // 因为vec只接受同类型的变量
    let formatters = vec![
        html as &dyn Formatter,
        rust as &dyn Formatter,
        markdown as &dyn Formatter,
    ];

    format(&mut text, formatters);

    println!("formatted text: {}", text);
}
