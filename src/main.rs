use clap::Parser;
use rcli::{process_csv, process_gen_pass, Opts, SubCommand};
use zxcvbn::zxcvbn;

// rcli csv i input.csv -o output.json --header -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            let password = process_gen_pass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            );
            let estimate = zxcvbn(&password, &[]);
            println!("Generate password: {}", password);
            eprintln!("Password strength: {}", estimate.score());
        }
    }

    Ok(())
}
