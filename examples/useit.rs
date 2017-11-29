
#[macro_use]
extern crate stdcli;
#[macro_use]
extern crate log;

use stdcli::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Stuff {
    a: u8,
    b: String,
}

fn main() {
    init_log(3, true, None);

    trace!("hello");
    info!("hello this has a little more info\nand a newline");
    warn!("hello");
    error!("hello");
    println!("{:#?}", hashmap!{"hi" => "you", "bob" => "dole"});
}
