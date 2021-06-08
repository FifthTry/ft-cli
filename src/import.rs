pub fn import(config: &crate::Config, re_sync: bool) -> crate::Result<()> {
    crate::sync(config, re_sync)
}
