//! Configure your application.
use baseview::{
    ParentWindowHandle,
    dpi::{LogicalSize, Size},
};
use raw_window_handle::HasWindowHandle;

/// Any settings specific to `iced_baseview`.
#[derive(Debug, Clone, PartialEq)]
pub struct IcedWindowSettings {
    /// The window title.
    pub title: String,

    /// The size of the window, either in physical or logical coordinates.
    pub size: Size,

    /// The amount of zoom (scaling) to apply. This is applied on top of the
    /// system's native scaling factor.
    pub scale_factor: f32,

    /// If the window is to be embedded in a parent window, the handle to that window.
    ///
    /// If `None`, the window will be standalone.
    pub parent: Option<ParentWindowHandle>,

    /// If the window expects to have a parent when first displayed.
    ///
    /// Setting this will delay the actual creation of the window until the parent is set (unless
    /// the window is shown first).
    ///
    /// If the `parent` field is already set, this does nothing and is ignored.
    pub wait_for_parent: bool,

    /// Ignore key inputs, except for modifier keys such as SHIFT and ALT
    pub ignore_non_modifier_keys: bool,

    /// Always redraw whenever the baseview window updates instead of only when iced wants to update
    /// the window. This works around a current baseview limitation where it does not support
    /// trigger a redraw on window visibility change (which may cause blank windows when opening or
    /// reopening the editor) and an iced limitation where it's not possible to have animations
    /// without using an asynchronous timer stream to send redraw messages to the application.
    pub always_redraw: bool,
}

impl IcedWindowSettings {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// The window title.
    #[inline]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// The size of the window, in logical coordinates.
    #[inline]
    pub fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// The amount of zoom (scaling) to apply. This is applied on top of the
    /// system's native scaling factor.
    #[inline]
    pub fn with_scale_factor(mut self, scale_factor: f32) -> Self {
        self.scale_factor = scale_factor;
        self
    }

    /// If the window is to be embedded in a parent window, the handle to that window.
    ///
    /// If `None`, the window will be standalone.
    #[inline]
    pub fn with_parent<'a, P: HasWindowHandle + 'a>(
        mut self,
        parent: impl Into<Option<&'a P>>,
    ) -> Self {
        self.parent = parent.into().map(ParentWindowHandle::from_window);
        self
    }

    /// Sets [`wait_for_parent`](Self::wait_for_parent) to the given value.
    pub fn with_wait_for_parent(mut self, wait_for_parent: bool) -> Self {
        self.wait_for_parent = wait_for_parent;
        self
    }

    /// Sets [`wait_for_parent`](Self::wait_for_parent) to `true`.
    #[inline]
    pub fn wait_for_parent(mut self) -> Self {
        self.wait_for_parent = true;
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

impl Default for IcedWindowSettings {
    fn default() -> Self {
        Self {
            title: String::new(),
            size: Size::Logical(LogicalSize {
                width: 300.0,
                height: 200.0,
            }),
            scale_factor: 1.0,
            parent: None,
            wait_for_parent: false,
            ignore_non_modifier_keys: false,
            always_redraw: false,
        }
    }
}
