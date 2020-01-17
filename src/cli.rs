use core::str::FromStr;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Output {
    #[structopt(name = "file")]
    File,
    #[structopt(name = "stdout")]
    STDOUT,
    #[structopt(name = "stderr")]
    STDERR,
}

impl FromStr for Output {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_ref() {
            "file" => Ok(Output::File),
            "stdout" => Ok(Output::STDOUT),
            "stderr" => Ok(Output::STDERR),
            &_ => Err(String::from("Invalid output format")),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "nompose",
    about = "Convert docker-compose files to nomad files."
)]
pub struct Opt {
    /// Output type: stdout, stderr or file
    #[structopt(name = "output", short, long, default_value = "stdout")]
    pub output: Output,
    /// Compose File: docker-compose file name
    #[structopt(
        name = "docker-compose-file",
        short = "-i",
        default_value = "docker-compose.yaml"
    )]
    pub compose_file: String,
}
