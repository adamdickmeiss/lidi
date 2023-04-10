use std::{io::Write, os::fd::AsRawFd};

use crate::receive::client;

pub(crate) fn start<C, F, E>(receiver: &super::Receiver<C, F, E>) -> Result<(), super::Error>
where
    C: Write + AsRawFd,
    F: Send + Sync + Fn() -> Result<C, E>,
    E: Into<super::Error>,
{
    loop {
        let (client_id, recvq) = receiver.for_clients.recv()?;

        log::debug!("try to acquire multiplex access..");
        receiver.multiplex_control.acquire();
        log::debug!("multiplex access acquired");

        let client_res = client::start(receiver, client_id, recvq);

        receiver.multiplex_control.release();

        if let Err(e) = client_res {
            log::error!("client {client_id:x}: send loop error: {e}");
        }
    }
}
