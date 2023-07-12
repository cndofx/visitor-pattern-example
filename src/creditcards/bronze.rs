use crate::offers::OfferVisitor;

use super::CreditCard;

#[derive(Clone, Copy)]
pub struct BronzeCreditCard;

impl CreditCard for BronzeCreditCard {
    fn name(&self) -> String {
        "Bronze Card".into()
    }

    fn accept(&self, v: &dyn OfferVisitor) {
        v.visit_bronze_card(*self);
    }
}
