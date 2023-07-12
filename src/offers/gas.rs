use crate::creditcards::{BronzeCreditCard, GoldCreditCard, SilverCreditCard};

use super::OfferVisitor;

#[derive(Clone, Copy)]
pub struct GasOffer;

impl OfferVisitor for GasOffer {
    fn visit_bronze_card(&self, _: BronzeCreditCard) {
        println!("Bronze card buying gas");
    }

    fn visit_silver_card(&self, _: SilverCreditCard) {
        println!("Silver card buying gas");
    }

    fn visit_gold_card(&self, _: GoldCreditCard) {
        println!("Gold card buying gas");
    }
}
