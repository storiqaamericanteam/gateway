use juniper;
use graphql::context::Context;
use graphql::models::*;
use juniper::ID as GraphqlID;

use super::*;

graphql_object!(Product: Context as "Product" |&self| {
    description: "Product's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Unique id"{
        ID::new(Service::Stores, Model::Product, self.id).to_string().into()
    }

    field raw_id() -> GraphqlID as "Unique id"{
        self.id.to_string().into()
    }

    field name() -> String as "Full Name" {
        self.name.clone()
    }

    field is_active() -> bool as "If the product was disabled (deleted), isActive is false" {
        self.is_active
    }

    field short_description() -> String as "Short description" {
        self.short_description.clone()
    }

    field long_description() -> Option<String> as "Long Description" {
        self.long_description.clone()
    }

    field price() -> f64 as "Price" {
        self.price.clone()
    }

    field currency_id() -> i32 as "Currency Id" {
        self.currency_id.clone()
    }

    field discount() -> Option<f64> as "Discount" {
        self.discount.clone()
    }

    field category() -> Option<i32> as "Category" {
        self.category.clone()
    }

    field photo_main() -> Option<String> as "Photo main" {
        self.photo_main.clone()
    }

});

graphql_object!(Connection<Product>: Context as "ProductsConnection" |&self| {
    description:"Products Connection"

    field edges() -> Vec<Edge<Product>> {
        self.edges.to_vec()
    }

    field page_info() -> PageInfo {
        self.page_info.clone()
    }
});

graphql_object!(Edge<Product>: Context as "ProductsEdge" |&self| {
    description:"Products Edge"
    
    field cursor() -> juniper::ID {
        self.cursor.clone()
    }

    field node() -> Product {
        self.node.clone()
    }
});
