use std::borrow::Cow;

use re_chunk::RowId;
use re_types::{
    components::{ChannelDataType, ColorModel, Colormap},
    datatypes::Blob,
    tensor_data::TensorElement,
};

/// Represents an `Image`, `SegmentationImage` or `DepthImage`.
///
/// It has enough information to render the image on the screen.
#[derive(Clone)]
pub struct ImageInfo {
    /// The row id that contaoned the blob.
    ///
    /// Can be used instead of hashing [`Self::blob`].
    pub blob_row_id: RowId,

    /// The image data, row-wise, with stride=width.
    pub blob: Blob,

    /// Width and height
    pub resolution: [u32; 2],

    /// The innermost data type (e.g. `U8`).
    pub data_type: ChannelDataType,

    /// `None` for depth images and segmentation images,
    /// `Some` for color images.
    pub color_model: Option<ColorModel>,

    /// Primarily for depth images atm
    pub colormap: Option<Colormap>,
    // TODO(#6386): `PixelFormat` and `ColorModel`
}

impl ImageInfo {
    #[inline]
    pub fn width(&self) -> u32 {
        self.resolution[0]
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.resolution[1]
    }

    /// 1 for grayscale and depth images, 3 for RGB, etc.
    #[doc(alias = "components")]
    #[doc(alias = "depth")]
    #[inline]
    pub fn num_channels(&self) -> usize {
        self.color_model.map_or(1, ColorModel::num_channels)
    }

    #[inline]
    pub fn bits_per_texel(&self) -> usize {
        // TODO(#6386): use `PixelFormat`
        self.data_type.bits() * self.num_channels()
    }

    /// Get the value of the element at the given index.
    ///
    /// Return `None` if out-of-bounds.
    #[inline]
    pub fn get_xyc(&self, x: u32, y: u32, channel: u32) -> Option<TensorElement> {
        let width = self.width();
        let height = self.height();
        let num_channels = self.num_channels();

        if width <= x || height <= y {
            return None;
        }
        debug_assert!(channel < num_channels as u32);
        if num_channels as u32 <= channel {
            return None;
        }

        let stride = width; // TODO(#6008): support stride
        let offset = (y as usize * stride as usize + x as usize) * num_channels + channel as usize;

        match self.data_type {
            ChannelDataType::U8 => self.blob.get(offset).copied().map(TensorElement::U8),
            ChannelDataType::U16 => get(&self.blob, offset).map(TensorElement::U16),
            ChannelDataType::U32 => get(&self.blob, offset).map(TensorElement::U32),
            ChannelDataType::U64 => get(&self.blob, offset).map(TensorElement::U64),

            ChannelDataType::I8 => get(&self.blob, offset).map(TensorElement::I8),
            ChannelDataType::I16 => get(&self.blob, offset).map(TensorElement::I16),
            ChannelDataType::I32 => get(&self.blob, offset).map(TensorElement::I32),
            ChannelDataType::I64 => get(&self.blob, offset).map(TensorElement::I64),

            ChannelDataType::F16 => get(&self.blob, offset).map(TensorElement::F16),
            ChannelDataType::F32 => get(&self.blob, offset).map(TensorElement::F32),
            ChannelDataType::F64 => get(&self.blob, offset).map(TensorElement::F64),
        }
    }

    /// Total number of elements in the image, e.g. `W x H x 3` for an RGB image.
    #[inline]
    pub fn num_elements(&self) -> usize {
        self.blob.len() * 8 / self.bits_per_texel()
    }

    /// Cast the buffer to the given type.
    ///
    /// This will never fail.
    /// If the buffer is 5 bytes long and the target type is `f32`, the last byte is just ignored.
    ///
    /// Cheap in most cases, but if the input buffer is not aligned to the element type,
    /// this function will copy the data.
    pub fn to_slice<T: bytemuck::Pod>(&self) -> Cow<'_, [T]> {
        let element_size = std::mem::size_of::<T>();
        let num_elements = self.blob.len() / element_size;
        let num_bytes = num_elements * element_size;
        let bytes = &self.blob[..num_bytes];

        if let Ok(slice) = bytemuck::try_cast_slice(bytes) {
            Cow::Borrowed(slice)
        } else {
            // This should happen very rarely.
            // But it can happen, e.g. when logging a `1x1xu8` image followed by a `1x1xf32` image
            // to the same entity path, and they are put in the same chunk.

            if cfg!(debug_asserttions) {
                re_log::warn_once!(
                    "The image buffer was not aligned to the element type {}",
                    std::any::type_name::<T>()
                );
            }
            re_tracing::profile_scope!("copy_image_buffer");

            let mut dest = vec![T::zeroed(); num_elements];
            let dest_bytes: &mut [u8] = bytemuck::cast_slice_mut(&mut dest);
            dest_bytes.copy_from_slice(bytes);
            Cow::Owned(dest)
        }
    }
}

fn get<T: bytemuck::Pod>(blob: &[u8], element_offset: usize) -> Option<T> {
    // NOTE: `blob` is not necessary aligned to `T`,
    // hence the complexity of this function.

    let size = std::mem::size_of::<T>();
    let byte_offset = element_offset * size;
    if blob.len() <= byte_offset + size {
        return None;
    }

    let slice = &blob[byte_offset..byte_offset + size];

    let mut dest = T::zeroed();
    bytemuck::bytes_of_mut(&mut dest).copy_from_slice(slice);
    Some(dest)
}
