use crate::offers::OfferVisitor;

use super::CreditCard;

#[derive(Clone, Copy)]
pub struct SilverCreditCard;

impl CreditCard for SilverCreditCard {
    fn name(&self) -> String {
        "Silver Card".into()
    }

    fn accept(&self, v: &dyn OfferVisitor) {
        v.visit_silver_card(*self);
    }
}
