use pelican_ui::drawable::{Drawable, Color, Align};
use pelican_ui::{include_dir, drawables, Component, Context, Application, Plugin};
use pelican_ui::events::{OnEvent, Event, TickEvent};
use pelican_ui::layouts::{Offset, Stack};
use pelican_ui::components::button::PrimaryButton;
use pelican_ui::components::avatar::{AvatarContent, AvatarIconStyle};
use pelican_ui::components::{TextInput, Toggle, TextSize, ExpandableText, Icon, TextStyle};
use pelican_ui::components::interface::general::{Bumper, Content, Header, Interface, Page};
use pelican_ui::plugin::PelicanUI;
use pelican_ui::theme::Theme;
use pelican_ui::components::RadioSelector;
use pelican_ui::components::interface::navigation::{AppPage, RootInfo};
use pelican_ui::components::list_item::{ListItemGroup, ListItem, ListItemInfoLeft};
use pelican_ui::components::interface::navigation::NavigationEvent;

use serde::{Serialize, Deserialize};

pub struct MotorcycleApp;

// impl Application for MotorcycleApp {
//     async fn new(ctx: &mut Context) -> impl Drawable {
//         ctx.state().set(AllBikes::default());
//         let home = RootInfo::icon("home", "Motorcycles", BuildBike::new(ctx).unwrap());

//         Interface::new(ctx, (vec![home], None))
//     }

//     fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> {
//         ctx.assets.include_assets(include_dir!("./resources"));
//         let theme = Theme::dark(&mut ctx.assets, Color::from_hex("#ff1f23", 255));
//         vec![Box::new(PelicanUI::new(ctx, theme))]
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bike {
    engine: String,
    frame: String,
    heated_grips: bool,
    saddlebags: bool,
    paint_shade: u8,
    nickname: String,
}

