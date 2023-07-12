use crate::creditcards::{BronzeCreditCard, GoldCreditCard, SilverCreditCard};

mod gas;
mod hotel;

pub use gas::GasOffer;
pub use hotel::HotelOffer;

pub trait OfferVisitor {
    fn visit_bronze_card(&self, card: BronzeCreditCard);
    fn visit_silver_card(&self, card: SilverCreditCard);
    fn visit_gold_card(&self, card: GoldCreditCard);
}
