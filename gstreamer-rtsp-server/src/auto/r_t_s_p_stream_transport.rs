// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use RTSPStream;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_rtsp;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct RTSPStreamTransport(Object<ffi::GstRTSPStreamTransport, ffi::GstRTSPStreamTransportClass>);

    match fn {
        get_type => || ffi::gst_rtsp_stream_transport_get_type(),
    }
}

impl RTSPStreamTransport {
    //pub fn new(stream: &RTSPStream, tr: /*Ignored*/&mut gst_rtsp::RTSPTransport) -> RTSPStreamTransport {
    //    unsafe { TODO: call ffi::gst_rtsp_stream_transport_new() }
    //}
}

pub trait RTSPStreamTransportExt {
    fn get_rtpinfo(&self, start_time: gst::ClockTime) -> Option<String>;

    fn get_stream(&self) -> Option<RTSPStream>;

    //fn get_transport(&self) -> /*Ignored*/Option<gst_rtsp::RTSPTransport>;

    fn get_url(&self) -> Option<gst_rtsp::RTSPUrl>;

    fn is_timed_out(&self) -> bool;

    fn keep_alive(&self);

    fn recv_data(&self, channel: u32, buffer: &gst::Buffer) -> gst::FlowReturn;

    fn send_rtcp(&self, buffer: &gst::Buffer) -> Result<(), glib::error::BoolError>;

    fn send_rtp(&self, buffer: &gst::Buffer) -> Result<(), glib::error::BoolError>;

    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError>;

    //fn set_callbacks<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, send_rtp: /*Unknown conversion*//*Unimplemented*/RTSPSendFunc, send_rtcp: /*Unknown conversion*//*Unimplemented*/RTSPSendFunc, user_data: P, notify: Q);

    //fn set_keepalive<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, keep_alive: /*Unknown conversion*//*Unimplemented*/RTSPKeepAliveFunc, user_data: P, notify: Q);

    fn set_timed_out(&self, timedout: bool);

    //fn set_transport(&self, tr: /*Ignored*/&mut gst_rtsp::RTSPTransport);

    fn set_url<'a, P: Into<Option<&'a gst_rtsp::RTSPUrl>>>(&self, url: P);
}

impl<O: IsA<RTSPStreamTransport>> RTSPStreamTransportExt for O {
    fn get_rtpinfo(&self, start_time: gst::ClockTime) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_transport_get_rtpinfo(self.to_glib_none().0, start_time.to_glib()))
        }
    }

    fn get_stream(&self) -> Option<RTSPStream> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_stream_transport_get_stream(self.to_glib_none().0))
        }
    }

    //fn get_transport(&self) -> /*Ignored*/Option<gst_rtsp::RTSPTransport> {
    //    unsafe { TODO: call ffi::gst_rtsp_stream_transport_get_transport() }
    //}

    fn get_url(&self) -> Option<gst_rtsp::RTSPUrl> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_stream_transport_get_url(self.to_glib_none().0))
        }
    }

    fn is_timed_out(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_transport_is_timed_out(self.to_glib_none().0))
        }
    }

    fn keep_alive(&self) {
        unsafe {
            ffi::gst_rtsp_stream_transport_keep_alive(self.to_glib_none().0);
        }
    }

    fn recv_data(&self, channel: u32, buffer: &gst::Buffer) -> gst::FlowReturn {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_transport_recv_data(self.to_glib_none().0, channel, buffer.to_glib_full()))
        }
    }

    fn send_rtcp(&self, buffer: &gst::Buffer) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_rtsp_stream_transport_send_rtcp(self.to_glib_none().0, buffer.to_glib_none().0), "Failed to send rtcp")
        }
    }

    fn send_rtp(&self, buffer: &gst::Buffer) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_rtsp_stream_transport_send_rtp(self.to_glib_none().0, buffer.to_glib_none().0), "Failed to send rtp")
        }
    }

    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_rtsp_stream_transport_set_active(self.to_glib_none().0, active.to_glib()), "Failed to set active")
        }
    }

    //fn set_callbacks<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, send_rtp: /*Unknown conversion*//*Unimplemented*/RTSPSendFunc, send_rtcp: /*Unknown conversion*//*Unimplemented*/RTSPSendFunc, user_data: P, notify: Q) {
    //    unsafe { TODO: call ffi::gst_rtsp_stream_transport_set_callbacks() }
    //}

    //fn set_keepalive<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, keep_alive: /*Unknown conversion*//*Unimplemented*/RTSPKeepAliveFunc, user_data: P, notify: Q) {
    //    unsafe { TODO: call ffi::gst_rtsp_stream_transport_set_keepalive() }
    //}

    fn set_timed_out(&self, timedout: bool) {
        unsafe {
            ffi::gst_rtsp_stream_transport_set_timed_out(self.to_glib_none().0, timedout.to_glib());
        }
    }

    //fn set_transport(&self, tr: /*Ignored*/&mut gst_rtsp::RTSPTransport) {
    //    unsafe { TODO: call ffi::gst_rtsp_stream_transport_set_transport() }
    //}

    fn set_url<'a, P: Into<Option<&'a gst_rtsp::RTSPUrl>>>(&self, url: P) {
        let url = url.into();
        let url = url.to_glib_none();
        unsafe {
            ffi::gst_rtsp_stream_transport_set_url(self.to_glib_none().0, url.0);
        }
    }
}
