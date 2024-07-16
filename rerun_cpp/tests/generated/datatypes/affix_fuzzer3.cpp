// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs".

#include "affix_fuzzer3.hpp"

#include "affix_fuzzer1.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun::datatypes {}

namespace rerun {
    const std::shared_ptr<arrow::DataType>& Loggable<datatypes::AffixFuzzer3>::arrow_datatype() {
        static const auto datatype = arrow::dense_union({
            arrow::field("_null_markers", arrow::null(), true, nullptr),
            arrow::field("degrees", arrow::float32(), false),
            arrow::field(
                "craziness",
                arrow::list(arrow::field(
                    "item",
                    Loggable<rerun::datatypes::AffixFuzzer1>::arrow_datatype(),
                    false
                )),
                false
            ),
            arrow::field(
                "fixed_size_shenanigans",
                arrow::fixed_size_list(arrow::field("item", arrow::float32(), false), 3),
                false
            ),
            arrow::field("empty_variant", arrow::null(), true),
        });
        return datatype;
    }

    Result<std::shared_ptr<arrow::Array>> Loggable<datatypes::AffixFuzzer3>::to_arrow(
        const datatypes::AffixFuzzer3* instances, size_t num_instances
    ) {
        // TODO(andreas): Allow configuring the memory pool.
        arrow::MemoryPool* pool = arrow::default_memory_pool();
        auto datatype = arrow_datatype();

        ARROW_ASSIGN_OR_RAISE(auto builder, arrow::MakeBuilder(datatype, pool))
        if (instances && num_instances > 0) {
            RR_RETURN_NOT_OK(Loggable<datatypes::AffixFuzzer3>::fill_arrow_array_builder(
                static_cast<arrow::DenseUnionBuilder*>(builder.get()),
                instances,
                num_instances
            ));
        }
        std::shared_ptr<arrow::Array> array;
        ARROW_RETURN_NOT_OK(builder->Finish(&array));
        return array;
    }

    rerun::Error Loggable<datatypes::AffixFuzzer3>::fill_arrow_array_builder(
        arrow::DenseUnionBuilder* builder, const datatypes::AffixFuzzer3* elements,
        size_t num_elements
    ) {
        if (builder == nullptr) {
            return rerun::Error(ErrorCode::UnexpectedNullArgument, "Passed array builder is null.");
        }
        if (elements == nullptr) {
            return rerun::Error(
                ErrorCode::UnexpectedNullArgument,
                "Cannot serialize null pointer to arrow array."
            );
        }

        ARROW_RETURN_NOT_OK(builder->Reserve(static_cast<int64_t>(num_elements)));
        for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
            const auto& union_instance = elements[elem_idx];
            ARROW_RETURN_NOT_OK(builder->Append(static_cast<int8_t>(union_instance.get_union_tag()))
            );

            auto variant_index = static_cast<int>(union_instance.get_union_tag());
            auto variant_builder_untyped = builder->child_builder(variant_index).get();

            using TagType = datatypes::detail::AffixFuzzer3Tag;
            switch (union_instance.get_union_tag()) {
                case TagType::None: {
                    ARROW_RETURN_NOT_OK(variant_builder_untyped->AppendNull());
                } break;
                case TagType::degrees: {
                    auto variant_builder =
                        static_cast<arrow::FloatBuilder*>(variant_builder_untyped);
                    ARROW_RETURN_NOT_OK(
                        variant_builder->Append(union_instance.get_union_data().degrees)
                    );
                } break;
                case TagType::craziness: {
                    auto variant_builder =
                        static_cast<arrow::ListBuilder*>(variant_builder_untyped);
                    (void)variant_builder;
                    return rerun::Error(
                        ErrorCode::NotImplemented,
                        "Failed to serialize AffixFuzzer3::craziness: objects (Object(\"rerun.testing.datatypes.AffixFuzzer1\")) in unions not yet implemented"
                    );
                } break;
                case TagType::fixed_size_shenanigans: {
                    auto variant_builder =
                        static_cast<arrow::FixedSizeListBuilder*>(variant_builder_untyped);
                    (void)variant_builder;
                    return rerun::Error(
                        ErrorCode::NotImplemented,
                        "Failed to serialize AffixFuzzer3::fixed_size_shenanigans: FixedSizeListBuilder in unions not yet implemented"
                    );
                } break;
                case TagType::empty_variant: {
                    auto variant_builder =
                        static_cast<arrow::NullBuilder*>(variant_builder_untyped);
                    ARROW_RETURN_NOT_OK(variant_builder->AppendNull());
                } break;
            }
        }

        return Error::ok();
    }
} // namespace rerun
