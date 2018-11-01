//! File containing buy now values object of graphql schema

use graphql::context::Context;
use graphql::models::*;

graphql_object!(BuyNowCheckout: Context as "BuyNowCheckout" |&self| {
    description: "buy now values info."

    field product() -> &Product as "Product" {
        &self.product
    }

    field coupon() -> &Option<Coupon> as "Coupon added user" {
        &self.coupon
    }

    field coupons_discounts() -> f64 as "Coupons discounts" {
        calculate_coupon_discount(&self)
    }

    field total_cost() -> f64 as "Total cost" {
        calculate_cost(&self) + calculate_delivery_cost(&self)
    }

    field total_cost_without_discounts() -> f64 as "Total without cost" {
        calculate_cost_without_discounts(&self) + calculate_delivery_cost(&self)
    }

    field total_count() -> &i32 as "Total products count" {
        &self.quantity.0
    }

    field price() -> &f64 as "Price" {
        &self.product.price.0
    }

    field subtotal() -> f64 as "Subtotal with discounts" {
        calculate_cost(&self)
    }

    field subtotal_without_discounts() -> f64 as "Subtotal without discounts" {
        calculate_cost_without_discounts(&self)
    }

    field delivery_cost() -> f64 as "Delivery cost" {
        calculate_delivery_cost(&self)
    }
});

fn calculate_cost(buy_now: &BuyNowCheckout) -> f64 {
    if buy_now.quantity.0 <= 0 {
        return 0f64;
    }

    if let Some(discount) = buy_now.product.discount.filter(|discount| *discount < ZERO_DISCOUNT) {
        let calc_cost = (buy_now.product.price.0 * (f64::from(buy_now.quantity.0))) * (1.0f64 - discount);

        return calc_cost;
    } else {
        if let Some(coupon) = buy_now.coupon.as_ref() {
            // set discount only 1 product
            let set_discount = (buy_now.product.price.0 * 1f64) - ((buy_now.product.price.0 / 100f64) * f64::from(coupon.percent));
            let calc_cost = set_discount + (buy_now.product.price.0 * (f64::from(buy_now.quantity.0) - 1f64));

            return calc_cost;
        }
    }

    buy_now.product.price.0 * f64::from(buy_now.quantity.0)
}

fn calculate_cost_without_discounts(buy_now: &BuyNowCheckout) -> f64 {
    if buy_now.quantity.0 <= 0 {
        return 0f64;
    }

    buy_now.product.price.0 * f64::from(buy_now.quantity.0)
}

fn calculate_coupon_discount(buy_now: &BuyNowCheckout) -> f64 {
    let cost_with_discounts = calculate_cost(buy_now);

    calculate_cost_without_discounts(buy_now) - cost_with_discounts
}

fn calculate_delivery_cost(buy_now: &BuyNowCheckout) -> f64 {
    if let Some(package) = buy_now.package.as_ref() {
        if let Some(price) = package.price {
            return price.0;
        }
    }

    0.0f64
}
