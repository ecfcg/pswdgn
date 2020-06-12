use pswdgn::{cli, Generator};

fn main() {
    let cli = cli::CommandLine::parse(cli::build());
    let generator = match Generator::from_cli(cli.length, cli.flags, cli.is_easy, cli.symbols) {
        Ok(gen) => gen,
        Err(e) => panic!("{}", e),
    };
    let generated = generator.generate();
    println!("{}", generated);
}
