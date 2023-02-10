use clap::{
    Args,
    Parser,
    Subcommand
};

#[derive(Parser)]
#[clap(author, version, allow_hyphen_values(true))]
pub struct ScrambledArgs {
    /// the scramble to solve
    pub scramble: String,
    /// leave empty to return single longest word
    #[command(subcommand)]
    pub response_type: Option<ResponseType>
}

#[derive(Subcommand, PartialEq)]
pub enum ResponseType {
    /// outputs all the words to the file specified
    #[command(short_flag = 'f', long_flag = "file")]
    File(File),
    /// outputs all the words to the terminal
    #[command(short_flag = 'l', long_flag = "list")]
    List(List),
}

#[derive(Args, PartialEq)]
pub struct File {
    /// the file to output the words to
    pub file_path: String,
    #[arg(long = "output")]
    pub max_output: Option<i32>,
    #[arg(long = "min_length")]
    pub min_length: Option<i32>,
}

#[derive(Args, PartialEq)]
pub struct List {
    #[arg(long = "output")]
    pub max_output: Option<i32>,
    #[arg(long = "min_length")]
    pub min_length: Option<i32>,
}
