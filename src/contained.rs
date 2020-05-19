use crate::{BasicAnimation, ContainedAnimation, EditableState, Resetable};
use quicksilver::geom::{Rectangle, Vector};

/// The struct used to turn a BasicAnimation into a ContainedAnimation
pub struct BasicAnimationContainer<Animation>
where
    Animation: BasicAnimation,
{
    pub(crate) animation: Animation,
    pub(crate) location: Rectangle,
}

impl<Animation> ContainedAnimation for BasicAnimationContainer<Animation>
where
    Animation: BasicAnimation,
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
    Animation: BasicAnimation + Resetable,
{
    fn reset(&mut self) {
        self.animation.reset()
    }
}

impl<Animation> BasicAnimationContainer<Animation>
where
    Animation: BasicAnimation,
{
    ///Turns the ContainedAnimation back into the SimpleAnimation
    pub fn unpack(self) -> (Animation, Rectangle) {
        (self.animation, self.location)
    }
}

impl<Animation, T> EditableState<T> for BasicAnimationContainer<Animation>
where
    Animation: BasicAnimation + EditableState<T>,
{
    fn set_state(&mut self, new_state: T) {
        self.animation.set_state(new_state)
    }
    fn get_state(&self) -> &T {
        self.animation.get_state()
    }
}
