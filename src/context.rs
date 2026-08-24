use anyhow::Result;
use vulkano::device::DeviceExtensions;
use vulkano::{Version, VulkanLibrary};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::swapchain::Surface;
use winit::window::Window;

pub struct Context {

}


// Change to use real Result type with error propagation
impl Context {
    pub fn new(compatible_window: &Window) -> Result<Context> {

        // Create vulkan instance
        let library = VulkanLibrary::new()?;
        let mut instance_extensions = Surface::required_extensions(&compatible_window)?; // If this crashes it's the raw handles versions mismatch
        

        if cfg!(debug_assertions) {
            instance_extensions.ext_debug_utils = library.supported_extensions().ext_debug_utils; // Sets to true iif they exist
        }

        let instance = Instance::new(library, InstanceCreateInfo {
            enabled_extensions: instance_extensions,
            ..InstanceCreateInfo::application_from_cargo_toml()
        })?;


        // Filter trough the devices to keep the compatible onces
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let physical_devices = instance.enumerate_physical_devices()?.filter(|p| {
            p.api_version() >= Version::V1_3 || p.supported_extensions().khr_dynamic_rendering
        }).filter(|p| {
            p.supported_extensions().contains(&device_extensions)
        }).filter_map(|p| {
            p.queue_family_properties()
        });



        Ok(Context {

        })
    }


}