// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use TlsCertificateRequestFlags;
use TlsConnection;
use TlsInteractionResult;
use TlsPassword;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct TlsInteraction(Object<ffi::GTlsInteraction, ffi::GTlsInteractionClass, TlsInteractionClass>);

    match fn {
        get_type => || ffi::g_tls_interaction_get_type(),
    }
}

pub const NONE_TLS_INTERACTION: Option<&TlsInteraction> = None;

pub trait TlsInteractionExt: 'static {
    fn ask_password<P: IsA<TlsPassword>, Q: IsA<Cancellable>>(&self, password: &P, cancellable: Option<&Q>) -> Result<TlsInteractionResult, Error>;

    fn ask_password_async<P: IsA<TlsPassword>, Q: IsA<Cancellable>, R: FnOnce(Result<TlsInteractionResult, Error>) + Send + 'static>(&self, password: &P, cancellable: Option<&Q>, callback: R);

    #[cfg(feature = "futures")]
    fn ask_password_async_future<P: IsA<TlsPassword> + Clone + 'static>(&self, password: &P) -> Box_<futures_core::Future<Item = (Self, TlsInteractionResult), Error = (Self, Error)>> where Self: Sized + Clone;

    fn invoke_ask_password<P: IsA<TlsPassword>, Q: IsA<Cancellable>>(&self, password: &P, cancellable: Option<&Q>) -> Result<TlsInteractionResult, Error>;

    fn invoke_request_certificate<P: IsA<TlsConnection>, Q: IsA<Cancellable>>(&self, connection: &P, flags: TlsCertificateRequestFlags, cancellable: Option<&Q>) -> Result<TlsInteractionResult, Error>;

    fn request_certificate<P: IsA<TlsConnection>, Q: IsA<Cancellable>>(&self, connection: &P, flags: TlsCertificateRequestFlags, cancellable: Option<&Q>) -> Result<TlsInteractionResult, Error>;

    fn request_certificate_async<P: IsA<TlsConnection>, Q: IsA<Cancellable>, R: FnOnce(Result<TlsInteractionResult, Error>) + Send + 'static>(&self, connection: &P, flags: TlsCertificateRequestFlags, cancellable: Option<&Q>, callback: R);

    #[cfg(feature = "futures")]
    fn request_certificate_async_future<P: IsA<TlsConnection> + Clone + 'static>(&self, connection: &P, flags: TlsCertificateRequestFlags) -> Box_<futures_core::Future<Item = (Self, TlsInteractionResult), Error = (Self, Error)>> where Self: Sized + Clone;
}

impl<O: IsA<TlsInteraction>> TlsInteractionExt for O {
    fn ask_password<P: IsA<TlsPassword>, Q: IsA<Cancellable>>(&self, password: &P, cancellable: Option<&Q>) -> Result<TlsInteractionResult, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_interaction_ask_password(self.as_ref().to_glib_none().0, password.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn ask_password_async<P: IsA<TlsPassword>, Q: IsA<Cancellable>, R: FnOnce(Result<TlsInteractionResult, Error>) + Send + 'static>(&self, password: &P, cancellable: Option<&Q>, callback: R) {
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn ask_password_async_trampoline<R: FnOnce(Result<TlsInteractionResult, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_interaction_ask_password_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = ask_password_async_trampoline::<R>;
        unsafe {
            ffi::g_tls_interaction_ask_password_async(self.as_ref().to_glib_none().0, password.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn ask_password_async_future<P: IsA<TlsPassword> + Clone + 'static>(&self, password: &P) -> Box_<futures_core::Future<Item = (Self, TlsInteractionResult), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let password = password.clone();
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.ask_password_async(
                &password,
                Some(&cancellable),
                move |res| {
                    let obj = obj_clone.into_inner();
                    let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn invoke_ask_password<P: IsA<TlsPassword>, Q: IsA<Cancellable>>(&self, password: &P, cancellable: Option<&Q>) -> Result<TlsInteractionResult, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_interaction_invoke_ask_password(self.as_ref().to_glib_none().0, password.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn invoke_request_certificate<P: IsA<TlsConnection>, Q: IsA<Cancellable>>(&self, connection: &P, flags: TlsCertificateRequestFlags, cancellable: Option<&Q>) -> Result<TlsInteractionResult, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_interaction_invoke_request_certificate(self.as_ref().to_glib_none().0, connection.as_ref().to_glib_none().0, flags.to_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn request_certificate<P: IsA<TlsConnection>, Q: IsA<Cancellable>>(&self, connection: &P, flags: TlsCertificateRequestFlags, cancellable: Option<&Q>) -> Result<TlsInteractionResult, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_interaction_request_certificate(self.as_ref().to_glib_none().0, connection.as_ref().to_glib_none().0, flags.to_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn request_certificate_async<P: IsA<TlsConnection>, Q: IsA<Cancellable>, R: FnOnce(Result<TlsInteractionResult, Error>) + Send + 'static>(&self, connection: &P, flags: TlsCertificateRequestFlags, cancellable: Option<&Q>, callback: R) {
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn request_certificate_async_trampoline<R: FnOnce(Result<TlsInteractionResult, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_interaction_request_certificate_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = request_certificate_async_trampoline::<R>;
        unsafe {
            ffi::g_tls_interaction_request_certificate_async(self.as_ref().to_glib_none().0, connection.as_ref().to_glib_none().0, flags.to_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn request_certificate_async_future<P: IsA<TlsConnection> + Clone + 'static>(&self, connection: &P, flags: TlsCertificateRequestFlags) -> Box_<futures_core::Future<Item = (Self, TlsInteractionResult), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let connection = connection.clone();
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.request_certificate_async(
                &connection,
                flags,
                Some(&cancellable),
                move |res| {
                    let obj = obj_clone.into_inner();
                    let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }
}

impl fmt::Display for TlsInteraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TlsInteraction")
    }
}
