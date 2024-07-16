//! Upload images to [`re_renderer`].

use std::borrow::Cow;

use egui::util::hash;
use wgpu::TextureFormat;

use re_chunk::RowId;
use re_renderer::{
    renderer::{ColorMapper, ColormappedTexture},
    resource_managers::Texture2DCreationDesc,
    RenderContext,
};
use re_types::components::Colormap;
use re_types::{components::ElementType, tensor_data::TensorDataMeaning};

use crate::{
    gpu_bridge::colormap::colormap_to_re_renderer, Annotations, ImageComponents, TensorStats,
};

use super::get_or_create_texture;

// ----------------------------------------------------------------------------

#[derive(Copy, Clone, Hash)]
enum TextureKeyUsage {
    // AnnotationContextColormap,
    TensorData(TensorDataMeaning),
}

/// Returns a texture key for a given row id & usage.
///
/// Several textures may be created from the same row.
/// This makes sure that they all get different keys!
fn generate_texture_key(row_id: RowId, usage: TextureKeyUsage) -> u64 {
    hash((row_id, usage))
}

pub fn image_to_gpu(
    render_ctx: &RenderContext,
    debug_name: &str,
    image: &ImageComponents,
    meaning: TensorDataMeaning,
    tensor_stats: &TensorStats,
    _annotations: &Annotations,
) -> anyhow::Result<ColormappedTexture> {
    re_tracing::profile_function!(format!(
        "meaning: {:?}, resolution: {:?}, element_type: {:?}",
        meaning, image.resolution, image.element_type,
    ));

    let texture_key = generate_texture_key(image.row_id, TextureKeyUsage::TensorData(meaning));

    assert_eq!(
        meaning,
        TensorDataMeaning::Depth,
        "Only depth images are implemented atm"
    );
    assert_eq!(
        image.color_model, None,
        "Only depth images are implemented atm"
    );

    let range = data_range(tensor_stats, image.element_type);

    let texture = get_or_create_texture(render_ctx, texture_key, || {
        general_texture_creation_desc_from_image(debug_name, image)
    })
    .map_err(|err| anyhow::anyhow!("Failed to create depth texture: {err}"))?;

    Ok(ColormappedTexture {
        texture,
        range,
        decode_srgb: false,
        multiply_rgb_with_alpha: false,
        gamma: 1.0,
        color_mapper: ColorMapper::Function(colormap_to_re_renderer(
            image.colormap.unwrap_or(Colormap::Turbo),
        )),
        shader_decoding: None,
    })
}

fn data_range(tensor_stats: &TensorStats, element_type: ElementType) -> [f32; 2] {
    let default_min = 0.0;
    let default_max = if element_type.is_float() {
        1.0
    } else {
        element_type.max_value()
    };

    let range = tensor_stats
        .finite_range
        .unwrap_or((default_min, default_max));
    let (mut min, mut max) = range;

    if !min.is_finite() {
        min = default_min;
    }
    if !max.is_finite() {
        max = default_max;
    }

    if max <= min {
        min = default_min;
        max = default_max;
    }

    [min as f32, max as f32]
}

/// Uploads the image to a texture in a format that closely resembled the input.
/// Uses no `Unorm/Snorm` formats.
fn general_texture_creation_desc_from_image<'a>(
    debug_name: &str,
    image: &'a ImageComponents,
) -> Texture2DCreationDesc<'a> {
    re_tracing::profile_function!();

    let [width, height] = image.resolution;

    let buf: &[u8] = image.blob.as_ref();

    let (data, format) = match image.element_type {
        ElementType::U8 => (Cow::Borrowed(buf), TextureFormat::R8Uint),
        ElementType::U16 => (Cow::Borrowed(buf), TextureFormat::R16Uint),
        ElementType::U32 => (Cow::Borrowed(buf), TextureFormat::R32Uint),
        ElementType::U64 => (
            // wgpu doesn't support u64 textures
            narrow_u64_to_f32s(&image.as_slice()),
            TextureFormat::R32Float,
        ),

        ElementType::I8 => (Cow::Borrowed(buf), TextureFormat::R8Sint),
        ElementType::I16 => (Cow::Borrowed(buf), TextureFormat::R16Sint),
        ElementType::I32 => (Cow::Borrowed(buf), TextureFormat::R32Sint),
        ElementType::I64 => (
            // wgpu doesn't support i64 textures
            narrow_i64_to_f32s(&image.as_slice()),
            TextureFormat::R32Float,
        ),

        ElementType::F16 => (Cow::Borrowed(buf), TextureFormat::R16Float),
        ElementType::F32 => (Cow::Borrowed(buf), TextureFormat::R32Float),
        ElementType::F64 => (
            // wgpu doesn't support f64 textures
            narrow_f64_to_f32s(&image.as_slice()),
            TextureFormat::R32Float,
        ),
    };

    Texture2DCreationDesc {
        label: debug_name.into(),
        data,
        format,
        width,
        height,
    }
}

// wgpu doesn't support u64 textures, so we need to narrow to f32:
fn narrow_u64_to_f32s(slice: &[u64]) -> Cow<'static, [u8]> {
    re_tracing::profile_function!();
    let bytes: Vec<u8> = slice
        .iter()
        .flat_map(|&f| (f as f32).to_le_bytes())
        .collect();
    bytes.into()
}

// wgpu doesn't support i64 textures, so we need to narrow to f32:
fn narrow_i64_to_f32s(slice: &[i64]) -> Cow<'static, [u8]> {
    re_tracing::profile_function!();
    let bytes: Vec<u8> = slice
        .iter()
        .flat_map(|&f| (f as f32).to_le_bytes())
        .collect();
    bytes.into()
}

// wgpu doesn't support f64 textures, so we need to narrow to f32:
fn narrow_f64_to_f32s(slice: &[f64]) -> Cow<'static, [u8]> {
    re_tracing::profile_function!();
    let bytes: Vec<u8> = slice
        .iter()
        .flat_map(|&f| (f as f32).to_le_bytes())
        .collect();
    bytes.into()
}
