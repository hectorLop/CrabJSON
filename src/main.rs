mod args;

fn main() {
    let args: Vec<String> = args::read_args();
    dbg!(args);
}
