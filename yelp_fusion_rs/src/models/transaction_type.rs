use serde::{Deserialize, Serialize};

/// Transactions that the business is registered for.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    /// Food is able to be picked up.
    Pickup,

    /// Food is able to be delivered.
    Delivery,

    /// Able to make reservations.
    RestaurantReservation,
}
