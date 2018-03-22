extern crate azul;

use azul::prelude::*;

const TEST_CSS: &str = include_str!("test_content.css");
const TEST_FONT: &[u8] = include_bytes!("../assets/fonts/weblysleekuil.ttf");
const TEST_IMAGE: &[u8] = include_bytes!("../assets/images/cat_image.jpg");

#[derive(Debug)]
pub struct MyAppData {
    // Your app data goes here
    pub my_data: u32,
}

impl LayoutScreen for MyAppData {

    fn get_dom(&self, _window_id: WindowId) -> Dom<MyAppData> {

        let mut dom = Dom::new(NodeType::Div);
        dom.class("__azul-native-button");
        dom.event(On::MouseUp, Callback::Sync(my_button_click_handler));
        
        for i in 0..self.my_data {
            dom.add_sibling(Dom::new(NodeType::Label { 
                text: format!("{}", i),
            }));
        }

        dom
    }
}

fn my_button_click_handler(app_state: &mut AppState<MyAppData>) -> UpdateScreen {
    app_state.data.my_data += 1;
    UpdateScreen::Redraw
}

fn main() {
    let css = Css::new_from_string(TEST_CSS).unwrap();

    let my_app_data = MyAppData {
        my_data: 0,
    };

    let mut app = App::new(my_app_data);

    app.add_font("Webly Sleeky UI", TEST_FONT).unwrap();
    app.remove_font("Webly Sleeky UI");

    app.add_image("MyImage", &mut TEST_IMAGE, ImageType::Jpeg).unwrap();
    app.remove_image("MyImage");

    // TODO: Multi-window apps currently crash
    // Need to re-factor the event loop for that
    app.create_window(WindowCreateOptions::default(), css).unwrap();
    app.start_render_loop();
}
