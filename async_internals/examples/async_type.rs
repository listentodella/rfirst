fn main() {
    let fut = async { 42 };

    println!("type of fut: {}", get_type_name(&fut));
    println!("type of hello fut is {}", get_type_name(&hello("Leo")));
}

fn get_type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

async fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}
