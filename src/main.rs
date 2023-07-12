use creditcards::{BronzeCreditCard, CreditCard, GoldCreditCard, SilverCreditCard};
use offers::{GasOffer, HotelOffer, OfferVisitor};

pub mod creditcards;
pub mod offers;

fn main() {
    let hotel = HotelOffer;
    let gas = GasOffer;
    let mut offers: Vec<Box<dyn OfferVisitor>> = Vec::new();
    offers.push(Box::new(hotel));
    offers.push(Box::new(gas));

    let bronze = BronzeCreditCard;
    let silver = SilverCreditCard;
    let gold = GoldCreditCard;
    let mut cards: Vec<Box<dyn CreditCard>> = Vec::new();
    cards.push(Box::new(bronze));
    cards.push(Box::new(silver));
    cards.push(Box::new(gold));

    println!("\niterating over offers");
    for offer in &offers {
        bronze.accept(offer.as_ref());
        silver.accept(offer.as_ref());
        gold.accept(offer.as_ref());
    }

    println!("\niterating over cards");
    for card in &cards {
        card.accept(&hotel);
        card.accept(&gas);
    }

    println!("\niterating over both");
    for card in &cards {
        for offer in &offers {
            card.accept(offer.as_ref());
        }
    }
}
