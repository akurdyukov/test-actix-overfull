extern crate actix;
extern crate futures;
extern crate tokio;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::collections::HashMap;
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

struct LocalActor {
    pub user_id: u64,
}

impl LocalActor {
    fn new(user_id: u64) -> Self {
        LocalActor {
            user_id,
        }
    }
}

impl Actor for LocalActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        info!("LocalActor for {:?} started", self.user_id);
    }
}

impl Handler<Command> for LocalActor {
    type Result = ();

    fn handle(&mut self, msg: Command, _: &mut Context<Self>) -> Self::Result {
        info!("Executing command {:?}!", msg);
    }
}


struct LocalSupervisor {
    user_to_address: HashMap<u64, Addr<LocalActor>>
}

impl LocalSupervisor {
    pub fn new() -> Self {
        LocalSupervisor {
            user_to_address: HashMap::new()
        }
    }

    fn get_child_addr(&mut self, user_id: u64) -> &Addr<LocalActor> {
        self.user_to_address.entry(user_id.clone())
            .or_insert_with(|| LocalActor::new(user_id).start() )
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

        let addr = self.get_child_addr(msg.user_id);
        addr.do_send(msg);
    }
}


fn main() {
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
 
    env_logger::Builder::from_env(env).init();

    System::run(move || {
        let root = LocalSupervisor::from_registry();

        for n in 0..1000 {
            root.do_send(Command::new(n % 5, n));
        }

        // wait forever
    });
}
