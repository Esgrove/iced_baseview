use baseview::WindowSize;
use baseview::dpi::LogicalPosition;
use iced_core::keyboard::Modifiers;
use iced_core::theme;
use iced_program::Program;
use std::marker::PhantomData;

use crate::core::mouse;
use crate::core::{Color, Size};
use crate::graphics::Viewport;
use crate::program;
use crate::shell::window::WindowQueue;
use crate::window;

/// The state of a windowed [`Application`].
pub struct State<P: Program> {
    _title: String,
    viewport: Viewport,
    surface_version: u64,
    cursor_position: Option<iced_runtime::core::Point>,
    theme: Option<P::Theme>,
    theme_mode: theme::Mode,
    default_theme: P::Theme,
    style: theme::Style,
    pub zoom_factor: f32,
    pub system_scale_factor: f64,
    pub modifiers: Modifiers,
    window_queue: WindowQueue,
    program: PhantomData<P>,
}

impl<P: Program> State<P> {
    /// Creates a new [`State`] for the provided [`Program`] and window.
    pub fn new(
        program: &program::Instance<P>,
        window_id: window::Id,
        window_physical_size: Size<u32>,
        system_scale_factor: f64,
        system_theme: theme::Mode,
        window_queue: WindowQueue,
    ) -> Self {
        let title = program.title(window_id);
        let zoom_factor = program.scale_factor(window_id);
        let theme = program.theme(window_id);
        let theme_mode = theme.as_ref().map(theme::Base::mode).unwrap_or_default();
        let default_theme = <P::Theme as theme::Base>::default(system_theme);
        let style = program.style(theme.as_ref().unwrap_or(&default_theme));

        let viewport = Viewport::with_physical_size(
            window_physical_size,
            system_scale_factor as f32 * zoom_factor,
        );

        Self {
            _title: title,
            viewport,
            surface_version: 0,
            cursor_position: None,
            theme,
            theme_mode,
            default_theme,
            style,
            zoom_factor,
            system_scale_factor,
            modifiers: Default::default(),
            window_queue,
            program: PhantomData,
        }
    }

    /// Returns the current [`Viewport`] of the [`State`].
    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn surface_version(&self) -> u64 {
        self.surface_version
    }

    /// Returns the physical [`Size`] of the [`Viewport`] of the [`State`].
    pub fn physical_size(&self) -> Size<u32> {
        self.viewport.physical_size()
    }

    /// Returns the logical [`Size`] of the [`Viewport`] of the [`State`].
    pub fn logical_size(&self) -> Size<f32> {
        self.viewport.logical_size()
    }

    pub fn system_scale_factor(&self) -> f32 {
        self.system_scale_factor as f32
    }

    /// Returns the current cursor position of the [`State`].
    pub fn cursor(&self) -> mouse::Cursor {
        self.cursor_position
            .map(mouse::Cursor::Available)
            .unwrap_or(mouse::Cursor::Unavailable)
    }

    /// Returns the current theme of the [`State`].
    pub fn theme(&self) -> &P::Theme {
        self.theme.as_ref().unwrap_or(&self.default_theme)
    }

    /// Returns the current background [`Color`] of the [`State`].
    pub fn background_color(&self) -> Color {
        self.style.background_color
    }

    /// Returns the current text [`Color`] of the [`State`].
    pub fn text_color(&self) -> Color {
        self.style.text_color
    }

    pub fn update_cursor_position(&mut self, event: &baseview::Event) {
        match event {
            baseview::Event::Mouse(baseview::MouseEvent::CursorMoved {
                position,
                modifiers: _,
            }) => {
                let logical_pos: LogicalPosition<f32> =
                    position.to_logical(self.system_scale_factor * self.zoom_factor as f64);

                self.cursor_position = Some(crate::core::Point {
                    x: logical_pos.x,
                    y: logical_pos.y,
                });
            }
            baseview::Event::Mouse(baseview::MouseEvent::CursorLeft) => {
                self.cursor_position = None;
            }
            _ => {}
        }
    }

    pub fn resized(&mut self, new_size: WindowSize) {
        self.system_scale_factor = new_size.scale_factor;

        self.viewport = Viewport::with_physical_size(
            Size::new(new_size.physical.width, new_size.physical.height),
            self.system_scale_factor as f32 * self.zoom_factor,
        );

        self.surface_version += 1;
    }

    /// Synchronizes the [`State`] with its [`Program`] and its respective window.
    pub fn synchronize(&mut self, program: &program::Instance<P>, window_id: window::Id) {
        // Update window title
        let new_title = program.title(window_id);
        if self._title != new_title {
            // TODO
            self._title = new_title;
        }

        // Update scale factor
        let new_zoom_factor = program.scale_factor(window_id);
        if self.zoom_factor != new_zoom_factor {
            self.zoom_factor = new_zoom_factor;

            self.window_queue.send(super::WindowCommand::ResizeWindow {
                size: self.viewport.logical_size(),
                zoom_factor: new_zoom_factor,
            });
        }

        // Update theme and appearance
        self.theme = program.theme(window_id);
        self.style = program.style(self.theme());

        let new_mode = self
            .theme
            .as_ref()
            .map(theme::Base::mode)
            .unwrap_or_default();

        if self.theme_mode != new_mode {
            self.theme_mode = if new_mode == theme::Mode::None {
                theme::Base::mode(&self.default_theme)
            } else {
                new_mode
            };
        }
    }

    pub fn set_system_theme(
        &mut self,
        window_id: window::Id,
        system_theme: theme::Mode,
        program: &program::Instance<P>,
    ) {
        let theme = program.theme(window_id);
        self.theme_mode = theme.as_ref().map(theme::Base::mode).unwrap_or_default();
        self.default_theme = <P::Theme as theme::Base>::default(system_theme);
        self.style = program.style(theme.as_ref().unwrap_or(&self.default_theme));
    }
}
