use crate::resources::Plugin;

#[derive(Debug)]
pub struct DomainPlugin {

}

impl DomainPlugin {
    pub fn new() -> Self {
        Self {

        }
    }
}
impl Plugin for DomainPlugin {
    fn plugin_name(&self) -> &'static str {
        "domain_plugin"
    }
}