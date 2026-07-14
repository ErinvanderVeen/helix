use crate::compositor::{Component, Context};
use helix_view::graphics::{Margin, Rect, Style};
use tui::{
    buffer::Buffer as Surface,
    text::{Span, Spans, Text},
};

pub struct TextOutput {
    contents: String,
}

impl TextOutput {
    pub fn new(contents: String) -> Self {
        Self { contents }
    }

    fn text(&self, style: Style) -> Text<'static> {
        let lines: Vec<Spans> = self
            .contents
            .lines()
            .map(|line| Spans::from(Span::styled(line.replace('\t', "    "), style)))
            .collect();
        Text::from(lines)
    }
}

impl Component for TextOutput {
    fn render(&mut self, area: Rect, surface: &mut Surface, cx: &mut Context) {
        use tui::widgets::{Paragraph, Widget, Wrap};

        let style = cx.editor.theme.get("ui.text");
        let text = self.text(style);

        let par = Paragraph::new(&text)
            .wrap(Wrap { trim: false })
            .scroll((cx.scroll.unwrap_or_default() as u16, 0));

        let margin = Margin::all(1);
        par.render(area.inner(margin), surface);
    }

    fn required_size(&mut self, viewport: (u16, u16)) -> Option<(u16, u16)> {
        let padding = 2;

        let contents = self.text(Style::default());

        let max_text_width = (viewport.0.saturating_sub(padding)).min(120);
        let (width, height) = crate::ui::text::required_size(&contents, max_text_width);

        Some((width + padding, height + padding))
    }
}
