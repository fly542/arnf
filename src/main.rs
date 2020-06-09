mod redis_client;
mod error;

use redis_client::RedisClient;

fn main() {
    //let addr = "127.0.0.1:6379";
    let addr = "abc6379";
    let conn_timeout = 0u64;
    let rw_timeout = 3u64;
    match RedisClient::new(addr, &conn_timeout, &rw_timeout) {
        Ok(stream) => {
            println!("connect ok from {}, peer={:?}", addr, stream.stream.peer_addr().unwrap());
        },
        Err(error) => { println!("connect faield from {} err={}", addr, error);
        },
    }
    println!("timout ={}", conn_timeout);
}
