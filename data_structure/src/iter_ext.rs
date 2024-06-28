use itertools::Itertools;

fn main() {
    let err_str = "bad happened";
    let input = vec![Ok(21), Err(err_str), Ok(7)];
    let it = input
        //.iter() // error
        .into_iter()
        .filter_map_ok(|x| if x > 10 { Some(x * 2) } else { None });

    println!("{:?}", it.collect::<Vec<_>>());
}
