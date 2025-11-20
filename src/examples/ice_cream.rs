use pelican_ui::drawable::{Drawable, Color, Align};
use pelican_ui::{Application, include_dir, drawables, Assets, Component, Context, Plugin};
use pelican_ui::events::OnEvent;
use pelican_ui::layouts::{Offset, Stack};
use pelican_ui::components::avatar::{AvatarContent, AvatarIconStyle};
use pelican_ui::components::{Toggle, Icon, TextInput};
use pelican_ui::components::text::{TextSize, ExpandableText, TextStyle};
use pelican_ui::components::interface::general::{Bumper, Content, Header, Interface, Page};
use pelican_ui::components::RadioSelector;
use pelican_ui::components::interface::navigation::{AppPage, RootInfo};
use pelican_ui::components::list_item::{ListItemGroup, ListItem, ListItemInfoLeft};
use pelican_ui::components::interface::navigation::NavigationEvent;
use pelican_ui::plugin::PelicanUI;
use pelican_ui::theme::Theme;

// use crate::examples::motorcycle::AllBikes;
// use crate::examples::motorcycle::BuildBike;

use serde::{Serialize, Deserialize};

pub struct IceCreamApp;

impl Application for IceCreamApp {
    fn interface(ctx: &mut Context) -> Interface {
        // ctx.state().set(AllOrders::default());
        // ctx.state().set(AllBikes::default());

        let home = RootInfo::icon("cone", "Ice Cream", hatchling_ui::Orange::new(ctx));
        // let moto = RootInfo::icon("cruiser", "My Bikes", BuildBike::new(ctx).ok().unwrap());

        Interface::new(ctx, vec![home])
    }

    fn theme(assets: &mut Assets) -> Theme {
        assets.include_assets(pelican_ui::include_dir!("./resources"));
        Theme::dark(assets, Color::from_hex("#ff1f84ff", 255))
    }
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct IceCreamOrder {
//     flavor: String,
//     sprinkles: bool,
//     cream: bool,
//     cherry: bool,
//     nuts: bool,
//     marshmallows: bool,
//     cookie_bits: bool,
//     syrup: bool,
//     is_cup: bool,
// }

// impl Default for IceCreamOrder {
//     fn default() -> Self {
//         IceCreamOrder {
//             flavor: "Chocolate".to_string(),
//             sprinkles: false,
//             cream: false,
//             cherry: false,
//             nuts: false,
//             marshmallows: false,
//             cookie_bits: false,
//             syrup: false,
//             is_cup: true,
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Default, Debug)]
// pub struct AllOrders {
//     orders: Vec<IceCreamOrder>
// }

// #[derive(Debug, Component)]
// pub struct Order(Stack, Page);

// impl OnEvent for Order {}
// impl AppPage for Order {}

// impl Order {
//     pub fn new(ctx: &mut Context) -> Result<Self, String> {
//         ctx.state().set(IceCreamOrder::default());
//         let text = ExpandableText::new(ctx, "No orders yet.\nGet started by creating an order.", TextSize::Md, TextStyle::Primary, Align::Center, None);
//         let text1 = ExpandableText::new(ctx, "No orders yet.\nGet started by creating an order.", TextSize::Md, TextStyle::Primary, Align::Center, None);
//         let text2 = ExpandableText::new(ctx, "No orders yet.\nGet started by creating an order.", TextSize::Md, TextStyle::Primary, Align::Center, None);
//         let text3 = ExpandableText::new(ctx, "No orders yet.\nGet started by creating an order.", TextSize::Md, TextStyle::Primary, Align::Center, None);

//         let orders = ctx.state().get::<AllOrders>().unwrap().orders.clone();

//         let items = orders.into_iter().map(|o| {
//             let mut toppings = Vec::new();

//             if o.sprinkles { toppings.push("sprinkles"); }
//             if o.cream { toppings.push("whipped cream"); }
//             if o.cherry { toppings.push("cherry on top"); }
//             if o.nuts { toppings.push("crushed nuts"); }
//             if o.marshmallows { toppings.push("marshmallows"); }
//             if o.cookie_bits { toppings.push("cookie crumble"); }
//             if o.syrup { toppings.push("chocolate syrup"); }

