use longport_candlesticks::{
    TRADE_SESSION_INTRADAY, TRADE_SESSION_OVERNIGHT, TRADE_SESSION_POST, TRADE_SESSION_PRE,
    TradeSessionType,
};
use time::Date;

use crate::quote::TradeSession;

#[inline]
pub(crate) fn parse_date(date: &str) -> Result<Date, time::error::Parse> {
    Date::parse(
        date,
        time::macros::format_description!("[year][month][day]"),
    )
}

pub(crate) fn format_date(date: Date) -> String {
    date.format(time::macros::format_description!("[year][month][day]"))
        .unwrap()
}

pub(crate) fn convert_trade_session(ts: TradeSession) -> TradeSessionType {
    match ts {
        TradeSession::Intraday => TRADE_SESSION_INTRADAY,
        TradeSession::Pre => TRADE_SESSION_PRE,
        TradeSession::Post => TRADE_SESSION_POST,
        TradeSession::Overnight => TRADE_SESSION_OVERNIGHT,
    }
}
