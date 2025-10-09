use mustache::drawable::{Color, Align};
use mustache::{drawables, start, Component, Context};
use mustache::events::OnEvent;

use pelican::components::{TextInput, Slider, ExpandableText, Icon, Text, TextStyle};
use pelican::components::button::{PrimaryButton, SecondaryButton};
use pelican::components::interface::general::{Bumper, Content, Header, Interface, Page};
use pelican::layout::{Offset, Stack};
use pelican::plugin::PelicanUI;
use pelican::components::interface::navigation::AppPage;

// Define the main application struct. This is our entry point type.
pub struct MyApp;

// Implement the Application trait for MyApp
impl mustache::Application for MyApp {
    // Asynchronously create the main drawable UI component
    async fn new(ctx: &mut Context) -> impl mustache::drawable::Drawable {
        // Create the first screen
        let home = FirstScreen::new(ctx);
        // Return the main interface with the first screen as the starting page
        Interface::new(ctx, home, None)
    }

    // Add the plugins you app will use
    fn plugins(ctx: &mut Context) -> Vec<Box<dyn mustache::Plugin>> {
        // Create theme from a custom color.
        let theme = pelican::theme::Theme::from(&mut ctx.assets, Color::from_hex("#00ff55ff", 255));
        // Return a vector of plugins, including the PelicanUI plugin with the created theme
        vec![Box::new(PelicanUI::new(ctx, theme))]
    }
}

// Macro to start the application
start!(MyApp);

// Define the first screen of the app
#[derive(Debug, Component)]
pub struct FirstScreen(Stack, Page);

// Implement event handling for FirstScreen (empty for now)
impl OnEvent for FirstScreen {}

// Implement the AppPage trait for navigation and UI behavior
impl AppPage for FirstScreen {
    // This screen does not have a navigation bar
    fn has_nav(&self) -> bool { false }

    // Handle page navigation. Always returns Err(self) because this page cannot navigate.
    fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        Err(self)
    }
}

impl FirstScreen {
    pub fn new(ctx: &mut Context) -> Self {
        let header = Header::home(ctx, "My Screen", None);

        let font_size = ctx.get::<PelicanUI>().get().0.theme().fonts.size;
        let color = ctx.get::<PelicanUI>().get().0.theme().colors.brand;

        let icon = Icon::new(ctx, "pelican_ui",  color, 128.0);
        let text = Text::new(ctx, "Hello World!", font_size.h2, TextStyle::Heading, Align::Center, None);
        let subtext = ExpandableText::new(ctx, "First project loaded successfully.", font_size.md, TextStyle::Primary, Align::Center, None);
        let slider = Slider::new(ctx, 50.0, None, None, |ctx: &mut Context, p: f32| println!("SLIDERrrrr"));
        let text_input = TextInput::new(ctx, Some("Label"), Some("What?"), "hey", None, None);
        let button = SecondaryButton::medium(ctx, "edit", "Hello", None, |_ctx: &mut Context| println!("Hey, there!"));

        let content = Content::new(
            ctx, Offset::Start,
            drawables![icon, text, subtext, slider, text_input, button]
        );

        let button = PrimaryButton::new(ctx, "Click Me!", |_ctx: &mut Context| println!("Hey, there!"), false);
        let bumper = Bumper::new(ctx, drawables![button]);

        FirstScreen(Stack::default(), Page::new(Some(header), content, Some(bumper)))
    }
    // pub fn new(ctx: &mut Context) -> Self {
    //     // Create a header for the page
    //     let header = Header::home(
    //         // The majority of UI components will require the app context.
    //         ctx,
    //         // The text on this header will say "My Screen"
    //         "My Screen", 
    //         // There will not be an icon button on this header
    //         None
    //     );

    //     let font_size = ctx.get::<PelicanUI>().get().0.theme().fonts.size;
    //     let color = ctx.get::<PelicanUI>().get().0.theme().colors.brand;

    //     // Create an icon element
    //     let icon = Icon::new(
    //         // This element requires the app context
    //         ctx, 
    //         // We choose the "pelican_ui" icon
    //         "pelican_ui", 
    //         // The color of the icon
    //         color, 
    //         // The size of the icon. Icons are always square.
    //         128.0
    //     );

    //     // Create the main heading text
    //     let text = Text::new(
    //         ctx,
    //         // This text will say "Hello World!"
    //         "Hello World!",
    //         // The size will be h2
    //         font_size.h2,
    //         // The style of this text will be heading
    //         TextStyle::Heading,
    //         // The text alignment
    //         Align::Center,
    //         // No max lines
    //         None
    //     );

    //     // Create subtext.
    //     let subtext = ExpandableText::new(
    //         ctx,
    //         "First project loaded successfully.",
    //         // Medium font size
    //         font_size.md,
    //         // This text will have primary text style.
    //         TextStyle::Primary,
    //         // Center the text
    //         Align::Center,
    //         // No max lines
    //         None
    //     );

    //     // Combine icon, heading, and subtext into page content
    //     let content = Content::new(
    //         ctx,
    //         // Vertically center items
    //         Offset::Center,
    //         // Use the drawables macro for the content's items
    //         drawables![icon, text, subtext]
    //     );

    //     // Create a button with the text "Click Me!" that prints "Hey, there!" on every press.
    //     let button = PrimaryButton::new(ctx, "Click Me!", |_ctx: &mut Context| println!("Hey, there!"), false);
    //     // Place the button in the bumper portion of the screen.
    //     let bumper = Bumper::new(ctx, drawables![button]);

    //     // Return the FirstScreen with a default Stack and a 
    //     // new Page containinhg our header, content, and bumper.
    //     FirstScreen(Stack::default(), Page::new(Some(header), content, Some(bumper)))
    // }
}
