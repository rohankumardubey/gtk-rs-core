// This file was generated by gir (b010d34) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct StyleProvider(Object<ffi::GtkStyleProvider>);

    match fn {
        get_type => || ffi::gtk_style_provider_get_type(),
    }
}

pub trait StyleProviderExt {
    //fn get_icon_factory(&self, path: /*Ignored*/&mut WidgetPath) -> Option<IconFactory>;

    //fn get_style(&self, path: /*Ignored*/&mut WidgetPath) -> Option<StyleProperties>;

    //fn get_style_property<P: IsA</*Ignored*/glib::ParamSpec>>(&self, path: /*Ignored*/&mut WidgetPath, state: StateFlags, pspec: &P) -> Option<glib::Value>;
}

impl<O: IsA<StyleProvider>> StyleProviderExt for O {
    //fn get_icon_factory(&self, path: /*Ignored*/&mut WidgetPath) -> Option<IconFactory> {
    //    unsafe { TODO: call ffi::gtk_style_provider_get_icon_factory() }
    //}

    //fn get_style(&self, path: /*Ignored*/&mut WidgetPath) -> Option<StyleProperties> {
    //    unsafe { TODO: call ffi::gtk_style_provider_get_style() }
    //}

    //fn get_style_property<P: IsA</*Ignored*/glib::ParamSpec>>(&self, path: /*Ignored*/&mut WidgetPath, state: StateFlags, pspec: &P) -> Option<glib::Value> {
    //    unsafe { TODO: call ffi::gtk_style_provider_get_style_property() }
    //}
}
