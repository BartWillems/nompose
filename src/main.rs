extern crate structopt;
extern crate tera;
extern crate terminator;

#[macro_use]
extern crate rust_embed;

use std::fs::File;
use std::io::prelude::*;

use structopt::StructOpt;
use terminator::Terminator;

use cli::Output;

mod cli;
mod compose;
mod template;

fn main() -> Result<(), Terminator> {
    let opt = cli::Opt::from_args();

    let tpl = template::load_template()?;

    let compose_file: String = read_file(opt.compose_file.as_ref())?;

    let compose_struct: compose::Compose = serde_yaml::from_str(&compose_file)?;

    for (service_name, service) in &compose_struct.services {
        match template::generate_nomad_cfg(tpl.as_ref(), service_name, &service) {
            Ok(res) => match opt.output {
                Output::STDOUT => println!("{}", res),
                Output::STDERR => eprintln!("{}", res),
                Output::File => write_file(service_name, res.as_ref())?,
            },
            Err(e) => eprintln!("unable to print the nomad file for {}: {}", service_name, e),
        }
    }

    Ok(())
}

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_file(filename: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.nomad", filename))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
