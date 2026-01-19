use nokhwa::{pixel_format::RgbFormat, utils::{CameraIndex, RequestedFormat, RequestedFormatType}, Camera as NokhwaCamera};

use crate::Frame;

type Result<T> = std::result::Result<T, nokhwa::NokhwaError>;

pub struct Camera {
    inner: NokhwaCamera,
}

impl Camera {
    pub fn new(index: u32) -> Result<Self> {
        Self::with_format(
            index,
            RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate),
        )
    }

    pub fn with_format(index: u32, format: RequestedFormat) -> Result<Self> {
        let mut camera = NokhwaCamera::new(CameraIndex::Index(index), format)?;

        camera.open_stream()?;

        Ok(Self { inner: camera })
    }

    pub fn capture(&mut self) -> Result<Frame> {
        let frame = self.inner.frame()?;
        let decoded = frame.decode_image::<RgbFormat>()?;

        Ok(Frame::new(decoded.into_raw()))
    }

    pub fn capture_many(&mut self, count: usize) -> Result<Vec<Frame>> {
        (0..count).map(|_| self.capture()).collect()
    }
}