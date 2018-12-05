//! This crate provides safe bindings to
//! [wlroots](https://github.com/swaywm/wlroots).
//!
//! Start your [Compositor](struct.Compositor.html) by implementing an [input
//! manager](manager/struct.InputManager.html) and an [output
//! manager](manager/struct.OutputManager.html) on two separate structs.
#![cfg_attr(not(feature = "unstable"), allow(unused_imports, unused_macros))]

#![allow(unused_unsafe)]
#[macro_use]
extern crate bitflags;
extern crate lazy_static;
pub extern crate libc;
extern crate vsprintf;
#[macro_use]
pub extern crate wlroots_sys;
pub extern crate wlroots_dehandle;
pub extern crate xkbcommon;

pub use wlroots_dehandle::wlroots_dehandle;
pub(crate) use wlroots_sys::wayland_sys;

#[macro_use]
mod macros;
#[cfg(feature = "unstable")]
mod manager;
#[cfg(feature = "unstable")]
mod compositor;
#[cfg(feature = "unstable")]
mod errors;
#[cfg(feature = "unstable")]
pub mod events;
mod types;
#[cfg(feature = "unstable")]
pub mod extensions;
#[cfg(feature = "unstable")]
pub mod render;
pub mod utils;
#[cfg(feature = "unstable")]
mod xwayland;
#[cfg(feature = "unstable")]
mod backend;

pub use types::*;

#[cfg(feature = "unstable")]
pub use self::backend::*;
#[cfg(feature = "unstable")]
pub use self::compositor::{compositor_handle, terminate, Compositor, CompositorBuilder,
                            CompositorHandle, CompositorHandler};
#[cfg(feature = "unstable")]
pub use self::events::{key_events, seat_events, tablet_pad_events, tablet_tool_events,
                        touch_events, xwayland_events,
                        pointer_events::{self, BTN_BACK, BTN_EXTRA, BTN_FORWARD, BTN_LEFT,
                                        BTN_MIDDLE, BTN_MOUSE, BTN_RIGHT, BTN_SIDE, BTN_TASK},
                        xdg_shell_v6_events, xdg_shell_events};
#[cfg(feature = "unstable")]
pub use self::manager::{InputManagerHandler, KeyboardHandler, OutputBuilder, OutputBuilderResult,
                        OutputHandler, OutputManagerHandler, PointerHandler, TabletPadHandler,
                        TabletToolHandler, TouchHandler, XdgV6ShellHandler,
                        XdgV6ShellManagerHandler, XdgShellHandler, XdgShellManagerHandler,
                        DragIconHandler};

#[cfg(feature = "unstable")]
pub use self::xwayland::{XWaylandManagerHandler, XWaylandServer, XWaylandSurface,
                         XWaylandSurfaceHandle, XWaylandSurfaceHandler, XWaylandSurfaceHints,
                         XWaylandSurfaceSizeHints};
#[cfg(feature = "unstable")]
pub use key_events::Key;
#[cfg(feature = "unstable")]
pub use wlroots_sys::{wlr_keyboard_modifiers, wlr_tablet_tool_axes, wl_shm_format::{self, *},
                      wlr_axis_orientation::{self, *}, wlr_axis_source::{self, *},
                      wlr_button_state::{self, *}, wlr_input_device_type::{self, *},
                      wlr_key_state::{self, *}, wlr_keyboard_modifier::{self, *},
                      wlr_tablet_pad_ring_source::{self, *},
                      wlr_tablet_pad_strip_source::{self, *},
                      wlr_tablet_tool_proximity_state::{self, *}};

#[cfg(feature = "unstable")]
pub use self::render::{matrix, GenericRenderer, Image, Renderer, Texture, TextureFormat};

#[cfg(feature = "unstable")]
pub use self::errors::*;

