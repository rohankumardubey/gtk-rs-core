// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::SocketConnectable;
use crate::TlsCertificateFlags;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GTlsCertificate")]
    pub struct TlsCertificate(Object<ffi::GTlsCertificate, ffi::GTlsCertificateClass>);

    match fn {
        type_ => || ffi::g_tls_certificate_get_type(),
    }
}

impl TlsCertificate {
    #[doc(alias = "g_tls_certificate_new_from_file")]
    #[doc(alias = "new_from_file")]
    pub fn from_file(file: impl AsRef<std::path::Path>) -> Result<TlsCertificate, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_tls_certificate_new_from_file(file.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_tls_certificate_new_from_files")]
    #[doc(alias = "new_from_files")]
    pub fn from_files(
        cert_file: impl AsRef<std::path::Path>,
        key_file: impl AsRef<std::path::Path>,
    ) -> Result<TlsCertificate, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_new_from_files(
                cert_file.as_ref().to_glib_none().0,
                key_file.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_tls_certificate_new_from_pem")]
    #[doc(alias = "new_from_pem")]
    pub fn from_pem(data: &str) -> Result<TlsCertificate, glib::Error> {
        let length = data.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_tls_certificate_new_from_pem(data.to_glib_none().0, length, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_tls_certificate_list_new_from_file")]
    pub fn list_new_from_file(
        file: impl AsRef<std::path::Path>,
    ) -> Result<Vec<TlsCertificate>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_list_new_from_file(
                file.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub const NONE_TLS_CERTIFICATE: Option<&TlsCertificate> = None;

pub trait TlsCertificateExt: 'static {
    #[doc(alias = "g_tls_certificate_get_issuer")]
    #[doc(alias = "get_issuer")]
    fn issuer(&self) -> Option<TlsCertificate>;

    #[doc(alias = "g_tls_certificate_is_same")]
    fn is_same(&self, cert_two: &impl IsA<TlsCertificate>) -> bool;

    #[doc(alias = "g_tls_certificate_verify")]
    fn verify(
        &self,
        identity: Option<&impl IsA<SocketConnectable>>,
        trusted_ca: Option<&impl IsA<TlsCertificate>>,
    ) -> TlsCertificateFlags;

    fn certificate(&self) -> Option<glib::ByteArray>;

    #[doc(alias = "certificate-pem")]
    fn certificate_pem(&self) -> Option<glib::GString>;
}

impl<O: IsA<TlsCertificate>> TlsCertificateExt for O {
    fn issuer(&self) -> Option<TlsCertificate> {
        unsafe {
            from_glib_none(ffi::g_tls_certificate_get_issuer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_same(&self, cert_two: &impl IsA<TlsCertificate>) -> bool {
        unsafe {
            from_glib(ffi::g_tls_certificate_is_same(
                self.as_ref().to_glib_none().0,
                cert_two.as_ref().to_glib_none().0,
            ))
        }
    }

    fn verify(
        &self,
        identity: Option<&impl IsA<SocketConnectable>>,
        trusted_ca: Option<&impl IsA<TlsCertificate>>,
    ) -> TlsCertificateFlags {
        unsafe {
            from_glib(ffi::g_tls_certificate_verify(
                self.as_ref().to_glib_none().0,
                identity.map(|p| p.as_ref()).to_glib_none().0,
                trusted_ca.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    fn certificate(&self) -> Option<glib::ByteArray> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::ByteArray as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"certificate\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `certificate` getter")
        }
    }

    fn certificate_pem(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"certificate-pem\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `certificate-pem` getter")
        }
    }
}

impl fmt::Display for TlsCertificate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TlsCertificate")
    }
}
