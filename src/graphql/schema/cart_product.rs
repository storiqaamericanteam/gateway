//! File containing PageInfo object of graphql schema
use std::collections::{HashMap, HashSet};

use futures::Future;
use hyper::Method;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use stq_api::orders::DeliveryInfo;
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Translation;
use stq_types::{BaseProductId, CartItem, CompanyPackageId, DeliveryMethodId, ProductId, Quantity};

use super::*;
use graphql::context::Context;
use graphql::models::*;
use graphql::schema::available_packages::*;
use graphql::schema::coupon::*;
use graphql::schema::product::*;

graphql_object!(CartProduct: Context as "CartProduct" |&self| {
    description: "Cart Product info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Orders, Model::CartProduct, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id.0
    }

    field name() -> &[Translation] as "Full Name" {
        &self.name
    }

    field quantity() -> &i32 as "Quantity" {
        &self.quantity.0
    }

    field price() -> &f64 as "Price" {
        &self.price.0
    }

    field subtotal(&executor) -> FieldResult<f64> as "Subtotal with discounts" {
        let context = executor.context();
        calculate_product_price(context, &self)
    }

    field subtotal_without_discounts() -> f64 as "Subtotal without discounts" {
        self.price.0 * f64::from(self.quantity.0)
    }

    field delivery_cost(&executor) -> FieldResult<f64> as "Delivery cost" {
        let context = executor.context();

        calculate_delivery_cost(context, &self)
    }

    field photo_main() -> &Option<String> as "Photo main" {
        &self.photo_main
    }

    field comment() -> &str as "Comment" {
        &self.comment
    }

    field selected() -> &bool as "Selected" {
        &self.selected
    }

    field company_package(&executor) -> FieldResult<Option<CompaniesPackages>> as "Company package" {
        let context = executor.context();
        match self.company_package_id {
            Some(company_package_id) => {
                let url = format!("{}/{}/{}",
                    context.config.service_url(Service::Delivery),
                    Model::CompanyPackage.to_url(),
                    company_package_id,
                );

                context.request::<Option<CompaniesPackages>>(Method::Get, url, None).wait()
            },
            None => Ok(None),
        }
    }

    field deprecated "use companyPackage" delivery_operator() -> &str as "Delivery Operator" {
        "Operator"
    }

    field delivery_period() -> &str as "Delivery Period" {
        "14 days"
    }

    field delivery_return_type() -> &str as "Delivery return type" {
        "funds return"
    }

    field delivery_return_paid_by() -> &str as "Delivery return paid by" {
        "Seller"
    }

    field pre_order() -> &bool as "Pre order" {
        &self.pre_order
    }

    field pre_order_days() -> &i32 as "Pre order days" {
        &self.pre_order_days
    }

    field coupon(&executor) -> FieldResult<Option<Coupon>> as "Coupon added user" {
        let context = executor.context();
        if let Some(coupon_id) = self.coupon_id {
            try_get_coupon(context, coupon_id)
        } else {
            Ok(None)
        }
    }

    field coupon_discount(&executor) -> FieldResult<f64> as "Coupon discount" {
        let context = executor.context();

        calculate_coupon_discount(context, &self)
    }

    field base_product(&executor,
        visibility: Option<Visibility> as "Specifies allowed visibility of the base_product"
    ) -> FieldResult<Option<BaseProduct>> as "Fetches base product by product." {
        let context = executor.context();
        let visibility = visibility.unwrap_or_default();

        let url = format!(
            "{}/{}/{}?visibility={}",
            &context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            self.base_product_id.to_string(),
            visibility,
        );

        context.request::<Option<BaseProduct>>(Method::Get, url, None)
            .wait()
    }

    field base_product_id() -> &i32 as "BaseProductId" {
        &self.base_product_id.0
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
            .map(Some)
    }
});

pub fn calculate_product_price(context: &Context, cart_product: &CartProduct) -> FieldResult<f64> {
    if cart_product.quantity.0 <= 0 {
        return Ok(0f64);
    }

    if let Some(discount) = cart_product.discount.filter(|discount| *discount < ZERO_DISCOUNT) {
        let calc_price = (cart_product.price.0 * (f64::from(cart_product.quantity.0))) * (1.0f64 - discount);

        return Ok(calc_price);
    } else {
        if let Some(coupon_id) = cart_product.coupon_id {
            if let Some(coupon) = try_get_coupon(context, coupon_id)? {
                // set discount only 1 product
                let set_discount = (cart_product.price.0 * 1f64) - ((cart_product.price.0 / 100f64) * f64::from(coupon.percent));
                let calc_price = set_discount + (cart_product.price.0 * (f64::from(cart_product.quantity.0) - 1f64));

                return Ok(calc_price);
            }
        }
    }

    Ok(cart_product.price.0 * f64::from(cart_product.quantity.0))
}

pub fn calculate_product_price_without_discounts(product: &CartProduct) -> f64 {
    if product.quantity.0 <= 0 {
        return 0f64;
    }

    product.price.0 * f64::from(product.quantity.0)
}

pub fn calculate_coupon_discount(context: &Context, product: &CartProduct) -> FieldResult<f64> {
    let price_with_discounts = calculate_product_price(context, product)?;

    Ok(calculate_product_price_without_discounts(product) - price_with_discounts)
}

pub fn calculate_delivery_cost(context: &Context, product: &CartProduct) -> FieldResult<f64> {
    if let Some(company_package_id) = product.company_package_id {
        return calculate_delivery(context, product.base_product_id, company_package_id, product.quantity);
    }

    Ok(0.0f64)
}

pub fn calculate_delivery(
    context: &Context,
    base_product_id: BaseProductId,
    company_package_id: CompanyPackageId,
    quantity: Quantity,
) -> FieldResult<f64> {
    if quantity.0 <= 0 {
        return Ok(0f64);
    }

    let package = get_available_package_for_user(context, base_product_id, company_package_id)?;

    if let Some(price) = package.price {
        return Ok(price.0 * f64::from(quantity.0));
    }

    Ok(0.0f64)
}

pub fn get_delivery_info(context: &Context, cart_items: &HashSet<CartItem>) -> FieldResult<HashMap<ProductId, DeliveryInfo>> {
    let mut delivery_info = vec![];

    for cart_item in cart_items.iter() {
        if let Some(delivery_method_id) = cart_item.delivery_method_id {
            match delivery_method_id {
                DeliveryMethodId::Package { id: company_package_id } => {
                    let product = get_product(context, cart_item.product_id)?;
                    let package = get_available_package_for_user(context, product.base_product_id, company_package_id)?;
                    let calc_price = calculate_delivery(context, product.base_product_id, company_package_id, cart_item.quantity)?;

                    let element = DeliveryInfo {
                        company_package_id,
                        name: package.name,
                        logo: package.logo,
                        price: calc_price,
                    };

                    delivery_info.push((cart_item.product_id, element));
                }
                _ => {
                    return Err(FieldError::new(
                        "Could not create orders for cart.",
                        graphql_value!({ "code": 100, "details": { "Delivery method not support." }}),
                    ));
                }
            }
        }
    }

    Ok(delivery_info.into_iter().collect::<HashMap<ProductId, DeliveryInfo>>())
}
