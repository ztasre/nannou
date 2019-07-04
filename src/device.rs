use cpal::Device as DeviceTrait;
use cpal::platform::{Device as CpalDevice, Devices as CpalDevices};
use std::ops::Deref;

/// A device that can be used to spawn an audio stream.
pub struct Device {
    pub(crate) device: CpalDevice,
}

/// An iterator yielding all available audio devices.
pub struct Devices {
    pub(crate) devices: CpalDevices,
}

/// An iterator yielding formats that are supported by the backend.
pub type SupportedInputFormats = cpal::platform::SupportedInputFormats;

/// An iterator yielding formats that are supported by the backend.
pub type SupportedOutputFormats = cpal::platform::SupportedOutputFormats;

impl Device {
    /// The unique name associated with this device.
    pub fn name(&self) -> Result<String, cpal::DeviceNameError> {
        self.device.name()
    }

    /// An iterator yielding formats that are supported by the backend.
    ///
    /// Can return an error if the device is no longer valid (e.g. it has been disconnected).
    pub fn supported_input_formats(
        &self
    ) -> Result<SupportedInputFormats, cpal::SupportedFormatsError> {
        self.device.supported_input_formats()
    }

    /// An iterator yielding formats that are supported by the backend.
    ///
    /// Can return an error if the device is no longer valid (e.g. it has been disconnected).
    pub fn supported_output_formats(
        &self
    ) -> Result<SupportedOutputFormats, cpal::SupportedFormatsError> {
        self.device.supported_output_formats()
    }

    /// The default format used for input streams.
    pub fn default_input_format(&self) -> Result<cpal::Format, cpal::DefaultFormatError> {
        self.device.default_input_format()
    }

    /// The default format used for output streams.
    pub fn default_output_format(&self) -> Result<cpal::Format, cpal::DefaultFormatError> {
        self.device.default_output_format()
    }

    /// The maximum number of output channels of any format supported by this device.
    pub fn max_supported_output_channels(&self) -> usize {
        self.supported_output_formats()
            .expect("failed to get supported output audio stream formats")
            .map(|fmt| fmt.channels as usize)
            .max()
            .unwrap_or(0)
    }

    /// The maximum number of input channels of any format supported by this device.
    pub fn max_supported_input_channels(&self) -> usize {
        self.supported_input_formats()
            .expect("failed to get supported input audio stream formats")
            .map(|fmt| fmt.channels as usize)
            .max()
            .unwrap_or(0)
    }
}

impl Deref for Device {
    type Target = CpalDevice;
    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl Iterator for Devices {
    type Item = Device;
    fn next(&mut self) -> Option<Self::Item> {
        self.devices.next().map(|device| Device { device })
    }
}
