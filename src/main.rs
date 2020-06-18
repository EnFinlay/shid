// Goal for this program is to add domains to the postgres database
// The usage is expected to be `someProgramThatOutputsDomains > bin/shid addDomain -p program -s source

use structopt::StructOpt;
use postgres::{Client, NoTls};
use std::io::{self, BufRead};

// Run different shid functions based on first arg
// TODO structopt has support for subcommands which I think I'm planning on using
// TODO I'll probably want to have better management of my postgres connections
// addDomain - add domains to postgres from stdin, with values determined by flags / EnvVars // TODO
#[derive(StructOpt)]
struct Cli {
  // The function to run
  function: String,

  #[structopt(short = "p", long = "program")]
  program: String,

  #[structopt(short = "s", long = "source", default_value = "recon")]
  source: String
}

fn main() {
  let args = Cli::from_args();

  // Verify Program (which always exists thanks to structopt)
  verify_program(&args.program);

  if args.function == "addDomain" {
    add_domain(args);
  }

  println!("Crystal Dolphins");
}

fn add_domain(args: Cli) {
  // Next steps
  // - decide on whether to enforce protocols (I'm thinking no right now? Can strip it off I think)
  // - add a unique constaint or checks if they exist for each domain
  // - get the program coming from env var
  // - have a default value for source

  let mut client = Client::connect("postgresql://postgres:docker@localhost:5432/shid", NoTls).unwrap();

  for line in io::stdin().lock().lines() {
      client.execute(r#"
      INSERT INTO domains
        (host, is_in_scope, are_subs_in_scope, source, created_at, updated_at, program_id)
      VALUES
        ($1, true, true, $2, now(), now(), (SELECT id
                                            FROM programs
                                            WHERE name = $3))
      "#, &[&line.unwrap(), &args.source, &args.program]).unwrap();
  }
}

fn verify_program(program: &String) {
  let mut client = Client::connect("postgresql://postgres:docker@localhost:5432/shid", NoTls).unwrap();

  let rows = client.query("SELECT id FROM programs WHERE name = $1", &[program]).unwrap();

  if rows.len() == 0 {
    panic!("Invalid program name {}", program);
  }
}
