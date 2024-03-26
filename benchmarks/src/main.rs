pub mod random_border;

fn main() {
    println!("Running benchmarks...");
    let random_border_dur = random_border::bench(10, 5);

    println!("Rundom border average duration: {:.2?}", random_border_dur)
}
