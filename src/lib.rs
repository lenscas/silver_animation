mod contained;
mod linear;

use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::Graphics,
    Result, Timer,
};

pub use crate::{
    contained::BasicAnimationContainer,
    linear::{Linear, LinearConfig, SimpleLinearConfig},
};
use std::marker::PhantomData;

/// A bare bones trait that is simply used to draw an animation at the given position
pub trait BasicAnimation<Size, Combined: AnimationShape<Size>> {
    ///Draws the animation
    fn draw(&mut self, gfx: &mut Graphics, location: Combined) -> Result<()>;
    ///Turns the animation into a ContainedAnimation at the given position
    ///
    ///This can make it easier to draw if the location (almost) never changes.
    fn contain(self, location: Combined) -> BasicAnimationContainer<Self, Size, Combined>
    where
        Self: Sized,
    {
        BasicAnimationContainer {
            location,
            animation: self,
            _size: PhantomData,
        }
    }
}
/// This is trait used to define the location and the size of an animation
/// This way you can base your animations of Rectangles, Circles, etc
pub trait AnimationShape<Size> {
    fn get_size(&self) -> Size;
    fn get_location(&self) -> Vector;
    fn set_size(&mut self, size: Size);
    fn set_location(&mut self, location: Vector);
    fn get_both(&self) -> Self;
}

impl AnimationShape<Vector> for Rectangle {
    fn get_size(&self) -> Vector {
        self.size
    }
    fn get_location(&self) -> Vector {
        self.pos
    }
    fn set_size(&mut self, size: Vector) {
        self.size = size;
    }
    fn set_location(&mut self, location: Vector) {
        self.pos = location;
    }
    fn get_both(&self) -> Self {
        *self
    }
}

impl AnimationShape<f32> for Circle {
    fn get_size(&self) -> f32 {
        self.radius
    }
    fn get_location(&self) -> Vector {
        self.pos
    }
    fn set_size(&mut self, size: f32) {
        self.radius = size;
    }
    fn set_location(&mut self, location: Vector) {
        self.pos = location;
    }
    fn get_both(&self) -> Self {
        *self
    }
}

///The same as BasicAnimation, however ContainedAnimations are in control of their position
///
///This one can be used when the location is also part of the animation.
pub trait ContainedAnimation<Size, Combined: AnimationShape<Size>> {
    fn draw(&mut self, gfx: &mut Graphics) -> Result<()>;
    fn set_location(&mut self, location: Vector);
    fn set_size(&mut self, size: Size);
    fn get_draw_pos(&self) -> Combined;
    fn get_position(&self) -> Vector;
    fn get_size(&self) -> Size;
}

///This should be implemented if outside sources can change the state the animation is in.
pub trait EditableState<T> {
    fn set_state(&mut self, new_state: T);
    fn get_state(&self) -> &T;
}
///This trait can be implemented if an animation can be reset (Go back to the first frame)
pub trait Resetable {
    fn reset(&mut self);
}

///This struct decides what frame count the animation is currently at, it automatically resets to 0.
pub struct AnimationTimer<T, MaxFrames: Fn(&T) -> usize> {
    timer: Timer,
    at_frame: usize,
    max_frames: MaxFrames,
    _t: PhantomData<T>,
}
impl<T, MaxFrames> AnimationTimer<T, MaxFrames>
where
    MaxFrames: Fn(&T) -> usize,
{
    pub fn new(max_frames: MaxFrames, timer: Timer) -> AnimationTimer<T, MaxFrames> {
        Self {
            timer,
            max_frames,
            at_frame: 0,
            _t: PhantomData,
        }
    }
    pub fn get_current_frame(&mut self, state: &T) -> usize {
        let frames_passed = self.timer.exhaust().map(usize::from).unwrap_or(0);

        match frames_passed.checked_add(self.at_frame) {
            Some(x) => {
                self.at_frame = x % (self.max_frames)(state);
            }
            None => {
                let max_size = (self.max_frames)(state);
                let bound_to_frame = frames_passed % max_size;
                self.at_frame = (bound_to_frame + self.at_frame) % max_size;
            }
        }
        self.at_frame
    }
    pub fn reset(&mut self) {
        self.at_frame = 0;
        self.timer.reset();
    }
}
