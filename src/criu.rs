use std::fs::File;
use std::io::{Cursor, ErrorKind, Read};
use std::os::unix::prelude::FromRawFd;
use std::process::Command;
use std::time::Duration;
use crate::rpc::{CriuOpts, CriuReqType, CriuReq, CriuResp};

use anyhow::{Context, Result, bail};

use nix::unistd::{write};
use nix::sys::socket::{AddressFamily, SockFlag, SockType, socketpair};
use prost::Message;

pub struct Criu  {
    fd: Option<i32>,
    pub file: Option<File>,
}

impl Criu {

    pub fn new() -> Result<Self> {
        Ok(Self {
            fd: Some(Self::setup_rpc()?),
            file: None,
        })
    }

    fn setup_rpc() -> Result<i32> {
        let (fd1, fd2) = socketpair(AddressFamily::Unix, SockType::SeqPacket, None, SockFlag::empty())?;
        let criu_swrk = Command::new("criu")
        .arg("swrk")
        .arg(fd2.to_string())
        .spawn()
        .context("Could not start criu swrk")?;

        Ok(fd1)
    }

    pub fn dump(&self, options: CriuOpts) -> Result<()> {
       self.send(CriuReqType::Dump, options)
    }

    pub fn pre_dump(&self, options: &CriuOpts) -> Result<()> {
        todo!();
    }

    pub fn restore(&self, options: &CriuOpts) -> Result<()> {
        todo!();
    }

    pub fn start_page_server(&self, options: &CriuOpts) -> Result<()> {
        todo!();
    }

    pub fn get_criu_version(&self) -> Result<CriuVersion> {
        todo!();
    }

    fn send(&self, request_type: CriuReqType, options: CriuOpts) -> Result<()> {

        let mut request = CriuReq::default();
        request.r#type = request_type as i32;
        request.opts =  Some(options);
            
        let mut request_buf = Vec::with_capacity(request.encoded_len());
        request.encode(&mut request_buf)?;
        write(self.fd.unwrap(), &request_buf)?;
    
        let mut response_buf = Vec::with_capacity(2*4096);
        let mut f = unsafe {File::from_raw_fd(self.fd.unwrap())};
        loop {
            match f.read(&mut response_buf) {
                Ok(0) => eprintln!("read zero"),
                Ok(n) => {
                    eprintln!("read {:?}", n);
                    break;
                },
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => Err(e)?, 
            }
            std::thread::sleep(Duration::from_millis(2000));
        }
       
        let response = CriuResp::decode(&mut Cursor::new(response_buf))?;
        if !response.success {
            bail!("Request was not successfull: {:?}", response);
        }
   
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CriuVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}