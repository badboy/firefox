use std::ptr;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

use libc::c_void;
use nserror::{nsresult, NS_OK};
use xpcom::{interfaces::nsISupports, xpcom, xpcom_method, RefPtr, nsIID};

static FLIP_FLOP: OnceLock<AtomicBool> = OnceLock::new();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nsEnterpriseUploaderConstructor(iid: &nsIID, result: *mut *mut c_void) -> nsresult {
    *result = ptr::null_mut();

    FLIP_FLOP.set(AtomicBool::new(false));

    let service = EnterpriseUploader::new();
    service.QueryInterface(iid, result)
}

#[xpcom(implement(nsIEnterpriseUploader), atomic)]
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
}
