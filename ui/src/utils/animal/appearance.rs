use nannou::App;
use nannou::wgpu::Texture;

#[derive(Clone)]
pub struct Appearance {
    icon: Texture,
}

impl Appearance {
    pub fn new(app: &App, name: &str) -> Self {
        let icon = Texture::from_path(app, format!("assets/images/animals/{}.png", name)).unwrap();
        Self { icon }
    }

    pub fn get_icon(&self) -> &Texture {
        &self.icon
    }
}
