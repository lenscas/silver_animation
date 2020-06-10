use crate::{AnimationShape, BasicAnimation, ContainedAnimation, EditableState, Resetable};
use quicksilver::geom::Vector;
use std::marker::PhantomData;

/// The struct used to turn a BasicAnimation into a ContainedAnimation
pub struct BasicAnimationContainer<Animation, Size, Combined>
where
    Animation: BasicAnimation<Size, Combined>,
    Combined: AnimationShape<Size>,
{
    pub(crate) animation: Animation,
    pub(crate) location: Combined,
    pub(crate) _size: PhantomData<Size>,
}

impl<Animation, Size, Combined> ContainedAnimation<Size, Combined>
    for BasicAnimationContainer<Animation, Size, Combined>
where
    Animation: BasicAnimation<Size, Combined>,
    Combined: AnimationShape<Size>,
{
    fn draw(&mut self, gfx: &mut quicksilver::graphics::Graphics) -> quicksilver::Result<()> {
        self.animation.draw(gfx, self.location.get_both())
    }
    fn set_location(&mut self, location: Vector) {
        self.location.set_location(location);
    }
    fn set_size(&mut self, size: Size) {
        self.location.set_size(size);
    }
    fn get_draw_pos(&self) -> Combined {
        self.location.get_both()
    }
    fn get_position(&self) -> Vector {
        self.location.get_location()
    }
    fn get_size(&self) -> Size {
        self.location.get_size()
    }
}

impl<Animation, Size, Combined> Resetable for BasicAnimationContainer<Animation, Size, Combined>
where
    Animation: BasicAnimation<Size, Combined> + Resetable,
    Combined: AnimationShape<Size>,
{
    fn reset(&mut self) {
        self.animation.reset()
    }
}

impl<Animation, Size, Combined> BasicAnimationContainer<Animation, Size, Combined>
where
    Animation: BasicAnimation<Size, Combined>,
    Combined: AnimationShape<Size>,
{
    ///Turns the ContainedAnimation back into the SimpleAnimation
    pub fn unpack(self) -> (Animation, Combined) {
        (self.animation, self.location)
    }
}

impl<Animation, T, Size, Combined> EditableState<T>
    for BasicAnimationContainer<Animation, Size, Combined>
where
    Animation: BasicAnimation<Size, Combined> + EditableState<T>,
    Combined: AnimationShape<Size>,
{
    fn set_state(&mut self, new_state: T) {
        self.animation.set_state(new_state)
    }
    fn get_state(&self) -> &T {
        self.animation.get_state()
    }
}
