mod processors;

#[used]
static API_VERSION_STRING: &str = concat!(
"MINIFI_API_VERSION=[",
stringify!(MINIFI_API_MAJOR_VERSION),
".",
stringify!(MINIFI_API_MINOR_VERSION),
".",
stringify!(MINIFI_API_PATCH_VERSION),
"]"
);