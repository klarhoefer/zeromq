
mod zeromq_sys;
use zeromq_sys::*;
pub use zeromq_sys::version;

pub enum SocketType {
    ZmqPair,
    ZmqPub,
    ZmqSub,
    ZmqReq,
    ZmqRep,
    ZmqDealer,
    ZmqRouter,
    ZmqPull,
    ZmqPush,
    ZmqXpub,
    ZmqXsub,
    ZmqStream,
}

impl From<SocketType> for i32 {
    fn from(sock_type: SocketType) -> i32 {
        sock_type as i32
    }
}


pub enum SocketOptions {
    ZmqSubscribe = 6,
}

impl From<SocketOptions> for i32 {
    fn from(opts: SocketOptions) -> i32 {
        opts as i32
    }
}


pub enum SendRecvOptions {
    ZmqWait,
    ZmqDontWait,
    ZmqSndMore,
}

impl From<SendRecvOptions> for i32 {
    fn from(opts: SendRecvOptions) -> i32 {
        opts as i32
    }
}

pub struct Context {
    ctx: ZmqContext,
}

impl Context {
    pub fn new() -> Self {
        let ctx = unsafe { zmq_ctx_new() };
        Context { ctx }
    }

    pub fn socket(&self, sock_type: SocketType) -> Socket {
        let sck = unsafe { zmq_socket(self.ctx, sock_type.into()) };
        Socket { sck }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            zmq_ctx_destroy(self.ctx);
        }
    }
}


pub struct Socket {
    sck: ZmqSocket,
}

fn add_zero(s: &str) -> Vec<u8> {
    let mut buffer = s.bytes().collect::<Vec<_>>();
    buffer.push(0);
    buffer
}

impl Socket {
    pub fn bind(&self, addr: &str) -> bool {
        let buffer = add_zero(addr);
        let rc = unsafe { zmq_bind(self.sck, buffer.as_ptr()) };
        rc == ZMQ_SUCCESS
    }

    pub fn connect(&self, addr: &str) -> bool {
        let buffer = add_zero(addr);
        let rc = unsafe { zmq_connect(self.sck, buffer.as_ptr()) };
        rc == ZMQ_SUCCESS
    }

    pub fn send(&self, buffer: &[u8], opts: SendRecvOptions) -> bool {
        let rc = unsafe { zmq_send(self.sck, buffer.as_ptr(), buffer.len(), opts.into()) };
        rc == ZMQ_SUCCESS
    }

    pub fn recv(&self, opts: SendRecvOptions) -> Option<Vec<u8>> {
        let mut buffer = [0u8; 4096];
        let count = unsafe { zmq_recv(self.sck, buffer.as_mut_ptr(), buffer.len(), opts.into()) };
        if count > 0 {
            let mut tmp = Vec::new();
            tmp.extend_from_slice(&buffer[..count as usize]);
            Some(tmp)
        } else {
            None
        }
    }

    pub fn set_options(&self, opts: SocketOptions, value: Option<&[u8]>) {
        let (ptr, len) = if let Some(value) = value {
            (value.as_ptr(), value.len())
        } else {
            (std::ptr::null(), 0)
        };
        unsafe {
            zmq_setsockopt(self.sck, opts.into(), ptr, len);
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe {
            zmq_close(self.sck);
        }
    }
}

#[cfg(test)]
mod tests {

    use std::thread::sleep_ms;
    use std::str;
    
    use super::{Context, SocketType, version, SendRecvOptions, SocketOptions};

    #[test]
    fn thread_a() {
        println!("Version {:?}", version());
        let ctx = Context::new();
        let sck = ctx.socket(SocketType::ZmqPub);
        if !sck.bind("tcp://*:5555") {
            panic!("Could not bind!");
        }
        sleep_ms(1000 * 4);
        println!("Publishing");
        for i in 0..512 {
            let msg = format!("This is message #{}", i);
            sck.send(msg.as_bytes(), SendRecvOptions::ZmqWait);
        }
        sck.send("Done!".as_bytes(), SendRecvOptions::ZmqDontWait);
    }

    #[test]
    fn thread_b() {
        let ctx = Context::new();
        let sck = ctx.socket(SocketType::ZmqSub);
        sleep_ms(1000 * 2);
        if !sck.connect("tcp://localhost:5555") {
            panic!("Could not connect!");
        }
        sck.set_options(SocketOptions::ZmqSubscribe, None);
        println!("Receiving");
        loop {
            if let Some(v) = sck.recv(SendRecvOptions::ZmqWait) {
                let s = str::from_utf8(&v).unwrap();
                println!("Received: {}", s);
                if s == "Done!" {
                    break;
                }
            } else {
                panic!("No message!");
            }
        }
    }
}
