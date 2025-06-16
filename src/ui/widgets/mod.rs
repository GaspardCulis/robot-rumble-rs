use bevy::prelude::*;

#[derive(Bundle)]
pub struct UIButton {
    node: Node,
    text: Text,
    font: TextFont,
    color: TextColor,
    layout: TextLayout,
}

impl Default for UIButton {
    fn default() -> Self {
        Self {
            node: Node {
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            text: Text::new("Button"),
            font: TextFont {
                font_size: 32.0,
                ..default()
            },
            color: TextColor(Color::WHITE),
            layout: TextLayout::default(),
        }
    }
}

impl UIButton {
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Text::new(text);
        self
    }

    pub fn with_justify_text(mut self, justify: JustifyText) -> Self {
        self.layout.justify = justify;
        self
    }
}
