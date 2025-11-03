use roost::start;

mod examples;
use examples::{
    ice_cream::IceCreamApp,
    // plants::PlantGrowerApp
};

// change the struct here to start a different example.
start!(IceCreamApp);
