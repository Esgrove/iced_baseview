//! Configure your application.
use baseview::{WindowOpenOptions, WindowScalePolicy, dpi::Size};

/// Any settings specific to `iced_baseview`.
#[derive(Debug, Clone, PartialEq)]
pub struct IcedBaseviewSettings {
    /// The window settings.
    pub window: WindowOpenOptions,

    /// Ignore key inputs, except for modifier keys such as SHIFT and ALT
    pub ignore_non_modifier_keys: bool,

    /// Always redraw whenever the baseview window updates instead of only when iced wants to update
    /// the window. This works around a current baseview limitation where it does not support
    /// trigger a redraw on window visibility change (which may cause blank windows when opening or
    /// reopening the editor) and an iced limitation where it's not possible to have animations
    /// without using an asynchronous timer stream to send redraw messages to the application.
    pub always_redraw: bool,
}

impl IcedBaseviewSettings {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_window_options(mut self, opts: WindowOpenOptions) -> Self {
        self.window = opts;
        self
    }

    #[inline]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.window = self.window.with_title(title);
        self
    }

    #[inline]
    pub fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.window = self.window.with_size(size);
        self
    }

    #[inline]
    pub fn with_scale_policy(mut self, scale: WindowScalePolicy) -> Self {
        self.window = self.window.with_scale_policy(scale);
        self
    }

    /// Ignore key inputs, except for modifier keys such as SHIFT and ALT.
    ///
    /// This may help with misbehaving DAWs in some cases.
    #[inline]
    pub fn with_ignore_non_modifier_keys(mut self, ignore: bool) -> Self {
        self.ignore_non_modifier_keys = ignore;
        self
    }

    /// Always redraw whenever the baseview window updates instead of only when iced wants to update
    /// the window. This works around a current baseview limitation where it does not support
    /// trigger a redraw on window visibility change (which may cause blank windows when opening or
    /// reopening the editor) and an iced limitation where it's not possible to have animations
    /// without using an asynchronous timer stream to send redraw messages to the application.
    #[inline]
    pub fn with_always_redraw(mut self, always_redraw: bool) -> Self {
        self.always_redraw = always_redraw;
        self
    }
}

impl Default for IcedBaseviewSettings {
    fn default() -> Self {
        Self {
            window: WindowOpenOptions::default(),
            ignore_non_modifier_keys: false,
            always_redraw: false,
        }
    }
}
