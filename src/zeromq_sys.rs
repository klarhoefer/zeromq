#![allow(dead_code)]


use std::os::raw::{c_void, c_int};


pub type ZmqContext = *mut c_void;
pub type ZmqSocket = *mut c_void;

pub type CharPtr = *const u8;

pub const ZMQ_SUCCESS: c_int = 0;

pub const ZMQ_PAIR: c_int = 0;
pub const ZMQ_PUB: c_int = 1;
pub const ZMQ_SUB: c_int = 2;
pub const ZMQ_REQ: c_int = 3;
pub const ZMQ_REP: c_int = 4;
pub const ZMQ_DEALER: c_int = 5;
pub const ZMQ_ROUTER: c_int = 6;
pub const ZMQ_PULL: c_int = 7;
pub const ZMQ_PUSH: c_int = 8;
pub const ZMQ_XPUB: c_int = 9;
pub const ZMQ_XSUB: c_int = 10;
pub const ZMQ_STREAM: c_int = 11;


extern {
    pub fn zmq_ctx_new() -> ZmqContext;
    pub fn zmq_ctx_destroy(ctx: ZmqContext) -> c_int;

    pub fn zmq_socket(ctx: ZmqContext, stype: c_int) -> ZmqSocket;
    pub fn zmq_close(sck: ZmqSocket) -> c_int;

    pub fn zmq_bind(sck: ZmqSocket, addr: CharPtr) -> c_int;
    pub fn zmq_connect(sck: ZmqSocket, addr: CharPtr) -> c_int;

    pub fn zmq_send(sck: ZmqSocket, buffer: *const u8, len: usize, flags: c_int) -> c_int;
    pub fn zmq_recv(sck: ZmqSocket, buffer: *mut u8, len: usize, flags: c_int) -> c_int;
}
