// Goal for this program is to add domains to the postgres database
// The usage is expected to be `someProgramThatOutputsDomains > bin/shid addDomain -p program -s source

use structopt::StructOpt;
use postgres::{Client, NoTls};
use std::io::{self, BufRead, Write};

// Run different shid functions based on first arg
// TODO structopt has support for subcommands which I think I'm planning on using
// TODO I'll probably want to have better management of my postgres connections
// addDomain - add domains to postgres from stdin, with values determined by flags / EnvVars // TODO
// TODO move functions into their own files since this is planned to get pretty big
#[derive(StructOpt)]
struct Cli {
  // The function to run
  function: String,

  #[structopt(short = "p", long = "program")]
  program: String,

  #[structopt(short = "s", long = "source", default_value = "recon")]
  source: String,

  // Note: this is only used by getDomain and is more reason I should move to using the structOpt subcommand shiz
  #[structopt(short = "b", long = "scanned_before", default_value = "1970-01-01")]
  scanned_before: String
}

fn main() {
  let args = Cli::from_args();

  // Verify Program (which always exists thanks to structopt)
  verify_program(&args.program);

  match args.function.as_str() {
    "addDomain" => add_domains(args),
    "getDomain" => get_domains(args),
    _ => panic!("How exactly did this happen"),
  }
}

fn add_domains(args: Cli) {
  // Next steps
  // - decide on whether to enforce protocols (I'm thinking no right now? Can strip it off I think)
  // - add a unique constaint or checks if they exist for each domain
  // - get the program coming from env var
  // - have a default value for source

  let mut client = Client::connect("postgresql://postgres:docker@localhost:5432/shid", NoTls).unwrap();

  for line in io::stdin().lock().lines() {
      let host = line.unwrap();

      let rows = client.query("SELECT COUNT(id) FROM domains WHERE host = $1", &[&host]).unwrap();
      let row = &rows[0];
      let count: i64 = row.get(0);

      if count > 0 {
        println!("rejected: {}", &host);
        continue;
      }

      client.execute(r#"
      INSERT INTO domains
        (host, is_in_scope, are_subs_in_scope, source, created_at, updated_at, program_id)
      VALUES
        ($1, true, true, $2, now(), now(), (SELECT id
                                            FROM programs
                                            WHERE name = $3))
      "#, &[&host, &args.source, &args.program]).unwrap();
  }
}

fn get_domains(args: Cli) {
  let mut client = Client::connect("postgresql://postgres:docker@localhost:5432/shid", NoTls).unwrap();

  let rows = client.query(r#"
  SELECT host
  FROM domains
  WHERE
  program_id = (SELECT id FROM programs WHERE name = $1)
  AND
    (last_scanned_at < TO_TIMESTAMP($2, 'YYYY-MM-DD HH:MI:SS')
    OR
    last_scanned_at IS NULL)
  "#
  , &[&args.program, &args.scanned_before]).unwrap();

  let stdout = io::stdout();
  let mut handle = stdout.lock();

  for row in &rows {
    let value: &str = row.get(0);
    handle.write_all(value.as_bytes()).expect("Writing to stdout failed");
    handle.write_all(b"\n").expect("Writing to stdout failed");
  }
}

fn verify_program(program: &String) {
  let mut client = Client::connect("postgresql://postgres:docker@localhost:5432/shid", NoTls).unwrap();

  let rows = client.query("SELECT id FROM programs WHERE name = $1", &[program]).unwrap();

  if rows.len() == 0 {
    panic!("Invalid program name {}", program);
  }
}
