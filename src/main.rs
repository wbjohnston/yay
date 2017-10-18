//! Clone of the unix coreutil `yes`

use std::io::{stdout, Write};
use std::env::args as argv;
use std::process;

const STDOUT_BUFFER_SIZE: usize = 8096; // No idea if this is the right size
const DEFAULT_PHRASE: &str   = "y";

const VERSION_FLAG: &str        = r#"--version"#;
const VERSION_STR: &str         = r#""#;

const HELP_FLAG: &str           = r#"--help"#;
const HELP_STR: &str            = r#"
Usage: yes [STRING]...
   or: yes OPTION

Repeatedly output a line with all specified STRING(s), or 'y'.

    --help      display this help and exit
    --version   output version information and exit
"#;

fn main() {
    let args: Vec<_> = argv().collect();

    let phrase: String = if args.len() > 1 {
        match args[1].as_str() {
            VERSION_FLAG => {
                println!("{}", VERSION_STR);
                process::exit(0);
            },
            HELP_FLAG => {
                println!("{}", HELP_STR);
                process::exit(0);
            }
            _ => {}
        };

        args[1..].join(" ")
    } else {
        DEFAULT_PHRASE.into()
    };

    let buffer_size = STDOUT_BUFFER_SIZE.max(phrase.len());

    // pull out a buffers worth of characters. This is not correct rn
    // as string can be cut off yes -> ye
    let ys: String = std::iter::repeat(phrase + "\n")
        .take(buffer_size)
        .collect();

    // acquire lock on stdout and hooOOooolld on
    let out = stdout();
    let mut lock = out.lock();

    loop { // unleash the beast
        write!(lock, "{}", ys)
            .unwrap_or_else(|e| {
                println!("ERROR: {:?}", e);
                process::exit(1);
             });
    }
}
