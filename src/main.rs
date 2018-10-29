extern crate actix;
extern crate futures;
extern crate tokio;

#[macro_use]
extern crate log;
extern crate env_logger;

use actix::prelude::*;

#[derive(Debug)]
struct Command {
    user_id: u64,
    payload: u64
}

impl Message for Command {
    type Result = ();
}

impl Command {
    fn new(user_id: u64, payload: u64) -> Self {
        Command {
            user_id,
            payload
        }
    }
}

struct LocalSupervisor {
}

impl LocalSupervisor {
    pub fn new() -> Self {
        LocalSupervisor {
        }
    }
}

impl Actor for LocalSupervisor {
    type Context = Context<Self>;
}

impl Supervised for LocalSupervisor {}

impl actix::SystemService for LocalSupervisor {}

impl Default for LocalSupervisor {
    fn default() -> LocalSupervisor {
        LocalSupervisor::new()
    }
}

impl Handler<Command> for LocalSupervisor {
    type Result = ();

    fn handle(&mut self, msg: Command, _: &mut Context<Self>) -> Self::Result {
        info!("Supervisor accepted validating command {:?}", msg);
    }
}


fn main() {
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
 
    env_logger::Builder::from_env(env).init();

    System::run(move || {
        let root = LocalSupervisor::new().start();
        // also does not work
        //let root = LocalSupervisor::from_registry();

        for n in 0..1000 {
            root.do_send(Command::new(n % 5, n));
        }

        // wait forever
    });
}
