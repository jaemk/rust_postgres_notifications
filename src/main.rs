extern crate postgres;
extern crate fallible_iterator;

use fallible_iterator::{FallibleIterator, IntoFallibleIterator};
use std::env;
use postgres::{Connection, TlsMode};

/// Start up a 'listener' with `cargo run`. Then send(insert) or update
/// rows using `cargo run send` / `cargo run update` from a separate shell.
pub fn main() {
    let arg  = env::args().nth(1).unwrap_or("listen".to_string());

    let conn = Connection::connect("postgres://james:enter@localhost", TlsMode::None).unwrap();
    if arg == "send" {
        conn.execute("insert into events (event) values ($1)", &[&"testing".to_string()]).unwrap();
        for row in &conn.query("select id, event from events", &[]).unwrap() {
            let id: i32 = row.get(0);
            let event: String = row.get(1);
            println!("id: {:?}, event: {:?}", id, event);
        }
    } else if arg == "update" {
        conn.execute("update events set event='wow!' where id = 1", &[]).unwrap();
    } else {
        println!("** Listening on 'events' and 'messages'");
        // Listen on arbitrary 'chan'. The psql trigger will communicate
        // over 'events' (the name of the table it is applied to).
        // You can test other channels from the psql shell: `NOTIFY messages, 'hello!';`
        conn.execute("listen events", &[]).unwrap();
        conn.execute("listen messages", &[]).unwrap();
        let mut notifs = conn.notifications();
        loop {
            let a = notifs.blocking_iter().next();
            println!("{:?}", a);
        }
    }
}
