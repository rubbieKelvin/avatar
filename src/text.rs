use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, TextureCreator},
    surface::Surface,
    ttf::{Font, Sdl2TtfContext},
    video::{Window, WindowContext},
};

pub enum GlobalyLoadedFonts {
    Tarzeau16,
}

/// Managing text globally including fonts
pub struct GlobalTextManager<'a, 'b> {
    texture_creator: TextureCreator<WindowContext>,
    font0: Font<'a, 'b>,
}

impl<'a, 'b> GlobalTextManager<'a, 'b> {
    pub fn new(
        ctx: &'a Sdl2TtfContext,
        texture_creator: TextureCreator<WindowContext>,
    ) -> Result<Self, String> {
        let font0 = ctx.load_font("fonts/tarzeau_ocr_a.ttf", 16)?;

        return Ok(GlobalTextManager {
            font0,
            texture_creator,
        });
    }

    pub fn write<S: AsRef<str>>(
        &'a self,
        text: S,
        font: GlobalyLoadedFonts,
    ) -> TextBuilder<'a, 'b> {
        let text = text.as_ref().to_string();
        return TextBuilder::new(
            text,
            match font {
                GlobalyLoadedFonts::Tarzeau16 => &self.font0,
            },
            &self.texture_creator,
        );
    }
}

pub struct TextBuilder<'a, 'b> {
    text: String,
    _tc: &'a TextureCreator<WindowContext>,
    _font: &'a Font<'a, 'b>,
    _color: Color,
    _position: (i32, i32),
    _centered: bool,
}

impl<'a, 'b> TextBuilder<'a, 'b> {
    fn new(
        text: String,
        font: &'a Font<'a, 'b>,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> TextBuilder<'a, 'b> {
        return TextBuilder {
            text,
            _tc: texture_creator,
            _font: font,
            _position: (0, 0),
            _color: Color::WHITE,
            _centered: false,
        };
    }
    #[allow(unused)]
    pub fn color<C: Into<Color>>(mut self, color: C) -> TextBuilder<'a, 'b> {
        self._color = color.into();
        return self;
    }

    #[allow(unused)]
    pub fn position(mut self, x: i32, y: i32) -> TextBuilder<'a, 'b> {
        self._position = (x, y);
        return self;
    }

    #[allow(unused)]
    pub fn centered(mut self) -> TextBuilder<'a, 'b> {
        self._centered = true;
        return self;
    }

    pub fn surface<'s>(&self) -> Result<(Surface<'s>, Rect), String> {
        let text = &self.text;
        let surf = self
            ._font
            .render(text)
            .blended(self._color)
            .map_err(|e| e.to_string())?;
        let mut rect = surf.rect();

        if self._centered {
            // derefed Rect is SDL_Rect, so we gotta deref both
            *rect = *rect.centered_on(Point::new(self._position.0, self._position.1));
        } else {
            rect.set_x(self._position.0);
            rect.set_y(self._position.1);
        }

        return Ok((surf, rect));
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let (surf, rect) = self.surface()?;

        // load to gpu
        let texture = surf.as_texture(self._tc).map_err(|e| e.to_string())?;

        // copy the texture to the canvas
        canvas.copy(&texture, None, Some(rect))?;
        return Ok(());
    }
}
