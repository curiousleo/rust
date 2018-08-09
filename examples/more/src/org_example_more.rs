#![doc = "This file was automatically generated by the varlink rust generator"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use failure::{Backtrace, Context, Fail};
use serde_json;
use std::io::BufRead;
use std::sync::{Arc, RwLock};
use varlink::{self, CallTrait};
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct State {
    pub start: Option<bool>,
    pub progress: Option<i64>,
    pub end: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TestMoreError_Args {
    pub reason: String,
}
pub trait VarlinkCallError: varlink::CallTrait {
    fn reply_test_more_error(&mut self, reason: String) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "org.example.more.TestMoreError",
            Some(serde_json::to_value(TestMoreError_Args { reason })?),
        ))
    }
}
impl<'a> VarlinkCallError for varlink::Call<'a> {}
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}
#[derive(Clone, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "IO error")]
    Io_Error(::std::io::ErrorKind),
    #[fail(display = "(De)Serialization Error")]
    SerdeJson_Error(serde_json::error::Category),
    #[fail(display = "Varlink Error")]
    Varlink_Error(varlink::ErrorKind),
    #[fail(display = "Unknown error reply: '{:#?}'", _0)]
    VarlinkReply_Error(varlink::Reply),
    #[fail(display = "org.example.more.TestMoreError: {:#?}", _0)]
    TestMoreError(Option<TestMoreError_Args>),
}
impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self.inner, f)
    }
}
impl Error {
    #[allow(dead_code)]
    pub fn kind(&self) -> ErrorKind {
        self.inner.get_context().clone()
    }
}
impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}
impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}
impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Error {
        let kind = e.kind();
        e.context(ErrorKind::Io_Error(kind)).into()
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        let cat = e.classify();
        e.context(ErrorKind::SerdeJson_Error(cat)).into()
    }
}
#[allow(dead_code)]
pub type Result<T> = ::std::result::Result<T, Error>;
impl From<varlink::Error> for Error {
    fn from(e: varlink::Error) -> Self {
        let kind = e.kind();
        match kind {
            varlink::ErrorKind::Io(kind) => e.context(ErrorKind::Io_Error(kind)).into(),
            varlink::ErrorKind::SerdeJsonSer(cat) => {
                e.context(ErrorKind::SerdeJson_Error(cat)).into()
            }
            kind => e.context(ErrorKind::Varlink_Error(kind)).into(),
        }
    }
}
impl From<varlink::Reply> for Error {
    fn from(e: varlink::Reply) -> Self {
        if varlink::Error::is_error(&e) {
            return varlink::Error::from(e).into();
        }
        match e {
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "org.example.more.TestMoreError" =>
            {
                match e {
                    varlink::Reply {
                        parameters: Some(p),
                        ..
                    } => match serde_json::from_value(p) {
                        Ok(v) => ErrorKind::TestMoreError(v).into(),
                        Err(_) => ErrorKind::TestMoreError(None).into(),
                    },
                    _ => ErrorKind::TestMoreError(None).into(),
                }
            }
            _ => ErrorKind::VarlinkReply_Error(e).into(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Ping_Reply {
    pub pong: String,
}
impl varlink::VarlinkReply for Ping_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Ping_Args {
    pub ping: String,
}
pub trait Call_Ping: VarlinkCallError {
    fn reply(&mut self, pong: String) -> varlink::Result<()> {
        self.reply_struct(Ping_Reply { pong }.into())
    }
}
impl<'a> Call_Ping for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StopServing_Reply {}
impl varlink::VarlinkReply for StopServing_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StopServing_Args {}
pub trait Call_StopServing: VarlinkCallError {
    fn reply(&mut self) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}
impl<'a> Call_StopServing for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TestMore_Reply {
    pub state: State,
}
impl varlink::VarlinkReply for TestMore_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TestMore_Args {
    pub n: i64,
}
pub trait Call_TestMore: VarlinkCallError {
    fn reply(&mut self, state: State) -> varlink::Result<()> {
        self.reply_struct(TestMore_Reply { state }.into())
    }
}
impl<'a> Call_TestMore for varlink::Call<'a> {}
pub trait VarlinkInterface {
    fn ping(&self, call: &mut Call_Ping, ping: String) -> varlink::Result<()>;
    fn stop_serving(&self, call: &mut Call_StopServing) -> varlink::Result<()>;
    fn test_more(&self, call: &mut Call_TestMore, n: i64) -> varlink::Result<()>;
    fn call_upgraded(
        &self,
        _call: &mut varlink::Call,
        _bufreader: &mut BufRead,
    ) -> varlink::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}
pub trait VarlinkClientInterface {
    fn ping(&mut self, ping: String) -> varlink::MethodCall<Ping_Args, Ping_Reply, Error>;
    fn stop_serving(&mut self) -> varlink::MethodCall<StopServing_Args, StopServing_Reply, Error>;
    fn test_more(&mut self, n: i64) -> varlink::MethodCall<TestMore_Args, TestMore_Reply, Error>;
}
#[allow(dead_code)]
pub struct VarlinkClient {
    connection: Arc<RwLock<varlink::Connection>>,
}
impl VarlinkClient {
    #[allow(dead_code)]
    pub fn new(connection: Arc<RwLock<varlink::Connection>>) -> Self {
        VarlinkClient { connection }
    }
}
impl VarlinkClientInterface for VarlinkClient {
    fn ping(&mut self, ping: String) -> varlink::MethodCall<Ping_Args, Ping_Reply, Error> {
        varlink::MethodCall::<Ping_Args, Ping_Reply, Error>::new(
            self.connection.clone(),
            "org.example.more.Ping",
            Ping_Args { ping },
        )
    }
    fn stop_serving(&mut self) -> varlink::MethodCall<StopServing_Args, StopServing_Reply, Error> {
        varlink::MethodCall::<StopServing_Args, StopServing_Reply, Error>::new(
            self.connection.clone(),
            "org.example.more.StopServing",
            StopServing_Args {},
        )
    }
    fn test_more(&mut self, n: i64) -> varlink::MethodCall<TestMore_Args, TestMore_Reply, Error> {
        varlink::MethodCall::<TestMore_Args, TestMore_Reply, Error>::new(
            self.connection.clone(),
            "org.example.more.TestMore",
            TestMore_Args { n },
        )
    }
}
#[allow(dead_code)]
pub struct VarlinkInterfaceProxy {
    inner: Box<VarlinkInterface + Send + Sync>,
}
#[allow(dead_code)]
pub fn new(inner: Box<VarlinkInterface + Send + Sync>) -> VarlinkInterfaceProxy {
    VarlinkInterfaceProxy { inner }
}
impl varlink::Interface for VarlinkInterfaceProxy {
    fn get_description(&self) -> &'static str {
        "# Example Varlink service\ninterface org.example.more\n\n# Enum, returning either start, progress or end\n# progress: [0-100]\ntype State (\n  start: ?bool,\n  progress: ?int,\n  end: ?bool\n)\n\n# Returns the same string\nmethod Ping(ping: string) -> (pong: string)\n\n# Dummy progress method\n# n: number of progress steps\nmethod TestMore(n: int) -> (state: State)\n\n# Stop serving\nmethod StopServing() -> ()\n\n# Something failed in TestMore\nerror TestMoreError (reason: string)\n"
    }
    fn get_name(&self) -> &'static str {
        "org.example.more"
    }
    fn call_upgraded(
        &self,
        call: &mut varlink::Call,
        bufreader: &mut BufRead,
    ) -> varlink::Result<Vec<u8>> {
        self.inner.call_upgraded(call, bufreader)
    }
    fn call(&self, call: &mut varlink::Call) -> varlink::Result<()> {
        let req = call.request.unwrap();
        match req.method.as_ref() {
            "org.example.more.Ping" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Ping_Args = match serde_json::from_value(args) {
                        Ok(v) => v,
                        Err(e) => {
                            let es = format!("{}", e);
                            let _ = call.reply_invalid_parameter(es.clone());
                            return Err(varlink::ErrorKind::SerdeJsonDe(es).into());
                        }
                    };
                    self.inner.ping(call as &mut Call_Ping, args.ping)
                } else {
                    call.reply_invalid_parameter("parameters".into())
                }
            }
            "org.example.more.StopServing" => {
                self.inner.stop_serving(call as &mut Call_StopServing)
            }
            "org.example.more.TestMore" => {
                if let Some(args) = req.parameters.clone() {
                    let args: TestMore_Args = match serde_json::from_value(args) {
                        Ok(v) => v,
                        Err(e) => {
                            let es = format!("{}", e);
                            let _ = call.reply_invalid_parameter(es.clone());
                            return Err(varlink::ErrorKind::SerdeJsonDe(es).into());
                        }
                    };
                    self.inner.test_more(call as &mut Call_TestMore, args.n)
                } else {
                    call.reply_invalid_parameter("parameters".into())
                }
            }
            m => call.reply_method_not_found(String::from(m)),
        }
    }
}