//             let toppings_text = match toppings.is_empty() {
//                 true => "No toppings".to_string(),
//                 false => toppings.join(", ")
//             };

//             let container = if o.is_cup { "cup" } else { "cone" };
//             let subtitle = format!("{container}, {toppings_text}");
//             ListItem::new(ctx, Some(AvatarContent::Icon(container.to_string(), AvatarIconStyle::Brand)), ListItemInfoLeft::new(&o.flavor, &subtitle, None, None), None, None, None, |_| {})
//         }).collect::<Vec<ListItem>>();

//         let (offset, content) = match items.is_empty() {
//             true => (Offset::Center, drawables![text, text1, text2, text3]),
//             false => (Offset::Start, drawables![ListItemGroup::new(items)])
//         };


//         // stack_end = move |ctx: &mut Context| ctx.trigger_event(NavigationEvent::Pop(len));;
//         // stack = move |ctx: &mut Context| ctx.trigger_event(NavigationEvent::Push(next.take().unwrap()));
//         // stack home = move |ctx: &mut Context| ctx.trigger_event(NavigationEvent::Push(first.1))

//         let bumper = Bumper::home(ctx, ("Order Now", |ctx: &mut Context| {
//             let page = Box::new(CupOrCone::new(ctx).unwrap());
//             ctx.trigger_event(NavigationEvent::Push(Some(page)))
//         }), None);

//         let content = Content::new(ctx, offset, content);
//         let header = Header::home(ctx, "Ice Cream", None);

//         Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
//     }
// }

// #[derive(Debug, Component)]
// pub struct CupOrCone(Stack, Page);

// impl OnEvent for CupOrCone {}
// impl AppPage for CupOrCone {}

// impl CupOrCone {
//     pub fn new(ctx: &mut Context) -> Result<Self, String> {
//         let is_cup = ctx.state().get_mut::<IceCreamOrder>().map(|i| i.is_cup).unwrap_or(false);
//         let default = if is_cup {0} else {1};

//         let selector = RadioSelector::new(ctx, default, vec![
//             ("Cup", "6oz recycalable cup", Box::new(|ctx: &mut Context| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.is_cup = true })),
//             ("Cone", "Classic ice cream cone", Box::new(|ctx: &mut Context| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.is_cup = false })),
//         ]);

//         let bumper = Bumper::stack(ctx, None, false, |ctx: &mut Context| {
//             let page = Box::new(Flavor::new(ctx).unwrap());
//             ctx.trigger_event(NavigationEvent::Push(Some(page)))
//         });

//         let content = Content::new(ctx, Offset::Start, drawables![selector]);
//         let header = Header::stack(ctx, "Cup or cone");

//         Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
//     }
// }

// #[derive(Debug, Component)]
// pub struct Flavor(Stack, Page);

// impl OnEvent for Flavor {}
// impl AppPage for Flavor {}

// impl Flavor {
//     pub fn new(ctx: &mut Context) -> Result<Self, String> {
//         let default = ctx.state().get_mut::<IceCreamOrder>().map(|i| match i.flavor.as_str() {
//             "Chocolate" => 0,
//             "Vanilla" => 1,
//             "Strawberry" => 2,
//             "Salted Caramel" => 3,
//             "Stracciatella" => 4,
//             "Pistachio" => 5,
//             _ => 0,
//         }).unwrap_or(0);

//         let selector = RadioSelector::new(ctx, default, vec![
//             ("Chocolate", "Rich and classic cocoa goodness", Box::new(|ctx: &mut Context| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.flavor = "Chocolate".to_string() })),
//             ("Vanilla", "Smooth and timeless — a perfect base for toppings", Box::new(|ctx: &mut Context| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.flavor = "Vanilla".to_string() })),
//             ("Strawberry", "Sweet and fruity, made with real berries", Box::new(|ctx: &mut Context| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.flavor = "Strawberry".to_string() })),
//             ("Salted Caramel", "Sweet caramel with a pinch of salt", Box::new(|ctx: &mut Context| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.flavor = "Salted Caramel".to_string() })),
//             ("Stracciatella", "Creamy vanilla with delicate chocolate flakes", Box::new(|ctx: &mut Context| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.flavor = "Stracciatella".to_string() })),
//             ("Pistachio", "Nutty, sweet, and slightly exotic", Box::new(|ctx: &mut Context| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.flavor = "Pistachio".to_string() }))
//         ]);

