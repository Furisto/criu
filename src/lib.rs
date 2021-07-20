pub mod rpc {
    include!(concat!(env!("OUT_DIR"), "/criu.rpc.rs"));
}

pub mod criu;

