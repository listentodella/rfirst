struct SelfReference<'a> {
    name: String,
    name_ref: Option<&'a str>,
}

fn main() {
    let mut data = SelfReference {
        name: "John".to_string(),
        name_ref: None,
    };
    data.name_ref = Some(&data.name);

    // 如果move,或者mem::swap, 编译器不会让他通过
    //move_it(data);
}

#[allow(dead_code)]
fn move_it(data: SelfReference) -> SelfReference {
    data
}