impl Default for Bike {
    fn default() -> Self {
        Bike {
            engine: "Yamaha R1 1000cc Inline-4".to_string(),
            frame: "Sport".to_string(),
            heated_grips: false,
            saddlebags: false,
            paint_shade: 50,
            nickname: "My Bike".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AllBikes {
    bikes: Vec<Bike>
}

#[derive(Debug, Component)]
pub struct BuildBike(Stack, Page);

impl OnEvent for BuildBike {}
impl AppPage for BuildBike {}

impl BuildBike {
    pub fn new(ctx: &mut Context) -> Result<Self, String> {
        ctx.state().set(Bike::default());

        let text = ExpandableText::new(ctx, "No bikes yet.\nStart building your dream motorcycle.", TextSize::Md, TextStyle::Primary, Align::Center, None);

        let bikes = ctx.state().get::<AllBikes>().unwrap().bikes.clone();

        let items = bikes.into_iter().map(|b| {
            let accessories: Vec<&str> = vec![
                if b.heated_grips {Some("heated grips")} else {None},
                if b.saddlebags {Some("saddlebags")} else {None},
            ].into_iter().flatten().collect();

            let accessories_text = match accessories.is_empty() {
                true => "No accessories".to_string(),
                false => accessories.join(", ")
            };

            let icon = b.frame.to_lowercase();

            ListItem::new(ctx, Some(AvatarContent::Icon(icon, AvatarIconStyle::Brand)), ListItemInfoLeft::new(&format!("{}, {}", b.engine, b.nickname), &format!("{}, {}", b.frame, accessories_text), None, None), None, None, None, |_| {})
        }).collect::<Vec<ListItem>>();

        let (offset, content) = match items.is_empty() {
            true => (Offset::Center, drawables![text]),
            false => (Offset::Start, drawables![ListItemGroup::new(items)])
        };

        let bumper = Bumper::home(ctx, ("Build Bike", |ctx: &mut Context| {
            let page = Box::new(FramePage::new(ctx).unwrap());
            ctx.trigger_event(NavigationEvent::Push(Some(page)))
        }), None);

        let content = Content::new(ctx, offset, content);
        let header = Header::home(ctx, "Motorcycles", None);

        Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
    }
}

#[derive(Debug, Component)]
pub struct FramePage(Stack, Page);

impl OnEvent for FramePage {}
impl AppPage for FramePage {}

impl FramePage {
    pub fn new(ctx: &mut Context) -> Result<Self, String> {
        let bike = ctx.state().get::<Bike>().unwrap().clone();
        let default = match bike.frame.as_str() {
            "Sport" => 0,
            "Cruiser" => 1,
            "Adventure" => 2,
            _ => 0
        };

        let selector = RadioSelector::new(ctx, default, vec![
            ("Sport", "Aggressive sport frame, perfect for track days with your R1", Box::new(|ctx| if let Some(b) = ctx.state().get_mut::<Bike>() { b.frame = "Sport".to_string() })),
            ("Cruiser", "Low-slung cruiser frame for torque-heavy V-twins", Box::new(|ctx| if let Some(b) = ctx.state().get_mut::<Bike>() { b.frame = "Cruiser".to_string() })),
            ("Adventure", "Tall adventure frame with long travel suspension for touring", Box::new(|ctx| if let Some(b) = ctx.state().get_mut::<Bike>() { b.frame = "Adventure".to_string() })),
        ]);

        let bumper = Bumper::stack(ctx, None, false, |ctx: &mut Context| {
            let page = Box::new(EnginePage::new(ctx).unwrap());
            ctx.trigger_event(NavigationEvent::Push(Some(page)))
        });

        let content = Content::new(ctx, Offset::Start, drawables![selector]);
        let header = Header::stack(ctx, "Choose Frame");

        Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
    }
}


#[derive(Debug, Component)]
pub struct EnginePage(Stack, Page);

impl OnEvent for EnginePage {}
impl AppPage for EnginePage {}

impl EnginePage {
    pub fn new(ctx: &mut Context) -> Result<Self, String> {
        let bike = ctx.state().get::<Bike>().unwrap().clone();

        let selector = match bike.frame.as_str() {
            "Sport" => {
                let options = vec![
                    ("Yamaha R1 998cc Inline-4", "Race-bred supersport with crossplane crank."),
                    ("Kawasaki ZX-10R 998cc Inline-4", "Track-tuned liter bike with savage top-end."),
                    ("Suzuki GSX-R1000 999cc Inline-4", "Smooth Gixxer power with razor precision."),
                ];
                let default = options.iter().position(|(n, _)| n == &bike.engine).unwrap_or(0);
                RadioSelector::new(ctx, default, options.into_iter().map(|(name, desc)| (
                    name, desc, Box::new(|ctx: &mut Context| if let Some(b) = ctx.state().get_mut::<Bike>() { b.engine = name.to_string(); }) as Box<dyn FnMut(&mut Context) + 'static>
                )).collect::<Vec<(&str, &str, Box<dyn FnMut(&mut Context)>)>>())
            }

            "Cruiser" => {
                let options = vec![
                    ("Indian Thunderstroke 116 V-Twin", "Silky smooth American muscle."),
                    ("Honda Rebel 1100 Parallel Twin (DCT)", "Modern twin with easygoing auto clutch."),
                ];
                let default = options.iter().position(|(n, _)| n == &bike.engine).unwrap_or(0);
                RadioSelector::new(ctx, default, options.into_iter().map(|(name, desc)| (
                    name, desc, Box::new(|ctx: &mut Context| if let Some(b) = ctx.state().get_mut::<Bike>() { b.engine = name.to_string(); }) as Box<dyn FnMut(&mut Context) + 'static>
                )).collect::<Vec<(&str, &str, Box<dyn FnMut(&mut Context)>)>>())
            }

            "Adventure" | _ => {
                let options = vec![
                    ("BMW R1250GS Boxer Twin", "Iconic boxer with endless touring torque."),
                    ("KTM 1290 Super Adventure V-Twin", "Raw rally power with off-road edge."),
                    ("Honda Africa Twin 1100 Parallel Twin", "Balanced and unstoppable overland motor."),
                ];
                let default = options.iter().position(|(n, _)| n == &bike.engine).unwrap_or(0);
                RadioSelector::new(ctx, default, options.into_iter().map(|(name, desc)| (
                    name, desc, Box::new(|ctx: &mut Context| if let Some(b) = ctx.state().get_mut::<Bike>() { b.engine = name.to_string(); }) as Box<dyn FnMut(&mut Context) + 'static>
                )).collect::<Vec<(&str, &str, Box<dyn FnMut(&mut Context)>)>>())
            }
        };

        let bumper = Bumper::stack(ctx, None, false, |ctx: &mut Context| {
            let page = Box::new(AccessoriesPage::new(ctx).unwrap());
            ctx.trigger_event(NavigationEvent::Push(Some(page)))
        });

        let content = Content::new(ctx, Offset::Start, drawables![selector]);
        let header = Header::stack(ctx, "Choose engine");

        Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
    }
}

#[derive(Debug, Component)]
pub struct AccessoriesPage(Stack, Page);

impl OnEvent for AccessoriesPage {}
impl AppPage for AccessoriesPage {}

impl AccessoriesPage {
    pub fn new(ctx: &mut Context) -> Result<Self, String> {
        let bike = ctx.state().get::<Bike>().unwrap().clone();

        let heated_grips = Toggle::new(ctx, "Heated grips", bike.heated_grips, |ctx: &mut Context, val: bool| if let Some(b) = ctx.state().get_mut::<Bike>() { b.heated_grips = val });
        let saddlebags = Toggle::new(ctx, "Saddlebags", bike.saddlebags, |ctx: &mut Context, val: bool| if let Some(b) = ctx.state().get_mut::<Bike>() { b.saddlebags = val });

        let bumper = Bumper::stack(ctx, None, false, |ctx: &mut Context| {
            let page = Box::new(BikeName::new(ctx).unwrap());
            ctx.trigger_event(NavigationEvent::Push(Some(page)))
        });

        let content = Content::new(ctx, Offset::Start, drawables![heated_grips, saddlebags]);
        let header = Header::stack(ctx, "Add Accessories");

        Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
    }
}


#[derive(Debug, Component)]
pub struct BikeName(Stack, Page);

impl AppPage for BikeName {}

impl BikeName {
    pub fn new(ctx: &mut Context) -> Result<Self, String> {
        
        let bike = ctx.state().get::<Bike>().unwrap().clone();
        let default = bike.nickname;

        let input = TextInput::new(ctx, Some(&default), Some("Bike nickname"), Some("Enter nickname..."), None, None);

        let bumper = Bumper::stack(ctx, None, false, |ctx: &mut Context| {
            let page = Box::new(BikeSummary::new(ctx).unwrap());
            ctx.trigger_event(NavigationEvent::Push(Some(page)))
        });

        let content = Content::new(ctx, Offset::Start, drawables![input]);
        let header = Header::stack(ctx, "Name bike");

        Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
    }
}

impl OnEvent for BikeName {
    fn on_event(&mut self, _ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> { 
        if event.as_any().downcast_ref::<TickEvent>().is_some() { 
            let is_disabled = self.1.content().find::<TextInput>().unwrap().value().is_empty();
            self.1.bumper().as_mut().unwrap().find::<PrimaryButton>().unwrap().1.disable(is_disabled);
        }

        vec![event]
    }
}

#[derive(Debug, Component)]
pub struct BikeSummary(Stack, Page);

impl OnEvent for BikeSummary {}
impl AppPage for BikeSummary {}

impl BikeSummary {
    pub fn new(ctx: &mut Context) -> Result<Self, String> {
        let bike = ctx.state().get::<Bike>().unwrap().clone();

        let text = ExpandableText::new(ctx, &format!("{} bike created", bike.frame), TextSize::H4, TextStyle::Heading, Align::Center, None);   
        let icon = Icon::new(ctx, &bike.frame.to_lowercase(), None, 128.0);

        let content = Content::new(ctx, Offset::Center, drawables![icon, text]);

        let bumper = Bumper::stack_end(ctx, |ctx: &mut Context| {
            for _ in 0..4 { ctx.trigger_event(NavigationEvent::Pop) }
        });

        let header = Header::stack_end(ctx, "Created bike", 4);

        if let Some(i) = ctx.state().get_mut::<AllBikes>() { i.bikes.push(bike) }

        Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
    }
}
