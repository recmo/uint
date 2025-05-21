mod benches;
mod prelude;

fn main() {
    let mut c = criterion::Criterion::default().configure_from_args();
    benches::group(&mut c);
    c.final_summary();
}
