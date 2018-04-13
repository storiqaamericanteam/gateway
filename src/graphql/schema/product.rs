//! File containing product object of graphql schema

use juniper;
use juniper::ID as GraphqlID;
use juniper::FieldResult;
use hyper::Method;
use futures::Future;
 
use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;
use super::*;

graphql_object!(Product: Context as "Product" |&self| {
    description: "Product's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::Product, self.id).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id
    }

    field is_active() -> &bool as "If the product was disabled (deleted), isActive is false" {
        &self.is_active
    }

    field discount() -> &Option<f64> as "Discount" {
        &self.discount
    }

    field photo_main() -> &Option<String> as "Photo main" {
        &self.photo_main
    }

    field additional_photos() -> &Option<Vec<String>> as "Additional photos of the product." {
        &self.additional_photos
    }

    field vendor_code() -> &Option<String> as "Vendor code" {
        &self.vendor_code
    }

    field cashback() -> &Option<f64> as "Cashback" {
        &self.cashback
    }

    field price() -> &f64 as "Price" {
        &self.price
    }

    field attributes(&executor) -> FieldResult<Option<Vec<AttrValue>>> as "Variants" {
       let context = executor.context();
        let url = format!("{}/{}/{}/attributes",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.id);

        context.request::<Vec<AttrValue>>(Method::Get, url, None)
            .wait()
            .or_else(|_| Ok(vec![]))
            .map(|u| Some(u))
    }

});

graphql_object!(Connection<Product, PageInfo>: Context as "ProductsConnection" |&self| {
    description:"Products Connection"

    field edges() -> &[Edge<Product>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<Product>: Context as "ProductsEdge" |&self| {
    description:"Products Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &Product {
        &self.node
    }
});
