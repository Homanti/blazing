mod service;
mod routes;
mod handlers;
mod ws_handler;

use uuid::Uuid;
use blazing_models::WsMessage;
pub use service::*;

pub use routes::*;
pub use handlers::*;
pub use ws_handler::*;

use blazing_ws::WsState;

pub type ChatWsState = WsState<ChatMessageHandler, Uuid, WsMessage>;