use juniper::ID as GraphqlID;

use super::*;

#[derive(GraphQLEnum, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[graphql(name = "OrderStatus", description = "Current order status")]
pub enum OrderStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "in_processing")]
    InProcessing,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "complete")]
    Complete,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Order {
    pub id: i32,
    pub status: OrderStatus,
    pub customer_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub store_id: i32,
    pub price: f64,
    pub receiver_name: String,
    pub slug: i32,
    pub payment_status: bool,
    pub delivery_company: String,
    pub track_id: Option<String>,
    pub creation_time: String,
    pub administrative_area_level_1: Option<String>,
    pub administrative_area_level_2: Option<String>,
    pub country: String,
    pub locality: Option<String>,
    pub political: Option<String>,
    pub postal_code: String,
    pub route: Option<String>,
    pub street_number: Option<String>,
    pub address: Option<String>,
    pub place_id: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Create order input object")]
pub struct CreateOrderInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Customer comments.")]
    pub customer_comments: Option<String>,
    #[graphql(description = "Address")]
    #[serde(flatten)]
    pub address_full: AddressInput,
    #[graphql(description = "Receiver name")]
    pub receiver_name: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CreateOrder {
    pub customer_id: i32,
    pub comments: Option<String>,
    #[serde(flatten)]
    pub address: AddressInput,
    pub receiver_name: String,
    pub cart_products: CartProductWithPriceHash,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Order Status Delivery input.")]
pub struct OrderStatusDeliveryInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of order.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "Track id.")]
    pub track_id: String,
    #[graphql(description = "Comments")]
    pub comments: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct OrderStatusDelivery {
    pub status: OrderStatus,
    pub track_id: String,
    pub comments: String,
}

impl From<OrderStatusDeliveryInput> for OrderStatusDelivery {
    fn from(order: OrderStatusDeliveryInput) -> Self {
        Self {
            status: OrderStatus::Sent,
            track_id: order.track_id,
            comments: order.comments,
        }
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Order Status Paid input.")]
pub struct OrderStatusPaidInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of order.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "Comments")]
    pub comments: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct OrderStatusPaid {
    pub status: OrderStatus,
    pub comments: String,
}

impl From<OrderStatusPaidInput> for OrderStatusPaid {
    fn from(order: OrderStatusPaidInput) -> Self {
        Self {
            status: OrderStatus::Paid,
            comments: order.comments,
        }
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Order Status Complete input.")]
pub struct OrderStatusCompleteInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of order.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "Comments")]
    pub comments: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct OrderStatusComplete {
    pub status: OrderStatus,
    pub comments: String,
}

impl From<OrderStatusCompleteInput> for OrderStatusComplete {
    fn from(order: OrderStatusCompleteInput) -> Self {
        Self {
            status: OrderStatus::Complete,
            comments: order.comments,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderHistoryItem {
    pub status: OrderStatus,
    pub user_id: i32,
    pub comments: Option<String>,
    pub creation_time: String,
}
