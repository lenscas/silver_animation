## Alpha notice
Although I am happy with the current api provided by silver_animation , it is targeting the an alpha version of quicksilver.
This means that things can (and probably will) break or change one day or another for one reason or another.

# silver_animation
silver_animation is a simple and basic animation system for quicksilver.
It allows you to either use a list of images or take full control and render something however you want.

It also provides a set of traits that provide a base on how to interact with animations. These can be used as building blocks for your own animation system(s)

## example
```rust
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Color, Graphics, Image},
    run, Input, Result, Settings, Timer, Window,
};
use silver_animation::SimpleLinearConfig;
use silver_animation::LinearConfig;

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

async fn app(window: Window, mut gfx: Graphics, mut inputs: Input) -> Result<()> {
    let images = vec![
        Image::load(&gfx, "img1.png").await?,
        Image::load(&gfx, "img2.png").await?,
        Image::load(&gfx, "img3.png").await?,
        Image::load(&gfx, "img4.png").await?,
    ];
    let timing = Timer::time_per_second(8.);
    let mut simple_animation = SimpleLinearConfig { images, timing }.to_animation();

    let image = Image::load(&gfx, "img1.png").await?;
    let timing = Timer::time_per_second(30.);

    let step_size: f32 = 5.;
    let amount_of_steps = (255. / step_size).ceil() as usize;
    let mut custom_animation = LinearConfig {
        begin_state: image,
        timing,
        draw: |state, tick, gfx, location| {
            gfx.draw_image_tinted(
                state,
                location,
                Color::from_rgba(0, (step_size * tick as f32) as u8, 0, 1.0),
            );
            Ok(())
        },
        max_frames: |_| amount_of_steps,
    }
    .to_animation();

    let simple_animation_location = Rectangle::new((100, 100), (100, 100));
    let custom_animation_location = Rectangle::new((100, 150), (100, 100));
    gfx.clear(Color::WHITE);
    gfx.present(&window)?;

    loop {
        while let Some(_) = inputs.next_event().await {}
        gfx.clear(Color::WHITE);
        simple_animation.draw(&mut gfx, simple_animation_location)?;
        custom_animation.draw(&mut gfx, custom_animation_location)?;
        gfx.present(&window)?;
    }
}
```
