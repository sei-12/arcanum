use crate::game_view::{GamePage, GameViewMessage};
use iced::widget::{button, column};
use iced::{Element, Subscription};

mod assets;
mod char_skill;
mod common;
mod game_view;
mod ui_state;
// mod view;

pub fn main() -> iced::Result {
    iced::application("Pixels", MainApp::update, MainApp::view)
        .subscription(MainApp::subscription)
        .font(include_bytes!("../assets/fonts/KaiseiOpti-Regular.ttf"))
        .default_font(iced::Font {
            family: iced::font::Family::Name("Kaisei Opti"),
            weight: iced::font::Weight::Normal,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::Normal,
        })
        .run()
}

trait PageTrait {
    fn view(&self) -> Element<'_, MainAppMessage>;
    fn update(&mut self, msg: MainAppMessage) -> Option<Box<dyn PageTrait>>;

    #[allow(unused_variables)]
    fn subscription(&self, subscriptions: &mut Vec<Subscription<MainAppMessage>>) {}
}

#[derive(Debug, Clone)]
enum MainAppMessage {
    GameViewMessage(GameViewMessage),
    HomePageMessage(HomePageMessage),
}

struct MainApp {
    page: Box<dyn PageTrait>,
}
impl Default for MainApp {
    fn default() -> Self {
        Self {
            page: Box::new(HomePage),
        }
    }
}

impl MainApp {
    fn update(&mut self, msg: MainAppMessage) {
        if let Some(next_page) = self.page.update(msg) {
            self.page = next_page;
        }
    }

    fn view(&self) -> Element<'_, MainAppMessage> {
        self.page.view()
    }

    fn subscription(&self) -> Subscription<MainAppMessage> {
        let mut subscriptions = Vec::new();
        self.page.subscription(&mut subscriptions);
        Subscription::batch(subscriptions)
    }
}

//--------------------------------------------------//
//                                                  //
//                    HOME PAGE                     //
//                                                  //
//--------------------------------------------------//
#[derive(Debug, Clone)]
enum HomePageMessage {
    StartNewGame,
}
struct HomePage;
impl PageTrait for HomePage {
    fn update(&mut self, msg: MainAppMessage) -> Option<Box<dyn PageTrait>> {
        let MainAppMessage::HomePageMessage(msg) = msg else {
            return None;
        };

        match msg {
            HomePageMessage::StartNewGame => Some(Box::new(GamePage::new())),
        }
    }

    fn view(&self) -> Element<'_, MainAppMessage> {
        column![
            button("start game").on_press(MainAppMessage::HomePageMessage(
                HomePageMessage::StartNewGame
            ))
        ]
        .into()
    }
}
