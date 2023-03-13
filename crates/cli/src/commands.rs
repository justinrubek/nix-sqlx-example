#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    User(User),
}

#[derive(clap::Args, Debug)]
pub(crate) struct User {
    #[clap(subcommand)]
    pub command: UserCommands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum UserCommands {
    List,
    Add {
        #[arg()]
        name: String,
    },
}
