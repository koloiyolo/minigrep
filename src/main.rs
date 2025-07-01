use std::env;
use std::process;

use minigrep::config::Config;
use minigrep::run;

fn main() {
    /* Pre Chapter 13 */
    // let args: Vec<String> = env::args().collect();

    // let config: Config = match Config::build(args) {
    //     Ok(config) => config,
    //     Err(err) => {
    //         eprintln!("Problem parsing arguments: {err}");
    //         process::exit(1);
    //     },
    // };

    /* Alternatively */
    //
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    /* After chapter 13.3 */
    let config = match Config::build(env::args()) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        }
    };

    if let Err(e) = run(config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}
