use clap::Parser;

/// slocc, quick and dirty signifigant line of code and file counter
#[derive(Parser, Debug)]
#[clap(author, about, long_about = None)]
pub struct Args {
    /// directory that will be traversed
    #[clap(short, long)]
    pub directory: Option<String>,

    /// file name prefix to ignore
    #[clap(short, long)]
    pub prefix_ignore: Option<String>,

    /// file suffix to ignore
    #[clap(short, long)]
    pub suffix_ignore: Option<String>,

    /// extensions to operate on
    #[clap(short, long)]
    pub extensions: Option<Vec<String>>,

    /// true/false, count number of semicolons
    #[clap(short, long)]
    pub colons: Option<bool>,

    /// true/false, include hidden files
    #[clap(short, long)]
    pub hidden: Option<bool>,
}
