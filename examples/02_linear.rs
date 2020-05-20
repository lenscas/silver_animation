//this is a more advanced version of simple_linear and shares the same idea
//The main diffrence is that instead of just presenting a diffrent image every animation frame
//we have full control over what we render

//In this example we improve upon example 1 to make a roating square but use some math instead of images to allow for much smaller step sizes

use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Color, Graphics, Image},
    run, Input, Result, Settings, Timer, Window,
};
use silver_animation::LinearConfig;
use silver_animation::SimpleLinearConfig;
fn main() {
    run(
        Settings {
            size: Vector::new(800.0, 600.0).into(),
            title: "Linear Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut inputs: Input) -> Result<()> {
    //load the image that we want to use for our animation
    let image = Image::load(&gfx, "img1.png").await?;
    //set how long every frame takes
    let timing = Timer::time_per_second(30.);

    //now, we set how big the tint changes is during every animation frame
    let step_size: f32 = 5.;
    //and then calculate how many frames we need to reach the end
    let amount_of_steps = (360. / step_size).ceil() as usize;
    let mut animation = LinearConfig {
        begin_state: image, //this is the state we start with. We can change this every draw call if we want, but for this example that is not needed
        timing,
        //here we define how to draw the animation
        //state = what we defined as begin_state, in this case the image of our square
        //tick = which animation frame we are currently at
        //gfx = the Graphics system
        //location = where on the screen we are supposed to draw it
        draw: |state, tick, gfx, location| {
            let origin = location.center();
            let rotation = step_size * tick as f32;
            gfx.set_transform(
                Transform::translate(origin)
                    * Transform::rotate(rotation)
                    * Transform::translate(-origin),
            );
            gfx.draw_image(state, location);
            gfx.set_transform(Transform::IDENTITY);
            Ok(())
        },
        //how many frames our animation has
        max_frames: |_| amount_of_steps,
    }
    .to_animation();

    let location = Rectangle::new((200, 200), (200, 200));
    gfx.clear(Color::WHITE);
    gfx.present(&window)?;

    loop {
        while let Some(_) = inputs.next_event().await {}
        gfx.clear(Color::WHITE);
        animation.draw(&mut gfx, location)?;
        gfx.present(&window)?;
    }
}
