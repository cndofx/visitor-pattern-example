use crate::creditcards::{BronzeCreditCard, GoldCreditCard, SilverCreditCard};

use super::OfferVisitor;

#[derive(Clone, Copy)]
pub struct HotelOffer;

impl OfferVisitor for HotelOffer {
    fn visit_bronze_card(&self, _: BronzeCreditCard) {
        println!("Bronze card paying at a hotel");
    }

    fn visit_silver_card(&self, _: SilverCreditCard) {
        println!("Silver card paying at a hotel");
    }

    fn visit_gold_card(&self, _: GoldCreditCard) {
        println!("Gold card paying at a hotel");
    }
}
