use pelican_ui::{Component, Context, Plugins, Plugin, maverick_start, start, Application, PelicanEngine, MaverickOS};
use pelican_ui::drawable::{Drawable, Component, Align};
use pelican_ui::runtime::{Services, ServiceList};
use pelican_ui::layout::{Layout, SizeRequest, Area};
use pelican_ui::events::OnEvent;
use std::collections::BTreeMap;

use pelican_ui_std::{
    Interface, Stack, 
    Page, Text, TextStyle,
    Offset, Content, Icon,
    ExpandableText, Header,
    AppPage,
};

pub struct MyApp;
impl Services for MyApp {
    fn services() -> ServiceList {ServiceList(BTreeMap::new())}
}

impl Plugins for MyApp {
    fn plugins(_ctx: &mut Context) -> Vec<Box<dyn Plugin>> {vec![]}
}

impl Application for MyApp {
    async fn new(ctx: &mut Context) -> Box<dyn Drawable> {
        let home = FirstScreen::new(ctx);
        let interface = Interface::new(ctx, Box::new(home), None, None);
        Box::new(interface)
    }
}

start!(MyApp);

#[derive(Debug, Component)]
pub struct FirstScreen(Stack, Page);
impl OnEvent for FirstScreen {}

impl AppPage for FirstScreen {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { Err(self) }
}

impl FirstScreen {
    pub fn new(ctx: &mut Context) -> Self {
        let header = Header::home(ctx, "My Screen", None);
        let font_size = ctx.theme.fonts.size;
        let color = ctx.theme.colors.text.heading;
        let icon = Icon::new(ctx, "pelican_ui", color, 128.0);
        let text = Text::new(ctx, "Hello World!", TextStyle::Heading, font_size.h2, Align::Center);
        let subtext = ExpandableText::new(ctx, "First project loaded successfully.", TextStyle::Primary, font_size.md, Align::Center, None);
        let content = Content::new(ctx, Offset::Center, vec![Box::new(icon), Box::new(text), Box::new(subtext)]);
        FirstScreen(Stack::default(), Page::new(Some(header), content, None))
    }
}