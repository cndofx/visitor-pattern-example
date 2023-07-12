mod bronze;
mod gold;
mod silver;

pub use bronze::BronzeCreditCard;
pub use gold::GoldCreditCard;
pub use silver::SilverCreditCard;

use crate::offers::OfferVisitor;

pub trait CreditCard {
    fn name(&self) -> String;
    fn accept(&self, v: &dyn OfferVisitor);
}
