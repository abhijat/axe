use structopt::StructOpt;

use crate::operations::parse_json;
use crate::options::Options;

mod structures;
mod options;
mod operations;


fn main() {
    let options: Options = Options::from_args();
    parse_json(&options);
}
