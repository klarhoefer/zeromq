#![allow(dead_code)]


use std::os::raw::{c_void, c_int};


pub type ZmqContext = *mut c_void;
pub type ZmqSocket = *mut c_void;

pub type CharPtr = *const u8;

pub const ZMQ_SUCCESS: c_int = 0;

// region defines

// Socket types.
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

// Socket options.
pub const ZMQ_AFFINITY: c_int = 4;
pub const ZMQ_ROUTING_ID: c_int = 5;
pub const ZMQ_SUBSCRIBE: c_int = 6;
pub const ZMQ_UNSUBSCRIBE: c_int = 7;
pub const ZMQ_RATE: c_int = 8;
pub const ZMQ_RECOVERY_IVL: c_int = 9;
pub const ZMQ_SNDBUF: c_int = 11;
pub const ZMQ_RCVBUF: c_int = 12;
pub const ZMQ_RCVMORE: c_int = 13;
pub const ZMQ_FD: c_int = 14;
pub const ZMQ_EVENTS: c_int = 15;
pub const ZMQ_TYPE: c_int = 16;
pub const ZMQ_LINGER: c_int = 17;
pub const ZMQ_RECONNECT_IVL: c_int = 18;
pub const ZMQ_BACKLOG: c_int = 19;
pub const ZMQ_RECONNECT_IVL_MAX: c_int = 21;
pub const ZMQ_MAXMSGSIZE: c_int = 22;
pub const ZMQ_SNDHWM: c_int = 23;
pub const ZMQ_RCVHWM: c_int = 24;
pub const ZMQ_MULTICAST_HOPS: c_int = 25;
pub const ZMQ_RCVTIMEO: c_int = 27;
pub const ZMQ_SNDTIMEO: c_int = 28;
pub const ZMQ_LAST_ENDPOINT: c_int = 32;
pub const ZMQ_ROUTER_MANDATORY: c_int = 33;
pub const ZMQ_TCP_KEEPALIVE: c_int = 34;
pub const ZMQ_TCP_KEEPALIVE_CNT: c_int = 35;
pub const ZMQ_TCP_KEEPALIVE_IDLE: c_int = 36;
pub const ZMQ_TCP_KEEPALIVE_INTVL: c_int = 37;
pub const ZMQ_IMMEDIATE: c_int = 39;
pub const ZMQ_XPUB_VERBOSE: c_int = 40;
pub const ZMQ_ROUTER_RAW: c_int = 41;
pub const ZMQ_IPV6: c_int = 42;
pub const ZMQ_MECHANISM: c_int = 43;
pub const ZMQ_PLAIN_SERVER: c_int = 44;
pub const ZMQ_PLAIN_USERNAME: c_int = 45;
pub const ZMQ_PLAIN_PASSWORD: c_int = 46;
pub const ZMQ_CURVE_SERVER: c_int = 47;
pub const ZMQ_CURVE_PUBLICKEY: c_int = 48;
pub const ZMQ_CURVE_SECRETKEY: c_int = 49;
pub const ZMQ_CURVE_SERVERKEY: c_int = 50;
pub const ZMQ_PROBE_ROUTER: c_int = 51;
pub const ZMQ_REQ_CORRELATE: c_int = 52;
pub const ZMQ_REQ_RELAXED: c_int = 53;
pub const ZMQ_CONFLATE: c_int = 54;
pub const ZMQ_ZAP_DOMAIN: c_int = 55;
pub const ZMQ_ROUTER_HANDOVER: c_int = 56;
pub const ZMQ_TOS: c_int = 57;
pub const ZMQ_CONNECT_ROUTING_ID: c_int = 61;
pub const ZMQ_GSSAPI_SERVER: c_int = 62;
pub const ZMQ_GSSAPI_PRINCIPAL: c_int = 63;
pub const ZMQ_GSSAPI_SERVICE_PRINCIPAL: c_int = 64;
pub const ZMQ_GSSAPI_PLAINTEXT: c_int = 65;
pub const ZMQ_HANDSHAKE_IVL: c_int = 66;
pub const ZMQ_SOCKS_PROXY: c_int = 68;
pub const ZMQ_XPUB_NODROP: c_int = 69;
pub const ZMQ_BLOCKY: c_int = 70;
pub const ZMQ_XPUB_MANUAL: c_int = 71;
pub const ZMQ_XPUB_WELCOME_MSG: c_int = 72;
pub const ZMQ_STREAM_NOTIFY: c_int = 73;
pub const ZMQ_INVERT_MATCHING: c_int = 74;
pub const ZMQ_HEARTBEAT_IVL: c_int = 75;
pub const ZMQ_HEARTBEAT_TTL: c_int = 76;
pub const ZMQ_HEARTBEAT_TIMEOUT: c_int = 77;
pub const ZMQ_XPUB_VERBOSER: c_int = 78;
pub const ZMQ_CONNECT_TIMEOUT: c_int = 79;
pub const ZMQ_TCP_MAXRT: c_int = 80;
pub const ZMQ_THREAD_SAFE: c_int = 81;
pub const ZMQ_MULTICAST_MAXTPDU: c_int = 84;
pub const ZMQ_VMCI_BUFFER_SIZE: c_int = 85;
pub const ZMQ_VMCI_BUFFER_MIN_SIZE: c_int = 86;
pub const ZMQ_VMCI_BUFFER_MAX_SIZE: c_int = 87;
pub const ZMQ_VMCI_CONNECT_TIMEOUT: c_int = 88;
pub const ZMQ_USE_FD: c_int = 89;
pub const ZMQ_GSSAPI_PRINCIPAL_NAMETYPE: c_int = 90;
pub const ZMQ_GSSAPI_SERVICE_PRINCIPAL_NAMETYPE: c_int = 91;
pub const ZMQ_BINDTODEVICE: c_int = 92;

// Send/recv options.
pub const ZMQ_DONTWAIT: c_int = 1;
pub const ZMQ_SNDMORE: c_int = 2;

// endregion

extern {
    pub fn zmq_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int);

    pub fn zmq_ctx_new() -> ZmqContext;
    pub fn zmq_ctx_destroy(ctx: ZmqContext) -> c_int;

    pub fn zmq_socket(ctx: ZmqContext, stype: c_int) -> ZmqSocket;
    pub fn zmq_close(sck: ZmqSocket) -> c_int;

    pub fn zmq_bind(sck: ZmqSocket, addr: CharPtr) -> c_int;
    pub fn zmq_connect(sck: ZmqSocket, addr: CharPtr) -> c_int;
    pub fn zmq_setsockopt(sck: ZmqSocket, option_id: c_int, option_value: *const u8, option_len: usize) -> c_int;

    pub fn zmq_send(sck: ZmqSocket, buffer: *const u8, len: usize, flags: c_int) -> c_int;
    pub fn zmq_recv(sck: ZmqSocket, buffer: *mut u8, len: usize, flags: c_int) -> c_int;
}

pub fn version() -> (c_int, c_int, c_int) {
    let mut major: c_int = 0;
    let mut minor: c_int = 0;
    let mut patch: c_int = 0;
    unsafe {
        zmq_version(&mut major as *mut _, &mut minor as *mut _, &mut patch as *mut _);
    }
    (major, minor, patch)
}
