use gdnative::api::{CSGBox, CSGSphere, Node};
use gdnative::prelude::*;

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct FallingBall {
    ball: Option<Ref<CSGSphere>>,
    ground: Option<Ref<CSGBox>>,
}

#[methods]
impl FallingBall {
    fn new(_base: &Node) -> Self {
        FallingBall::default()
    }

    #[method]
    fn _ready(&mut self, #[base] _base: &Node) {
        self.ball = Some(
            unsafe { _base.get_node_as::<CSGSphere>("ball") }
                .unwrap()
                .claim(),
        );
        self.ground = Some(
            unsafe { _base.get_node_as::<CSGBox>("ground") }
                .unwrap()
                .claim(),
        );

        let ball = unsafe { self.ball.unwrap().assume_safe() };
        let ball_pos = ball.translation();
    }
}
