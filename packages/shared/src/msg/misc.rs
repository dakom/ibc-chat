use cosmwasm_schema::cw_serde;

/// Like [cosmwasm_std::Order] but serialized as a string
#[cw_serde]
#[derive(Eq, Copy)]
pub enum Order {
    /// Ascending order
    Ascending,
    /// Descending order
    Descending,
}

impl From<Order> for cosmwasm_std::Order {
    fn from(order: Order) -> Self {
        match order {
            Order::Ascending => Self::Ascending,
            Order::Descending => Self::Descending,
        }
    }
}

impl From<cosmwasm_std::Order> for Order {
    fn from(order: cosmwasm_std::Order) -> Self {
        match order {
            cosmwasm_std::Order::Ascending => Self::Ascending,
            cosmwasm_std::Order::Descending => Self::Descending,
        }
    }
}