use super::Texture;

pub struct Material<'a> {
    diffuse: &'a Texture,
}

impl<'a> Material<'a> {
    pub fn new(diffuse: &'a Texture) -> Self {
        Self { diffuse }
    }

    pub fn diffuse(&self) -> &Texture {
        self.diffuse
    }
}
