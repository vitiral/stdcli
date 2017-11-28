
#[macro_use]
extern crate stdcli as cli;

fn main() {
    cli::loggerv::
    info!("hello");
    warn!("hello");
    error!("hello");
    println!("{:#?}", hashmap!{"hi" => "you", "bob" => "dole"});
}
