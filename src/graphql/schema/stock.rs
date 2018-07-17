//! File containing wizard store object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Stock: Context as "Stock" |&self| {
    description: "Warehouse Product info."

    interfaces: [&Node]

    field id(&executor) -> GraphqlID as "Base64 Unique id"{
        let context = executor.context();

        let id = format!("{}{}", self.warehouse_id, self.product_id);

        id.into()
    }

    field product_id() -> &i32 as "Product id"{
        &self.product_id.0
    }

    field product(&executor) -> FieldResult<Option<Product>> as "Fetches product." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.product_id.to_string()
        );

        context.request::<Option<Product>>(Method::Get, url, None)
            .wait()
    }

    field warehouse_id() -> String as "Warehouse id"{
        self.warehouse_id.clone().to_string()
    }

    field warehouse(&executor) -> FieldResult<Option<Warehouse>> as "Fetches warehouse." {
        let context = executor.context();

        let url = format!(
            "{}/{}/by-id/{}",
            &context.config.service_url(Service::Warehouses),
            Model::Warehouse.to_url(),
            self.warehouse_id.to_string()
        );

        context.request::<Option<Warehouse>>(Method::Get, url, None)
            .wait()
    }

    field quantity() -> &i32 as "Quantity"{
        &self.quantity.0
    }

});
