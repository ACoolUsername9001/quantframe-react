use serde::{Deserialize, Serialize};

use crate::utils::modules::logger;

#[derive(PartialEq, Debug, Clone)]
pub enum UIEvent {
    OnInitialize,
    UpdateSettings,
    UpdateAppInfo,
    UpdateError,
    UpdateOrders,
    UpdateTransaction,
    UpdateAuction,
    UpdateChats,
    UpdateChatMessages,
    UpdateStockItems,
    UpdateStockRivens,
    UpdateUser,
    UpdateLiveTradingRunningState,
    OnLiveTradingError,
    OnLiveTradingMessage,
    // Notification
    OnNotificationError,
    OnNotificationWarn,
    OnNotificationSuccess,
}
impl UIEvent {
    pub fn as_str(&self) -> &str {
        match *self {
            UIEvent::OnInitialize => "App:OnInitialize",
            UIEvent::UpdateSettings => "App:UpdateSettings",
            UIEvent::UpdateAppInfo => "App:UpdateAppInfo",
            UIEvent::UpdateError => "App:UpdateError",
            UIEvent::UpdateOrders => "WFM:UpdateOrders",
            UIEvent::UpdateTransaction => "WFM:UpdateTransaction",
            UIEvent::UpdateAuction => "WFM:UpdateAuction",
            UIEvent::UpdateChats => "WFM:UpdateChats",
            UIEvent::UpdateChatMessages => "WFM:UpdateChatMessages",
            UIEvent::UpdateStockItems => "Stock:UpdateStockItems",
            UIEvent::UpdateStockRivens => "Stock:UpdateStockRivens",
            UIEvent::UpdateUser => "User:Update",
            UIEvent::UpdateLiveTradingRunningState => "LiveTrading:UpdateRunningState",
            UIEvent::OnLiveTradingError => "LiveTrading:OnError",
            UIEvent::OnLiveTradingMessage => "LiveTrading:OnMessage",
            UIEvent::OnNotificationError => "Notification:OnError",
            UIEvent::OnNotificationWarn => "Notification:OnWarn",
            UIEvent::OnNotificationSuccess => "Notification:OnSuccess",
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum UIOperationEvent {
    CreateOrUpdate,
    Delete,
    Set,
}
impl UIOperationEvent {
    pub fn as_str(&self) -> &str {
        match *self {
            UIOperationEvent::CreateOrUpdate => "CREATE_OR_UPDATE",
            UIOperationEvent::Delete => "DELETE",
            UIOperationEvent::Set => "SET",
        }
    }
}
