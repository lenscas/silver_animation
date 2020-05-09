use crate::{ContainedAnimation, Resetable, SimpleAnimation};
use quicksilver::geom::{Rectangle, Vector};

///A simple wrapper over a SimpleAnimation and Rectangle
///
///Using this wrapper can make drawing easier if the location isn't going to change often or at all
pub struct BasicAnimationContainer<Animation>
where
    Animation: SimpleAnimation,
{
    pub(crate) animation: Animation,
    pub(crate) location: Rectangle,
}

impl<Animation> ContainedAnimation for BasicAnimationContainer<Animation>
where
    Animation: SimpleAnimation,
{
    fn draw(&mut self, gfx: &mut quicksilver::graphics::Graphics) -> quicksilver::Result<()> {
        self.animation.draw(gfx, self.location)
    }
    fn set_location(&mut self, location: quicksilver::geom::Vector) {
        self.location.pos = location;
    }
    fn set_size(&mut self, size: quicksilver::geom::Vector) {
        self.location.size = size;
    }
    fn get_draw_pos(&self) -> Rectangle {
        self.location
    }
    fn get_position(&self) -> Vector {
        self.location.pos
    }
    fn get_size(&self) -> Vector {
        self.location.size
    }
}

impl<Animation> Resetable for BasicAnimationContainer<Animation>
where
    Animation: SimpleAnimation + Resetable,
{
    fn reset(&mut self) {
        self.animation.reset()
    }
}

impl<Animation> BasicAnimationContainer<Animation>
where
    Animation: SimpleAnimation,
{
    ///Turns the ContainedAnimation back into the SimpleAnimation
    pub fn unpack(self) -> (Animation, Rectangle) {
        (self.animation, self.location)
    }
}
