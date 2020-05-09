//This is the most simple animation system
//It consists out of a series of images and a timer decides which one to display at any given moment

//In this example we load in 4 images to make a roating square

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics, Image},
    lifecycle::{run, EventStream, Settings, Window},
    Result, Timer,
};
use silver_animation::linear::SimpleLinearConfig;

fn main() {
    run(
        Settings {
            size: Vector::new(800.0, 600.0).into(),
            title: "Simple Linear Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
    //load the images that you want to display
    let images = vec![
        Image::load(&gfx, "img1.png").await?,
        Image::load(&gfx, "img2.png").await?,
        Image::load(&gfx, "img3.png").await?,
        Image::load(&gfx, "img4.png").await?,
    ];
    //set how long every image gets shown
    let timing = Timer::time_per_second(8.);
    let mut animation = SimpleLinearConfig { images, timing }.to_animation();
    let location = Rectangle::new((200, 200), (200, 200));
    gfx.clear(Color::WHITE);
    gfx.present(&window)?;

    loop {
        while let Some(_) = events.next_event().await {}
        gfx.clear(Color::WHITE);
        animation.draw(&mut gfx, location)?;
        gfx.present(&window)?;
    }
}
