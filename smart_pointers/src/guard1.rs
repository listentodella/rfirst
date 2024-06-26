use std::sync::Mutex;

fn main() {
    let m = Mutex::new(Mutex::new(233));
    let g = m.lock().unwrap();
    {
        rayon::join(
            || {
                let mut g1 = g.lock().unwrap();
                *g1 += 233;
                println!("g1: {}", *g1);
            },
            || {
                let mut g1 = g.lock().unwrap();
                *g1 += 233;
                println!("g1: {}", *g1);
            },
        );
    }
}
