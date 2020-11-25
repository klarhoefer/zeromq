
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
        Socket { sck, addr: None }
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
    addr: Option<String>,
}

impl Socket {
    pub fn bind(&mut self, addr: &str) -> bool {
        let mut addr = addr.to_string();
        addr.push('\0');
        let rc = unsafe { zmq_bind(self.sck, addr.as_ptr()) };
        self.addr = Some(addr);
        rc == ZMQ_SUCCESS
    }

    pub fn unbind(&mut self) -> bool {
        if let Some(addr) = &self.addr {
            let rc = unsafe { zmq_unbind(self.sck, addr.as_ptr()) };
            self.addr = None;
            rc == ZMQ_SUCCESS
        } else {
            false
        }
    }

    pub fn connect(&mut self, addr: &str) -> bool {
        let mut addr = addr.to_string();
        addr.push('\0');
        let rc = unsafe { zmq_connect(self.sck, addr.as_ptr()) };
        self.addr = Some(addr);
        rc == ZMQ_SUCCESS
    }

    pub fn disconnect(&mut self) -> bool {
        if let Some(addr) = &self.addr {
            let rc = unsafe { zmq_disconnect(self.sck, addr.as_ptr()) };
            self.addr = None;
            rc == ZMQ_SUCCESS
        } else {
            false
        }
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

    use std::thread::sleep;
    use std::time::Duration;
    use std::str;
    
    fn sleep_ms(ms: u64) {
        sleep(Duration::from_millis(ms));
    }

    use super::{Context, SocketType, version, SendRecvOptions, SocketOptions};

    #[test]
    fn thread_a() {
        println!("Version {:?}", version());
        let ctx = Context::new();
        let mut sck = ctx.socket(SocketType::ZmqPub);
        if !sck.bind("tcp://*:5555") {
            panic!("Could not bind!");
        }
        println!("Ready!");
        sleep_ms(1000 * 2);
        println!("Publishing...");
        for i in 0..64 {
            let msg = format!("This is message #{}", i);
            sck.send(msg.as_bytes(), SendRecvOptions::ZmqWait);
        }
        sck.send("Done!".as_bytes(), SendRecvOptions::ZmqDontWait);
    }

    #[test]
    fn thread_b() {
        let ctx = Context::new();
        let mut sck = ctx.socket(SocketType::ZmqSub);
        sleep_ms(1000 * 1);
        if !sck.connect("tcp://localhost:5555") {
            panic!("Could not connect!");
        }
        sck.set_options(SocketOptions::ZmqSubscribe, None);
        println!("Receiving...");
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
