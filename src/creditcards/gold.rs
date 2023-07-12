use crate::offers::OfferVisitor;

use super::CreditCard;

#[derive(Clone, Copy)]
pub struct GoldCreditCard;

impl CreditCard for GoldCreditCard {
    fn name(&self) -> String {
        "Gold Card".into()
    }

    fn accept(&self, v: &dyn OfferVisitor) {
        v.visit_gold_card(*self);
    }
}
