mod contained;
mod linear;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Graphics,
    Result,
};

pub use crate::{
    contained::BasicAnimationContainer,
    linear::{Linear, LinearConfig, SimpleLinearConfig},
};

/// A bare bones trait that is simply used to draw an animation at the given position
pub trait BasicAnimation {
    ///Draws the animation
    fn draw(&mut self, gfx: &mut Graphics, location: Rectangle) -> Result<()>;
    ///Turns the animation into a ContainedAnimation at the given position
    ///
    ///This can make it easier to draw if the location (almost) never changes.
    fn contain(self, location: Rectangle) -> BasicAnimationContainer<Self>
    where
        Self: Sized,
    {
        BasicAnimationContainer {
            location,
            animation: self,
        }
    }
}
///The same as BasicAnimation, however ContainedAnimations are in control of their position
///
///This one can be used when the location is also part of the animation.
pub trait ContainedAnimation {
    fn draw(&mut self, gfx: &mut Graphics) -> Result<()>;
    fn set_location(&mut self, location: Vector);
    fn set_size(&mut self, size: Vector);
    fn get_draw_pos(&self) -> Rectangle;
    fn get_position(&self) -> Vector;
    fn get_size(&self) -> Vector;
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
