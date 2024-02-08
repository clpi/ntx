use std::ops::Deref;
use log::info;
use ntex_identity::{
    Identity, IdentityService, IdentityPolicy, RequestIdentity,
    CookieIdentityPolicy, CookieIdentityPolicyError,
};
use ntex_session::{
    Session, SessionStatus, UserSession, CookieSession
};
use ntex::{
    task::LocalWaker,
    codec::{BytesCodec, Decoder, Encoder},
    channel::{
        oneshot::{Sender as OSSender, Receiver as OSReceiver},
        pool::{Pool, Receiver as PReceiver, Sender as PSender, new,self},
        condition::{Condition, Waiter,},
        mpsc::{channel, Receiver, Sender, SendError, WeakSender},
        condition
    },
    time::{
        Deadline, Interval, Seconds, Sleep, Timeout, TimeoutChecked, TimerHandle,
        deadline, interval, sleep, timeout_checked, timeout, Millis, system_time, now,
    },
    io::{ Io, OnDisconnect, IoBoxed, IoRef, DispatcherConfig, IoStatusUpdate, Filter,
        Sealed, IoStream, ReadContext, FilterLayer, Layer, Handle, TimerHandle as IoTimerHandle,
        RecvError, Dispatcher, DispatchItem, Base, WriteBuf, WriteStatus, Framed, TimerHandle as IOTimerHandle,
        types::{PeerAddr, QueryItem, HttpProtocol, self}
    },
    codec::{Decoder as CDecoder, Encoder as CEncoder, BytesCodec as CBytesCodec},
    // http::{
        // HttpProtocol as HttpProtocolHttp, HttpServiceBuilder, HttpMessage, HeaderMap, RequestHead, RequestHeadType,
        // ResponseBuilder, KeepAlive, Uri, Version, Method as HMethod, ServiceConfig, StatusCode, HttpService, DateService,
        // body::{
            // ResponseBody, BodyStream, SizedStream, BodySize, BoxedBodyStream, Body, MessageBody, self,
        // },
        // header::{
            // HOST, IF_MATCH, SEC_WEBSOCKET_ACCEPT, SEC_WEBSOCKET_EXTENSIONS, SEC_WEBSOCKET_KEY, SEC_WEBSOCKET_PROTOCOL,
            // SEC_WEBSOCKET_VERSION, SET_COOKIE, SERVER, CONTENT_SECURITY_POLICY, COOKIE, CONNECTION, DATE, DNT, CONTENT_DISPOSITION,
            // CACHE_CONTROL, CONTENT_LOCATION, ORIGIN, PUBLIC_KEY_PINS_REPORT_ONLY, PUBLIC_KEY_PINS, PRAGMA, PROXY_AUTHORIZATION,
            // FORWARDED, FROM, IF_RANGE, IF_MODIFIED_SINCE, IF_UNMODIFIED_SINCE, WARNING, WWW_AUTHENTICATE, MAX_FORWARDS,
            // IF_NONE_MATCH, Either, ToStrError, ETAG, EXPECT, EXPIRES, AGE, RANGE, ACCESS_CONTROL_ALLOW_CREDENTIALS
        // },
    // },
    web::{
        types::{
            State, Form, Query, Path, Json, JsonConfig,
            PayloadConfig, Payload, FormConfig
        },
        guard::{
            Get, Guard, Patch, Put, Delete, Post, Options, Connect, Trace, Head,
            Method, All, Any, Not, AllGuard, AnyGuard, Header, Host,
        },
        middleware::{
            DefaultHeaders,Logger,
        },
        dev::{AppConfig, WebServiceAdapter, WebServiceConfig, ResourceMap, WebServiceFactory},
        ws::{CloseCode, CloseReason, Frame, Message, WsSink},
        HttpServer, Responder, HttpResponse,
        self, App, get, post, delete, put, middleware,
    },
};
use ntx::models::user::User;
use ntx::route::{about, index, info};
use ntx::state::AppState;





#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    ntx::app::run().await
}