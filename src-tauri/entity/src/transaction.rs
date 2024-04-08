//! SeaORM Entity. Generated by sea-orm-codegen 0.3.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "transaction")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub wfm_id: String,
    pub wfm_url: String,
    pub item_name: String,
    pub item_type: String,
    pub item_unique_name: String,
    pub sub_type: Option<serde_json::Value>,
    pub tags: String,
    pub transaction_type: TransactionType,
    pub quantity: i64,
    pub user_name: String,
    pub price: i64,
    #[sea_orm(updated_at)]
    pub updated_at: DateTimeUtc,
    #[sea_orm(created_at)]
    pub created_at: DateTimeUtc,
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(15))")]
#[derive(Eq)]
pub enum TransactionType {
    #[sea_orm(string_value = "sell")]
    Sell,
    #[sea_orm(string_value = "buy")]
    Buy,
}

impl TransactionType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "sell" => Self::Sell,
            "buy" => Self::Buy,
            _ => panic!("Invalid transaction type"),
        }
    }
}
impl Serialize for TransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            TransactionType::Buy => "buy",
            TransactionType::Sell => "sell",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for TransactionType {
    fn deserialize<D>(deserializer: D) -> Result<TransactionType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "buy" => TransactionType::Buy,
            "sell" => TransactionType::Sell,
            _ => panic!("Invalid transaction type"),
        })
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
