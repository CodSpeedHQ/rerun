---
title: "TensorData"
---
<!-- DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/docs/mod.rs -->

An N-dimensional array of numbers.

The number of dimensions and their respective lengths is specified by the `shape` field.
The dimensions are ordered from outermost to innermost. For example, in the common case of
a 2D RGB Image, the shape would be `[height, width, channel]`.

These dimensions are combined with an index to look up values from the `buffer` field,
which stores a contiguous array of typed values.

Note that the buffer may in a format with downsampled chroma, such as NV12 or YUY2.
For chroma downsampled formats the shape has to be the shape of the decoded image.

## Fields

* data: [`TensorData`](../datatypes/tensor_data.md)

## API reference links
 * 🌊 [C++ API docs for `TensorData`](https://ref.rerun.io/docs/cpp/stable/structrerun_1_1components_1_1TensorData.html)
 * 🐍 [Python API docs for `TensorData`](https://ref.rerun.io/docs/python/stable/common/components#rerun.components.TensorData)
 * 🦀 [Rust API docs for `TensorData`](https://docs.rs/rerun/latest/rerun/components/struct.TensorData.html)


## Used by

* [`BarChart`](../archetypes/bar_chart.md)
* [`Image`](../archetypes/image.md)
* [`Mesh3D`](../archetypes/mesh3d.md)
* [`Tensor`](../archetypes/tensor.md)
