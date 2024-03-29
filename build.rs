use clap::CommandFactory;
use clap_complete::{
    generate_to,
    Shell::{Bash, Fish, Zsh},
};
use clap_mangen::{self, Man};
use std::{env, fs};

// Include Args struct.
include!("src/args.rs");

fn main() {
    // Generate man & completions directories.
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("unable to determine manifest dir"));
    let man_dir = manifest_dir.join("man");
    let comp_dir = manifest_dir.join("completions");

    fs::create_dir_all(&man_dir).expect("unable to create man directory");
    fs::create_dir_all(&comp_dir).expect("unable to create completions directory");

    // Retrieve Args and set binary name.
    let mut cmd = Args::command();
    cmd.set_bin_name("datalogger");

    // Generate & write man page.
    let mut buffer: Vec<u8> = Vec::new();
    Man::new(cmd.clone())
        .render(&mut buffer)
        .expect("unable to generate man page");
    fs::write(man_dir.join("datalogger.1"), buffer).expect("unable to write man page");

    // Generate shell completions.
    for shell in [Bash, Fish, Zsh] {
        generate_to(shell, &mut cmd, "datalogger", &comp_dir)
            .expect("unable to generate completions");
    }
}
