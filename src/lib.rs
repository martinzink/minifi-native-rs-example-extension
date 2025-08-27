mod processors;

use minifi_native::sys::MINIFI_API_VERSION_TAG;

#[used]
static PRESERVE_API_VERSION: &[u8] = MINIFI_API_VERSION_TAG;
