use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entity, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
///This system controls the camera and ties it to a character at all times
pub struct CameraSystem {
    pub(crate) character: Entity,
    pub(crate) camera: Entity,
}

impl<'s> System<'s> for CameraSystem {
    type SystemData = WriteStorage<'s, Transform>;

    fn run(&mut self, mut transforms: Self::SystemData) {
        let char = transforms.get(self.character).unwrap().translation().clone();
        let cam = transforms.get_mut(self.camera).unwrap();

        //TODO
        cam.translation_mut().x = char.x + 250.;
        cam.translation_mut().y = char.y + 80.;
    }
}