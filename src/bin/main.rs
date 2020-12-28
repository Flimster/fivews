use fivewsdb::db::*;

use structopt::StructOpt;

const DB_PATH: &str = "./lidb";

#[derive(StructOpt, Debug)]
#[structopt(name = "cli")]

struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Read {
        pattern: String,
    },
    Write {
        #[structopt(default_value = "")]
        who: String,
        #[structopt(default_value = "")]
        what: String,
        #[structopt(default_value = "")]
        r#where: String,
        #[structopt(default_value = "")]
        why: String,
        when: Option<String>,
    },
}

fn main() {
    let opt = Opt::from_args();
    let mut db = FiveWsDB::new(DB_PATH);

    match opt.cmd {
        Command::Write {
            who,
            what,
            r#where,
            why,
            when,
        } => {
            let time;
            if let Some(w) = when {
                time = w;
            } else {
                time = chrono::Utc::now().to_string();
            }
            match db.update(who, what, time, r#where, why) {
                Err(e) => eprintln!("{}", e.to_string()),
                _ => {}
            };
        }
        Command::Read { pattern } => {
            println!("Who | What | When | Where | Why");
            for e in db.read(&pattern) {
                println!("{}", e.to_string());
            }
        }
    }
}
