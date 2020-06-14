// Goal for this program is to add domains to the postgres database
// The usage is expected to be `someProgramThatOutputsDomains > bin/shid addDomain -p program -s source

use structopt::StructOpt;

// Run different shid functions based on first arg
// addDomain - add domains to postgres from stdin, with values determined by flags / EnvVars // TODO
#[derive(StructOpt)]
struct Cli {
  // The function to run
  function: String,

  #[structopt(short = "p", long = "program")]
  program: String,

  #[structopt(short = "s", long = "source")]
  source: String
}

fn main() {
  let args = Cli::from_args();

  println!("Crystal Dolphins");
}
