use std::{error::Error, process::Command};

// 定义类型别名
// 需要注意的是,该类型被Box封装,意味着它分配在堆上
// 另外Errory是一个trait, 这意味着BoxedError的实例也是trait object
pub type BoxedError = Box<dyn Error + Send + Sync>;

pub trait Executor {
    fn run(&self) -> Result<Option<i32>, BoxedError>;
}

pub struct Shell<'a, 'b> {
    cmd: &'a str,
    args: &'b [&'a str],
}

impl<'a, 'b> Shell<'a, 'b> {
    fn new(cmd: &'a str, args: &'b [&'a str]) -> Self {
        Self { cmd, args }
    }
}

impl<'a, 'b> Executor for Shell<'a, 'b> {
    fn run(&self) -> Result<Option<i32>, BoxedError> {
        let output = Command::new(self.cmd).args(self.args).output()?;
        Ok(output.status.code())
    }
}

/// 使用泛型参数
pub fn execute_generics(cmd: &impl Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

/// 使用trait object:&dyn T
pub fn execute_trait_object(cmd: &dyn Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

/// 使用trait object:Box<dyn T>
pub fn execute_boxed_trait_object(cmd: Box<dyn Executor>) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_shall_work() {
        let cmd = Shell::new("ls", &[]);
        let ret = cmd.run().unwrap();
        assert_eq!(ret, Some(0));
    }

    #[test]
    fn execute_shall_work() {
        let cmd = Shell::new("ls", &[]);

        let ret = execute_generics(&cmd).unwrap();
        assert_eq!(ret, Some(0));
        let ret = execute_trait_object(&cmd).unwrap();
        assert_eq!(ret, Some(0));

        let boxed = Box::new(cmd);
        let ret = execute_boxed_trait_object(boxed).unwrap();
        assert_eq!(ret, Some(0));
    }
}
