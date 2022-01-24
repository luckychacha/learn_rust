use bytes::Bytes;
use photon_rs::{PhotonImage, native::open_image_from_bytes, transform};
use lazy_static::lazy_static;

use crate::pb::{Contrast, Crop, Fliph, Flipv, Resize, Watermark, filter::Filter};

use super::{Engine, SpecTransform};

lazy_static! {
    // 预先把水印文件加载为静态变量
    static ref WATERMARK: PhotonImage = {
        // 这里你需要把我 github 项目下的对应图片拷贝到你的目录
        // 在编译的时候 include_bytes! 宏会直接把图片文件读入编译后的二进制
        let data = include_bytes!("../../rust-logo.png");
        let watermark = open_image_from_bytes(data).unwrap();
        transform::resize(&watermark, 64, 64, transform::SamplingFilter::Nearest)
    };
}

pub struct Photon(PhotonImage);

impl TryFrom<Bytes> for Photon {
    type Error = anyhow::Error;

    fn try_from(data: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(open_image_from_bytes(&data)?))
    }
}

impl Engine for Photon {
    fn apply(&mut self, specs: &[crate::pb::Spec]) {
        todo!()
    }

    fn generate(self, format: image::ImageOutputFormat) -> Vec<u8> {
        todo!()
    }
}

impl SpecTransform<&Crop> for Photon {
    fn transform(&mut self, op: &Crop) {
        todo!()
    }
}

impl SpecTransform<&Contrast> for Photon {
    fn transform(&mut self, op: &Contrast) {
        todo!()
    }
}

impl SpecTransform<&Flipv> for Photon {
    fn transform(&mut self, op: &Flipv) {
        todo!()
    }
}

impl SpecTransform<&Fliph> for Photon {
    fn transform(&mut self, op: &Fliph) {
        todo!()
    }
}

impl SpecTransform<&Filter> for Photon {
    fn transform(&mut self, op: &Filter) {
        todo!()
    }
}

impl SpecTransform<&Resize> for Photon {
    fn transform(&mut self, op: &Resize) {
        todo!()
    }
}

impl SpecTransform<&Watermark> for Photon {
    fn transform(&mut self, op: &Watermark) {
        todo!()
    }
}