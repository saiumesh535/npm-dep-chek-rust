use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "1.0", author = "saiumesh. <d.sai535@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short = 's', long = "src", about = "path to source directory")]
    pub source: String,

    #[clap(short = 'p', long = "package", about = "path to package.json")]
    pub package: String,
}

pub fn parse_cli() -> Opts {
    let opts: Opts = Opts::parse();
    opts
}
