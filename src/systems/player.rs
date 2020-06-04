use amethyst::{
    derive::SystemDesc,
    ecs::{Entity, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::{PhysicsBodyDescription, SimpleAnimation, StateAnimation};

///This system controls the character control
#[derive(SystemDesc)]
pub struct PlayerSystem {
    player: Entity,
}

impl PlayerSystem {
    pub(crate) fn new(player: Entity) -> Self {
        PlayerSystem {
            player,
        }
    }
}

#[allow(dead_code)]
impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, PhysicsBodyDescription>,
        WriteStorage<'s, SimpleAnimation>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut descs, mut animations, input): Self::SystemData) {
        let player_desc = descs.get_mut(self.player).unwrap();
        let player_anim = animations.get_mut(self.player).unwrap();

        if let Some(x) = input.axis_value("x-axis") {
            if x == 0. {
                player_desc.set_velocity_direction_x(0.);
                player_anim.change_state(StateAnimation::Idle);
            } else {
                player_desc.set_velocity_direction_x(x);
                player_anim.change_state(StateAnimation::Run);
            }
        }

        if let Some(jump) = input.action_is_down("Jump"){
            if jump {
                player_desc.set_velocity_direction_y(1.);
            } else {
                player_desc.set_velocity_direction_y(0.);
            }
        }
    }
}