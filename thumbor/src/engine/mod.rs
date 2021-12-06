use image::ImageOutputFormat;

use crate::pb::Spec;

mod photon;
pub use photon::Photon;

pub trait Engine {
    // 对 engine 按照 specs 进行一系列有序的处理。
    fn apply(&mut self, specs: &[Spec]);

    // 从 engine 中生成目标图片，注意这里用的是 self，而非 self 的引用
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

// SpecTransform: 未来如果添加更多 spec，只需要实现它即可。
pub trait SpecTransform<T> {

    // 对图片使用 op 做 transform。
    fn transform(&mut self, op: T);
}