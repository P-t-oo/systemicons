use windows::Win32::{
    Graphics::Gdi::{
        DeleteObject, HBITMAP, HGDIOBJ
    }, UI::WindowsAndMessaging::{DestroyIcon, HICON}
};

pub struct IconDropper(pub HICON);

impl Drop for IconDropper {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe { let _ = DestroyIcon(self.0); };
        }
    }
}

pub struct BitmapDropper(pub HBITMAP);

impl Drop for BitmapDropper {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe { let _ = DeleteObject(HGDIOBJ(self.0.0)); };
        }
    }
}



