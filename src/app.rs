use relm4::RelmApp;

use crate::{
    kbd::Keyboard,
    ptr::Pointer,
    ui::{UIMessage, UIModel},
};

pub struct App {
    ui: RelmApp<UIMessage>,
    keyboard: Keyboard,
    pointer: Pointer,
}
impl App {
    pub fn new() -> Self {
        let ui = RelmApp::<UIMessage>::new("vkbd");
        let styles = include_str!("../assets/style.css");
        relm4::set_global_css_with_priority(styles, relm4::gtk::STYLE_PROVIDER_PRIORITY_USER);

        App {
            ui,
            pointer: Pointer {},
            keyboard: Keyboard::new(),
        }
    }

    pub fn run(self) {
        self.ui.run::<UIModel>((self.keyboard, self.pointer));
    }
}
