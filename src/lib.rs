use pelican_ui::start;

mod examples;
use examples::{
    ice_cream::IceCreamApp,
    plants::PlantGrowerApp,
    motorcycle::MotorcycleApp,
};

// change the struct here to start a different example.
start!(PlantGrowerApp);
