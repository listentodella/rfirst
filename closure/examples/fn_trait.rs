// 定义一个 trait Executor，它有一个方法 execute，接收一个字符串参数，返回一个 Result<String, &'static str>
pub trait Executor {
    fn execute(&self, cmd: &str) -> Result<String, &'static str>;
}

// 定义一个结构体 BashExecutor，它有一个字段 env，类型为 String
struct BashExecutor {
    env: String,
}

// 为 BashExecutor 实现 Executor trait
impl Executor for BashExecutor {
    fn execute(&self, cmd: &str) -> Result<String, &'static str> {
        Ok(format!(
            "fake bash execute: env: {}, cmd: {}",
            self.env, cmd
        ))
    }
}

// 为 Fn(&str) -> Result<String, &'static str> 实现 Executor trait
impl<F> Executor for F
where
    F: Fn(&str) -> Result<String, &'static str>,
{
    fn execute(&self, cmd: &str) -> Result<String, &'static str> {
        self(cmd)
    }
}

fn main() {
    let env = "PATH=/usr/bin".to_string();
    let cmd = "cat /etc/passwd";

    let r1 = execute(cmd, BashExecutor { env: env.clone() });
    println!("{:?}", r1);

    let r2 = execute(cmd, |cmd: &str| {
        Ok(format!("fake fish execute: env: {}, cmd: {}", env, cmd))
    });
    println!("{:?}", r2);
}

// 定义一个函数 execute，接收一个字符串参数 cmd 和一个实现了 Executor trait 的参数 exec
// 返回一个 Result<String, &'static str>
fn execute(cmd: &str, exec: impl Executor) -> Result<String, &'static str> {
    exec.execute(cmd)
}
