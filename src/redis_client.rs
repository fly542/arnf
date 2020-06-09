use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

use crate::error::AError;

#[allow(dead_code)]
pub struct RedisClient {
    pub stream: TcpStream,
    rw_timeout: u64,
}

impl RedisClient {

pub fn new(addr: &str, conn_timeout: &u64, rw_timeout: &u64) -> Result<RedisClient, AError> {
    if *conn_timeout > 0 {
        let server : SocketAddr = addr.parse()?;
        let stream = TcpStream::connect_timeout(&server,
            Duration::from_secs(*conn_timeout))?;
        Ok(RedisClient {stream, rw_timeout:*rw_timeout})
    } else {
        let stream = TcpStream::connect(addr)?;
        Ok(RedisClient { stream: stream, rw_timeout:*rw_timeout})
    }
}

}

