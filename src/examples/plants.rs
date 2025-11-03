use roost::drawable::{Drawable, Color, Align};
use roost::{include_dir, drawables, Component, Context, Application, Plugin};
use roost::events::OnEvent;
use roost::layouts::{Offset, Stack};

use pelican::components::interface::navigation::PelicanError;
use pelican::components::avatar::{AvatarContent, AvatarIconStyle};
use pelican::components::{Toggle, TextSize, ExpandableText, Icon, TextStyle};
use pelican::components::interface::general::{Bumper, Content, Header, Interface, Page};
use pelican::plugin::PelicanUI;
use pelican::theme::Theme;
use pelican::components::RadioSelector;
use pelican::components::interface::navigation::{AppPage, RootInfo};
use pelican::components::list_item::{ListItemGroup, ListItem, ListItemInfoLeft};
use pelican::page;

use serde::{Serialize, Deserialize};

pub struct PlantGrowerApp;

impl Application for PlantGrowerApp {
    async fn new(ctx: &mut Context) -> impl Drawable {
        ctx.state().set(AllPlants::default());

        let home = RootInfo::icon("home", "My Plants", |ctx: &mut Context| Box::new(Home::new(ctx).ok().unwrap()) as Box<dyn AppPage>);

        Interface::new(ctx, (vec![home], None))
    }

    fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> {
        ctx.assets.include_assets(include_dir!("./resources"));
        let theme = Theme::light(&mut ctx.assets, Color::from_hex("#00bf69ff", 255));
        vec![Box::new(PelicanUI::new(ctx, theme))]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plant {
    name: String,
    variation: String,
    sunlight: f32,
    water: f32
}

impl Default for Plant {
    fn default() -> Self {
        Plant {
            name: "Sparklepuff".to_string(),
            variation: "Flower".to_string(),
            sunlight: 0.0,
            water: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AllPlants {
    plants: Vec<Plant>
}

#[derive(Debug, Component)]
pub struct Home(Stack, Page);

impl OnEvent for Home {}
impl AppPage for Home {
    fn has_navigator(&self) -> bool {true}
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) 
        -> Result<Box<dyn AppPage>, PelicanError> {
        match index {
            1 => page!(NewPlant::new(ctx), self),
            _ => Err(PelicanError::InvalidPage(Some(self)))
        }
    }
}

impl Home {
    pub fn new(ctx: &mut Context) -> Result<Self, String> {
        ctx.state().set(Plant::default());

        let text = ExpandableText::new(ctx, "No plants yet.\nGet started by planting a new seed.", TextSize::Md, TextStyle::Primary, Align::Center, None);

        let plants = ctx.state().get::<AllPlants>().unwrap().plants.clone();

        let items = plants.into_iter().map(|plant| {
            let icon = plant.variation.to_string().to_lowercase();
            ListItem::new(ctx, Some(AvatarContent::Icon(icon, AvatarIconStyle::Brand)), ListItemInfoLeft::new(&plant.name, &plant.variation, None, None), None, None, None, |_| {})
        }).collect::<Vec<ListItem>>();

        let (offset, content) = match items.is_empty() {
            true => (Offset::Center, drawables![text]),
            false => (Offset::Start, drawables![ListItemGroup::new(items)])
        };

        let bumper = Bumper::home(ctx, "Plant New Seed", None);
        let content = Content::new(ctx, offset, content);
        let header = Header::home(ctx, "My Plants", None);

        Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
    }
}

#[derive(Debug, Component)]
pub struct NewPlant(Stack, Page);

impl OnEvent for NewPlant {}
impl AppPage for NewPlant {
    fn has_navigator(&self) -> bool {true}
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) 
        -> Result<Box<dyn AppPage>, PelicanError> {
        match index {
            0 => page!(Home::new(ctx), self),
            1 => page!(PlantSummary::new(ctx), self),
            // 1 => page!(Toppings::new(ctx), self),
            _ => Err(PelicanError::InvalidPage(Some(self)))
        }
    }
}

impl NewPlant {
    pub fn new(ctx: &mut Context) -> Result<Self, String> {
        let default = ctx.state().get_mut::<Plant>().map(|i| match i.variation.as_str() {
            "Flower" => 0,
            "Christmas Tree" => 1,
            "Tomato" => 2,
            "Potato" => 3,
            _ => 0,
        }).unwrap_or(0);

        let selector = RadioSelector::new(ctx, default, vec![
            ("Flower", "The Sparklepuff™ blooms in rainbow shades every morning", Box::new(|ctx: &mut Context| if let Some(i) = ctx.state().get_mut::<Plant>() { i.variation = "Flower".to_string() })),
            ("Christmas Tree", "The Whistlestem™ grows so tall it might whistle in the wind", Box::new(|ctx: &mut Context| if let Some(i) = ctx.state().get_mut::<Plant>() { i.variation = "Tree".to_string() })),
            ("Tomato", "The Goldberry™ is known for its perfectly spherical, bouncy fruit", Box::new(|ctx: &mut Context| if let Some(i) = ctx.state().get_mut::<Plant>() { i.variation = "Tomato".to_string() })),
            ("Potato", "The SnuggleSpud™ loves naps in warm, sunny soil", Box::new(|ctx: &mut Context| if let Some(i) = ctx.state().get_mut::<Plant>() { i.variation = "Potato".to_string() })),
        ]);

        let bumper = Bumper::stack(ctx, false);
        let content = Content::new(ctx, Offset::Start, drawables![selector]);
        let header = Header::stack(ctx, "Choose seed");

        Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
    }
}

#[derive(Debug, Component)]
pub struct PlantSummary(Stack, Page);

impl OnEvent for PlantSummary {}
impl AppPage for PlantSummary {
    fn has_navigator(&self) -> bool { true }

    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) 
        -> Result<Box<dyn AppPage>, PelicanError> {
        match index {
            0 => page!(Home::new(ctx), self),
            1 => page!(Home::new(ctx), self),
            _ => Err(PelicanError::InvalidPage(Some(self))),
        }
    }
}

impl PlantSummary {
    pub fn new(ctx: &mut Context) -> Result<Self, String> {
        let plant = ctx.state().get::<Plant>().unwrap().clone();

        let icon = plant.variation.to_lowercase();
        let planted = Icon::new(ctx, &icon, None, 96.0);
        let dirt = Icon::new(ctx, "dirt", None, 128.0);

        let content = Content::new(ctx, Offset::Center, drawables![planted, dirt]);

        let bumper = Bumper::stack_end(ctx);
        let header = Header::stack_end(ctx, &format!("{} planted", plant.variation));

        Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
    }
}

// #[derive(Debug, Component)]
// pub struct PlantCare(Stack, Page);

// impl OnEvent for PlantCare {}
// impl AppPage for PlantCare {
//     fn has_navigator(&self) -> bool { true }

//     fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) 
//         -> Result<Box<dyn AppPage>, PelicanError> {
//         match index {
//             0 => page!(NewPlant::new(ctx), self),
//             1 => page!(PlantSummary::new(ctx), self),
//             _ => Err(PelicanError::InvalidPage(Some(self))),
//         }
//     }
// }

// impl PlantCare {
//     pub fn new(ctx: &mut Context) -> Result<Self, String> {
//         let plant = ctx.state().get_mut::<Plant>().unwrap();

//         let sunlight_slider = Slider::new(ctx, 0.0..=100.0, plant.sunlight, |ctx, val| {
//             if let Some(p) = ctx.state().get_mut::<Plant>() { p.sunlight = val; }
//         });
//         let water_slider = Slider::new(ctx, 0.0..=100.0, plant.water, |ctx, val| {
//             if let Some(p) = ctx.state().get_mut::<Plant>() { p.water = val; }
//         });

//         let content = Content::new(ctx, Offset::Start, drawables![
//             Text::new(ctx, "Adjust sunlight", TextSize::Md, TextStyle::Primary),
//             sunlight_slider,
//             Text::new(ctx, "Adjust water", TextSize::Md, TextStyle::Primary),
//             water_slider
//         ]);

//         let bumper = Bumper::stack(ctx, false);
//         let header = Header::stack(ctx, "Plant Care");

//         Ok(Self(Stack::default(), Page::new(header, content, Some(bumper))))
//     }
// }
