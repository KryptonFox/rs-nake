#[derive(Debug, Default)]
pub enum Scene {
    #[default]
    Menu,
    Game,
    GameOver,
}

pub struct GameState {
    pub scene: Scene,
    pub gameover: bool,
    pub apples: u32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            scene: Scene::Menu,
            gameover: false,
            apples: 0,
        }
    }

    pub fn set_scene(&mut self, scene: Scene) {
        self.scene = scene;
    }
}
