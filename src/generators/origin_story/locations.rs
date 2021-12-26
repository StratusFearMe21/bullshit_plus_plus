use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(RandGen)]
pub enum Location {
    Local(LocalLocation),
    EifelTower,
    BurgeKalifa,
    EmpireState,
    TajMahal,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(location) => write!(f, "the local {}", location),
            Self::EifelTower => write!(f, "the Eifel Tower"),
            Self::BurgeKalifa => write!(f, "the Burge Kalifa"),
            Self::EmpireState => write!(f, "the Empire State Building"),
            Self::TajMahal => write!(f, "the Taj Mahal"),
        }
    }
}

#[derive(RandGen)]
pub enum LocalLocation {
    // Grocery Stores
    GroceryStore,
    Safeway,
    Giant,
    Albertsons,
    // Coffee Shops
    CoffeeShop,
    Starbucks,
    PeetsCoffee,
    // Arcades
    Arcade,
    CheckECheeses,
    DaveAndBusters,
}

impl Display for LocalLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            // Grocery Stores
            Self::GroceryStore => write!(f, "grocery store"),
            Self::Safeway => write!(f, "Safeway"),
            Self::Giant => write!(f, "Giant"),
            Self::Albertsons => write!(f, "Albertsons"),
            // Coffee Shops
            Self::CoffeeShop => write!(f, "coffee shop"),
            Self::Starbucks => write!(f, "Starbucks"),
            Self::PeetsCoffee => write!(f, "Peets Coffee"),
            // Arcades
            Self::Arcade => write!(f, "arcade"),
            Self::CheckECheeses => write!(f, "Chuck E. Cheeses"),
            Self::DaveAndBusters => write!(f, "Dave and Busters"),
        }
    }
}