//         let bumper = Bumper::stack(ctx, None, false, |ctx: &mut Context| {
//             let page = Box::new(Toppings::new(ctx).unwrap());
//             ctx.trigger_event(NavigationEvent::Push(Some(page)))
//         });

//         let content = Content::new(ctx, Offset::Start, drawables![selector]);
//         let header = Header::stack(ctx, "Choose flavor");

//         Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
//     }
// }

// #[derive(Debug, Component)]
// pub struct Toppings(Stack, Page);

// impl OnEvent for Toppings {}
// impl AppPage for Toppings {}

// impl Toppings {
//     pub fn new(ctx: &mut Context) -> Result<Self, String> {        
//         let default = ctx.state().get_mut::<IceCreamOrder>().map(|i| i.sprinkles).unwrap_or(false);
//         let sprinkles = Toggle::new(ctx, "Include sprinkles", default, |ctx: &mut Context, y: bool| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.sprinkles = y; });

//         let default = ctx.state().get_mut::<IceCreamOrder>().map(|i| i.cream).unwrap_or(false);
//         let cream = Toggle::new(ctx, "Include whipped cream", default, |ctx: &mut Context, y: bool| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.cream = y; });

//         let default = ctx.state().get_mut::<IceCreamOrder>().map(|i| i.cherry).unwrap_or(false);
//         let cherry = Toggle::new(ctx, "Add cherry on top", default, |ctx: &mut Context, y: bool| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.cherry = y; });

//         let default = ctx.state().get_mut::<IceCreamOrder>().map(|i| i.nuts).unwrap_or(false);
//         let nuts = Toggle::new(ctx, "Add crushed nuts", default, |ctx: &mut Context, y: bool| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.nuts = y; });

//         let default = ctx.state().get_mut::<IceCreamOrder>().map(|i| i.syrup).unwrap_or(false);
//         let syrup = Toggle::new(ctx, "Drizzle chocolate syrup", default, |ctx: &mut Context, y: bool| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.syrup = y; });

//         let default = ctx.state().get_mut::<IceCreamOrder>().map(|i| i.marshmallows).unwrap_or(false);
//         let mallows = Toggle::new(ctx, "Mini marshmallows", default, |ctx: &mut Context, y: bool| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.marshmallows = y; });

//         let default = ctx.state().get_mut::<IceCreamOrder>().map(|i| i.cookie_bits).unwrap_or(false);
//         let cookies = Toggle::new(ctx, "Cookie crumble", default, |ctx: &mut Context, y: bool| if let Some(i) = ctx.state().get_mut::<IceCreamOrder>() { i.cookie_bits = y; });

//         let bumper = Bumper::stack(ctx, None, false, |ctx: &mut Context| {
//             let page = Box::new(Success::new(ctx).unwrap());
//             ctx.trigger_event(NavigationEvent::Push(Some(page)))
//         });

//         let content = Content::new(ctx, Offset::Start, drawables![sprinkles, cream, cherry, nuts, syrup, mallows, cookies]);
//         let header = Header::stack(ctx, "Select toppings");

//         Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
//     }
// }

// #[derive(Debug, Component)]
// pub struct Success(Stack, Page);

// impl OnEvent for Success {}
// impl AppPage for Success {}

// impl Success {
//     pub fn new(ctx: &mut Context) -> Result<Self, String> {
//         let my_order = ctx.state().get::<IceCreamOrder>().unwrap().clone();
//         let colors = ctx.get::<PelicanUI>().get().0.theme().colors;
//         let text = ExpandableText::new(ctx, &format!("{} Ice Ordered", my_order.flavor), TextSize::H4, TextStyle::Heading, Align::Center, None);   
        
//         let icon = if my_order.is_cup {"cup"} else {"cone"};
//         let icon = Icon::new(ctx, icon, Some(colors.text.primary), 128.0);

//         let bumper = Bumper::stack_end(ctx);

//         let content = Content::new(ctx, Offset::Center, drawables![icon, text]);
//         let header = Header::stack_end(ctx, "Order completed");

//         if let Some(i) = ctx.state().get_mut::<AllOrders>() { i.orders.push(my_order) }

//         Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
//     }
// }
