use gui::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>, // values that the user can select
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to draw a selectbox - omitted in our example
        println!(
            "Drawing a SelectBox, width: {}, height: {}, options: {:?}",
            self.width, self.height, self.options
        )
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("ok"),
            }),
        ],
    };

    screen.run();
}
