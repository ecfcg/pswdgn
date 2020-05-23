use pswdgn::{cli, Generator};

fn main() {
    let command_line = cli::CommandLine::parse(cli::build());
    let generated = Generator::from_cli(command_line).generate();
    println!("{}", generated);
}
