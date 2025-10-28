use std::ffi::CStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

use libc::c_char;
use nserror::{nsresult, NS_OK};
use xpcom::interfaces::{nsIEnterpriseUploader, nsISupports};
use xpcom::{xpcom, xpcom_method, RefPtr};

static FLIP_FLOP: OnceLock<AtomicBool> = OnceLock::new();

#[unsafe(no_mangle)]
#[expect(clippy::missing_safety_doc, reason = "Inherently unsafe.")]
pub unsafe extern "C" fn new_enterprise_uploader_service(
    result: *mut *const nsIEnterpriseUploader,
) {
    _ = FLIP_FLOP.set(AtomicBool::new(false));

    unsafe {
        let service: RefPtr<EnterpriseUploader> = EnterpriseUploader::new();
        RefPtr::new(service.coerce::<nsIEnterpriseUploader>()).forget(&mut *result);
    }
}

#[xpcom(implement(nsIEnterpriseUploader, nsIObserver), atomic)]
pub struct EnterpriseUploader {}

impl EnterpriseUploader {
    fn new() -> RefPtr<EnterpriseUploader> {
        EnterpriseUploader::allocate(InitEnterpriseUploader {})
    }

    xpcom_method!(get_is_alive => GetIsAlive() -> bool);
    fn get_is_alive(&self) -> Result<bool, nsresult> {
        let cell = FLIP_FLOP.wait();
        Ok(cell.fetch_not(Ordering::SeqCst))
    }

    xpcom_method!(observe => Observe(subject: *const nsISupports, topic: *const c_char, data: *const u16));
    fn observe(
        &self,
        _subject: *const nsISupports,
        topic: *const c_char,
        _data: *const u16,
    ) -> Result<nsresult, nsresult> {
        unsafe {
            let topic = CStr::from_ptr(topic);
            let topic = topic.to_string_lossy();
            eprintln!("EnterpriseUploader.observe. topic={topic}");
            Ok(NS_OK)
        }
    }
}
