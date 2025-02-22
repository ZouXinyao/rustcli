// rcli -- csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rustcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    // println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
