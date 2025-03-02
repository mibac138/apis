use structopt::StructOpt;

const PROGRAM_NAME: &str = "mcp";

mod cmds;
/// taken from share-secrets-safely/tools
mod options;

use options::Args;
use options::SubCommand::*;

#[paw::main]
fn main(args: Args) {
    let res = match args.cmd {
        Completions(args) => cmds::completions::generate(Args::clap(), args),
        FetchSpecs(args) => cmds::fetch_specs::execute(args),
        Substitute(args) => cmds::substitute::execute(args),
    };
    failure_tools::ok_or_exit(res);
}
