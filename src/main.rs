use std::{fs::{self, File}, io::stdin, os::unix::prelude::AsRawFd};

use criu::{criu::Criu, rpc};
use anyhow::Result;

fn main() -> Result<()> {
    let image_dir = "/tmp/criu/image_dir";
    fs::create_dir_all(image_dir)?;
    let image_dir = File::open(image_dir)?;

    let mut options = rpc::CriuOpts::default();
    options.images_dir_fd = image_dir.as_raw_fd();
    options.pid = Some(121366);
    options.log_level = Some(4);
    options.log_file = Some("dump.log".to_owned());

    let criu = Criu::new()?;
    criu.dump(options)?;

    Ok(())
}
