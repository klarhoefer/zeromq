
mod zeromq_sys;
use zeromq_sys::*;

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

pub enum SendRecvOptions {
    ZmqStandard,
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
        let ret = unsafe { zmq_bind(self.sck, buffer.as_ptr()) };
        ret == ZMQ_SUCCESS
    }

    pub fn connect(&self, addr: &str) -> bool {
        let buffer = add_zero(addr);
        let ret = unsafe { zmq_bind(self.sck, buffer.as_ptr()) };
        ret == ZMQ_SUCCESS
    }

    pub fn send(&self, buffer: &[u8], opts: SendRecvOptions) -> bool {
        let ret = unsafe { zmq_send(self.sck, buffer.as_ptr(), buffer.len(), opts.into()) };
        ret == ZMQ_SUCCESS
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

    use super::{Context, SocketType};

    #[test]
    fn it_works() {
        let ctx = Context::new();
        let sck = ctx.socket(SocketType::ZmqPub);
        sck.bind("tcp://*:5555");
    }
}
