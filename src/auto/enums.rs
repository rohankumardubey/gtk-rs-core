// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::Quark;
use glib::StaticType;
use glib::Type;
use glib::error::ErrorDomain;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_ffi;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Colorspace {
    Rgb,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for Colorspace {
    type GlibType = ffi::GdkColorspace;

    fn to_glib(&self) -> ffi::GdkColorspace {
        match *self {
            Colorspace::Rgb => ffi::GDK_COLORSPACE_RGB,
            Colorspace::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkColorspace> for Colorspace {
    fn from_glib(value: ffi::GdkColorspace) -> Self {
        match value {
            0 => Colorspace::Rgb,
            value => Colorspace::__Unknown(value),
        }
    }
}

impl StaticType for Colorspace {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_colorspace_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for Colorspace {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for Colorspace {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for Colorspace {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum InterpType {
    Nearest,
    Tiles,
    Bilinear,
    Hyper,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for InterpType {
    type GlibType = ffi::GdkInterpType;

    fn to_glib(&self) -> ffi::GdkInterpType {
        match *self {
            InterpType::Nearest => ffi::GDK_INTERP_NEAREST,
            InterpType::Tiles => ffi::GDK_INTERP_TILES,
            InterpType::Bilinear => ffi::GDK_INTERP_BILINEAR,
            InterpType::Hyper => ffi::GDK_INTERP_HYPER,
            InterpType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkInterpType> for InterpType {
    fn from_glib(value: ffi::GdkInterpType) -> Self {
        match value {
            0 => InterpType::Nearest,
            1 => InterpType::Tiles,
            2 => InterpType::Bilinear,
            3 => InterpType::Hyper,
            value => InterpType::__Unknown(value),
        }
    }
}

impl StaticType for InterpType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_interp_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for InterpType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for InterpType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for InterpType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PixbufAlphaMode {
    Bilevel,
    Full,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for PixbufAlphaMode {
    type GlibType = ffi::GdkPixbufAlphaMode;

    fn to_glib(&self) -> ffi::GdkPixbufAlphaMode {
        match *self {
            PixbufAlphaMode::Bilevel => ffi::GDK_PIXBUF_ALPHA_BILEVEL,
            PixbufAlphaMode::Full => ffi::GDK_PIXBUF_ALPHA_FULL,
            PixbufAlphaMode::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkPixbufAlphaMode> for PixbufAlphaMode {
    fn from_glib(value: ffi::GdkPixbufAlphaMode) -> Self {
        match value {
            0 => PixbufAlphaMode::Bilevel,
            1 => PixbufAlphaMode::Full,
            value => PixbufAlphaMode::__Unknown(value),
        }
    }
}

impl StaticType for PixbufAlphaMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_pixbuf_alpha_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PixbufAlphaMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PixbufAlphaMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PixbufAlphaMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PixbufError {
    CorruptImage,
    InsufficientMemory,
    BadOption,
    UnknownType,
    UnsupportedOperation,
    Failed,
    IncompleteAnimation,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for PixbufError {
    type GlibType = ffi::GdkPixbufError;

    fn to_glib(&self) -> ffi::GdkPixbufError {
        match *self {
            PixbufError::CorruptImage => ffi::GDK_PIXBUF_ERROR_CORRUPT_IMAGE,
            PixbufError::InsufficientMemory => ffi::GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY,
            PixbufError::BadOption => ffi::GDK_PIXBUF_ERROR_BAD_OPTION,
            PixbufError::UnknownType => ffi::GDK_PIXBUF_ERROR_UNKNOWN_TYPE,
            PixbufError::UnsupportedOperation => ffi::GDK_PIXBUF_ERROR_UNSUPPORTED_OPERATION,
            PixbufError::Failed => ffi::GDK_PIXBUF_ERROR_FAILED,
            PixbufError::IncompleteAnimation => ffi::GDK_PIXBUF_ERROR_INCOMPLETE_ANIMATION,
            PixbufError::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkPixbufError> for PixbufError {
    fn from_glib(value: ffi::GdkPixbufError) -> Self {
        match value {
            0 => PixbufError::CorruptImage,
            1 => PixbufError::InsufficientMemory,
            2 => PixbufError::BadOption,
            3 => PixbufError::UnknownType,
            4 => PixbufError::UnsupportedOperation,
            5 => PixbufError::Failed,
            6 => PixbufError::IncompleteAnimation,
            value => PixbufError::__Unknown(value),
        }
    }
}

impl ErrorDomain for PixbufError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::gdk_pixbuf_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(PixbufError::CorruptImage),
            1 => Some(PixbufError::InsufficientMemory),
            2 => Some(PixbufError::BadOption),
            3 => Some(PixbufError::UnknownType),
            4 => Some(PixbufError::UnsupportedOperation),
            5 => Some(PixbufError::Failed),
            6 => Some(PixbufError::IncompleteAnimation),
            _ => Some(PixbufError::Failed),
        }
    }
}

impl StaticType for PixbufError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_pixbuf_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PixbufError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PixbufError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PixbufError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PixbufRotation {
    None,
    Counterclockwise,
    Upsidedown,
    Clockwise,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for PixbufRotation {
    type GlibType = ffi::GdkPixbufRotation;

    fn to_glib(&self) -> ffi::GdkPixbufRotation {
        match *self {
            PixbufRotation::None => ffi::GDK_PIXBUF_ROTATE_NONE,
            PixbufRotation::Counterclockwise => ffi::GDK_PIXBUF_ROTATE_COUNTERCLOCKWISE,
            PixbufRotation::Upsidedown => ffi::GDK_PIXBUF_ROTATE_UPSIDEDOWN,
            PixbufRotation::Clockwise => ffi::GDK_PIXBUF_ROTATE_CLOCKWISE,
            PixbufRotation::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkPixbufRotation> for PixbufRotation {
    fn from_glib(value: ffi::GdkPixbufRotation) -> Self {
        match value {
            0 => PixbufRotation::None,
            90 => PixbufRotation::Counterclockwise,
            180 => PixbufRotation::Upsidedown,
            270 => PixbufRotation::Clockwise,
            value => PixbufRotation::__Unknown(value),
        }
    }
}

impl StaticType for PixbufRotation {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_pixbuf_rotation_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PixbufRotation {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PixbufRotation {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PixbufRotation {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

