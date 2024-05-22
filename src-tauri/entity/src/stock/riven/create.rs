use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::attribute::RivenAttribute;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateStockRiven {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bought")]
    pub bought: Option<i64>,
    #[serde(default = "String::default")]
    #[serde(rename = "wfm_id")]
    pub wfm_id: String,
    #[serde(rename = "wfm_url")]
    pub wfm_url: String,
    #[serde(rename = "mod_name")]
    pub mod_name: String,
    #[serde(default = "String::default")]
    #[serde(rename = "weapon_name")]
    pub weapon_name: String,
    #[serde(default = "String::default")]
    #[serde(rename = "weapon_unique_name")]
    pub weapon_unique_name: String,
    #[serde(rename = "weapon_type")]
    pub weapon_type: String,
    #[serde(rename = "mastery_rank")]
    pub mastery_rank: i64,
    #[serde(rename = "re_rolls")]
    pub re_rolls: i64,
    #[serde(rename = "polarity")]
    pub polarity: String,
    #[serde(rename = "attributes")]
    pub attributes: Vec<RivenAttribute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minimum_price")]
    pub minimum_price: Option<i64>,
    #[serde(rename = "rank")]
    pub rank: i64,
}

impl CreateStockRiven {
    pub fn to_stock(&self) -> super::stock_riven::Model {
        super::stock_riven::Model::new(
            self.wfm_id.clone(), 
            self.wfm_url.clone(),
            None, 
            self.weapon_name.clone(),
            self.weapon_type.clone(),
            self.weapon_unique_name.clone(),
            self.rank,
            self.mod_name.clone(),
            super::attribute::RivenAttributeVec(self.attributes.clone()),
            self.mastery_rank,
            self.re_rolls,
            self.polarity.clone(),
            self.bought.unwrap_or(0),
            self.minimum_price,
            false, 
            "".to_string())
    }
}