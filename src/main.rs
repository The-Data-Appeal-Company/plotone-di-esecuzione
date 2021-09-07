use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

use argh::FromArgs;
use crossbeam_channel::{Receiver, unbounded};
use postgres::{Client, NoTls};

#[derive(FromArgs)]
/// Execute parallel query on postgres.
struct Configuration {
    /// db port. default: 5432
    #[argh(option, default = "5432", short = 'P')]
    port: u16,
    /// db host
    #[argh(option, short = 'h')]
    host: String,
    /// db name
    #[argh(option, short = 'n')]
    name: String,
    /// db username
    #[argh(option, short = 'u')]
    user: String,
    /// db password
    #[argh(option, short = 'p')]
    password: String,
    /// number of parallel queries. default: 5
    #[argh(option, default = "5", short = 'c')]
    concurrency: usize,
    /// queries file path
    #[argh(option, short = 'i')]
    input: String,
}

struct Consumer {
    channel: Receiver<String>,
    pg_client: Client,
}

fn main() {
    let conf: Configuration = argh::from_env();

    let (s, r) = unbounded();

    let filename = conf.input.clone();
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let my_s = s.clone();
        my_s.send(line).unwrap();
    }

    let mut consumers = Vec::new();
    for _id in 0..conf.concurrency {
        consumers.push(Consumer {
            channel: r.clone(),
            pg_client: get_connection(&conf),
        });
    }

    let mut children = Vec::new();
    for mut consumer in consumers {
        let child = thread::spawn(move || {
            while !consumer.channel.is_empty() {
                match consumer.channel.recv() {
                    Ok(sql) => {
                        match consumer.pg_client.execute(sql.as_str(), &[]) {
                            Ok(_) => println!("executed: {}", sql),
                            Err(e) => eprintln!("error executing: {}, error: {}", sql, e)
                        }
                    }
                    Err(e) => {
                        eprintln!("cannot receive sql: {}", e)
                    }
                }
            }
        });

        children.push(child);
    }

    for c in children {
        c.join().expect("could not join thread")
    }
}

fn get_connection(conf: &Configuration) -> Client {
    Client::connect(
        format!("host={} port={} dbname={} user={} password={}",
                conf.host, conf.port, conf.name, conf.user, conf.password).as_str(),
        NoTls).expect("cannot connect to db")
}
