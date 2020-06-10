use crate::{contained::BasicAnimationContainer, BasicAnimation, Resetable};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Graphics, Image},
    Result, Timer,
};

fn simple_draw(
    state: &mut Vec<Image>,
    frame: usize,
    gfx: &mut Graphics,
    location: Rectangle,
) -> Result<()> {
    let img = state.get(frame).unwrap_or_else(|| {
        panic!(
            "Error getting animation frame. Wanted : {}, max size : {}",
            frame,
            state.len()
        )
    });
    gfx.draw_image(img, location);
    Ok(())
}

fn simple_get_len(frames: &[Image]) -> usize {
    frames.len()
}

///The easiest animation system. It renders the images in order, switching to the next one based on the animation speed.
pub struct SimpleLinearConfig {
    ///The images that make up the animation.
    pub images: Vec<Image>,
    ///Controls how fast the animation plays.
    pub timing: Timer,
}

pub type SimpleLinearGetSize = dyn Fn(&Vec<Image>) -> usize;
pub type SimpleLinearConfigDraw =
    dyn Fn(&mut Vec<Image>, usize, &mut Graphics, Rectangle) -> Result<()>;

impl SimpleLinearConfig {
    ///Turn the config into an actual animation struct.
    pub fn into_animation(
        self,
    ) -> Linear<Vec<Image>, Box<SimpleLinearConfigDraw>, Box<SimpleLinearGetSize>> {
        Linear::new(LinearConfig {
            begin_state: self.images,
            timing: self.timing,
            draw: Box::new(simple_draw),
            max_frames: Box::new(|v| simple_get_len(v)),
        })
    }
}

///A more advanced animation than SimpleLinearConfig.
///It follows the same idea however you are in full control over what is used to draw images and how they are drawn.
pub struct LinearConfig<T, DrawFunc, MaxFrames>
where
    DrawFunc: Fn(&mut T, usize, &mut Graphics, Rectangle) -> Result<()>,
    MaxFrames: Fn(&T) -> usize,
{
    ///Data needed to draw the animation. This can be mutated during draw calls if needed.
    pub begin_state: T,
    ///Controls how fast the animation plays.
    pub timing: Timer,
    ///The function that will actuall draw the current animation frame.
    pub draw: DrawFunc,
    ///How many frames the animation has before it loops.
    pub max_frames: MaxFrames,
}

impl<T, DrawFunc, MaxFrames> LinearConfig<T, DrawFunc, MaxFrames>
where
    DrawFunc: Fn(&mut T, usize, &mut Graphics, Rectangle) -> Result<()>,
    MaxFrames: Fn(&T) -> usize,
{
    ///Turn the config into an actual animation.
    pub fn into_animation(self) -> Linear<T, DrawFunc, MaxFrames> {
        Linear::new(self)
    }
}

///The backbone of both Linear animation systems.
pub struct Linear<T, DrawFunc, MaxFrames>
where
    DrawFunc: Fn(&mut T, usize, &mut Graphics, Rectangle) -> Result<()>,
    MaxFrames: Fn(&T) -> usize,
{
    config: LinearConfig<T, DrawFunc, MaxFrames>,
    last_frame: usize,
}
impl<T, DrawFunc, MaxFrames> Linear<T, DrawFunc, MaxFrames>
where
    DrawFunc: Fn(&mut T, usize, &mut Graphics, Rectangle) -> Result<()>,
    MaxFrames: Fn(&T) -> usize,
{
    ///Create a new animation.
    pub fn new(config: LinearConfig<T, DrawFunc, MaxFrames>) -> Self {
        Self {
            config,
            last_frame: 0,
        }
    }
    ///Draw the animation.
    pub fn draw(&mut self, gfx: &mut Graphics, location: Rectangle) -> Result<()> {
        <Self as BasicAnimation<_, _>>::draw(self, gfx, location)
    }

    pub fn contain(self, location: Rectangle) -> BasicAnimationContainer<Self, Vector, Rectangle> {
        <Self as BasicAnimation<_, _>>::contain(self, location)
    }
}
impl<T, DrawFunc, MaxFrames> BasicAnimation<Vector, Rectangle> for Linear<T, DrawFunc, MaxFrames>
where
    DrawFunc: Fn(&mut T, usize, &mut Graphics, Rectangle) -> Result<()>,
    MaxFrames: Fn(&T) -> usize,
{
    fn draw(&mut self, gfx: &mut Graphics, location: Rectangle) -> Result<()> {
        let frames_passed = self.config.timing.exhaust().map(usize::from).unwrap_or(0);

        match frames_passed.checked_add(self.last_frame) {
            Some(x) => {
                self.last_frame = x % (self.config.max_frames)(&self.config.begin_state);
            }
            None => {
                let max_size = (self.config.max_frames)(&self.config.begin_state);
                let bound_to_frame = frames_passed % max_size;
                self.last_frame = (bound_to_frame + self.last_frame) % max_size;
            }
        }
        (self.config.draw)(&mut self.config.begin_state, self.last_frame, gfx, location)
    }
}

impl<T, DrawFunc, MaxFrames> Resetable for Linear<T, DrawFunc, MaxFrames>
where
    DrawFunc: Fn(&mut T, usize, &mut Graphics, Rectangle) -> Result<()>,
    MaxFrames: Fn(&T) -> usize,
{
    fn reset(&mut self) {
        self.config.timing.reset()
    }
}
