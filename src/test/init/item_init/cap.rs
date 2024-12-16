use anthol_store::item::{
    attr::{AttrIndex, AttrIndexes, AttrSpecificData, ItemAttrsV1},
    image::ItemImagesV1,
    spec::{ItemSpecsV1, SpecCategory, SpecKey, SpecLabel},
    Item, ItemVersion,
};
use shared::{
    item::{
        attr::{AttrColor, AttrKeys, AttrType},
        MediaDataWithCaption, Tag,
    },
    media::{
        mime::{Mime, MimeImage},
        MediaData,
    },
    unit::Currency,
};

pub fn get() -> Item {
    Item {
        id: "baseball-cap".try_into().unwrap(),
        name: "The Baseball Cap".to_string(),
        version: ItemVersion::V1 {
            descriptions: vec![
                "カジュアルな場に最適なベースボールキャップ。".to_string(),
            ],
            tags: vec![Tag::new("cap").unwrap()],
            images: ItemImagesV1::builder()
                .image(
                    0,
                    MediaDataWithCaption::builder()
                        .data(
                            MediaData::builder()
                                .url("https://images.unsplash.com/photo-1519079754742-f83afaef6d35")
                                .mime(Mime::Image(MimeImage::Jpeg))
                                .alt("A black baseball cap with a white logo on the front.")
                                .build(),
                        )
                        .caption("Polaroid OneStep 2, seen from the front in its black-and-white casing, allows users to immediately print out the photos they have taken.")
                        .build(),
                )
                .image(
                    1,
                    MediaDataWithCaption::builder()
                        .data(
                            MediaData::builder()
                                .url("https://images.unsplash.com/photo-1547022188-a8c4136f649b")
                                .mime(Mime::Image(MimeImage::Jpeg))
                                .alt("A yellow baseball cap on the front.")
                                .build(),
                        )
                        .caption("The design is reminiscent of the original OneStep and is packed with easy-to-use features.")
                        .build(),
                )
                .image(
                    2,
                    MediaDataWithCaption::builder()
                        .data(
                            MediaData::builder()
                                .url("https://images.unsplash.com/photo-1526170160160-1a5eb242ab58")
                                .mime(Mime::Image(MimeImage::Jpeg))
                                .alt("A red baseball cap on the front.")
                                .build(),
                        )
                        .caption("White and black Polaroid instant camera beside books.")
                        .build(),
                )
                .index_vec(0, vec![0])
                .index_vec(1, vec![1])
                .index_vec(2, vec![2])
                .build(),
            specs: ItemSpecsV1::builder()
                .spec(
                    0,
                    SpecCategory::builder("Size")
                        .label(0,
                            SpecLabel::builder("Height")
                                .value(0, vec!["12cm"]).build())
                        .label(1,
                            SpecLabel::builder("Width")
                                .value(0, vec!["12cm"]).build())
                        .label(2,
                            SpecLabel::builder("Depth")
                                .value(0, vec!["17.3cm"]).build())
                        .build()
                )
                .index(
                    0,
                    SpecKey::builder(0)
                        .label(0, 0)
                        .label(1, 0)
                        .label(2, 0)
                        .build()
                )
                .build(),
            attrs: ItemAttrsV1::builder()
                .attr(
                    AttrKeys::new(0, 0, 0, 0),
                    AttrSpecificData::builder()
                        .stock(10)
                        .price(Currency::USD, 99.5)
                        .price(Currency::JPY, 10000_f64)
                        .image_vec_key(0)
                        .spec_key(0)
                        .build()
                )
                .attr(
                    AttrKeys::new(1, 0, 0, 0),
                    AttrSpecificData::builder()
                        .stock(10)
                        .price(Currency::USD, 82.5)
                        .price(Currency::JPY, 10000_f64)
                        .image_vec_key(1)
                        .spec_key(0)
                        .build()
                )
                .attr(
                    AttrKeys::new(2, 0, 0, 0),
                    AttrSpecificData::builder()
                        .stock(2)
                        .price(Currency::USD, 96.5)
                        .price(Currency::JPY, 10000_f64)
                        .image_vec_key(2)
                        .spec_key(0)
                        .build()
                )
                .attr(
                    AttrKeys::new(0, 1, 0, 0),
                    AttrSpecificData::builder()
                        .stock(0)
                        .price(Currency::USD, 79.0)
                        .price(Currency::JPY, 10000_f64)
                        .image_vec_key(0)
                        .spec_key(0)
                        .build()
                )
                .attr(
                    AttrKeys::new(1, 1, 0, 0),
                    AttrSpecificData::builder()
                        .stock(10)
                        .price(Currency::USD, 92.3)
                        .price(Currency::JPY, 10000_f64)
                        .image_vec_key(1)
                        .spec_key(0)
                        .build()
                )
                .attr(
                    AttrKeys::new(2, 1, 0, 0),
                    AttrSpecificData::builder()
                        .stock(10)
                        .price(Currency::USD, 89.0)
                        .price(Currency::JPY, 10000_f64)
                        .image_vec_key(2)
                        .spec_key(0)
                        .build()
                )
                .attr(
                    AttrKeys::new(0, 2, 0, 0),
                    AttrSpecificData::builder()
                        .stock(10)
                        .price(Currency::USD, 94.5)
                        .price(Currency::JPY, 10000_f64)
                        .image_vec_key(0)
                        .spec_key(0)
                        .build()
                )
                .attr(
                    AttrKeys::new(1, 2, 0, 0),
                    AttrSpecificData::builder()
                        .stock(10)
                        .price(Currency::USD, 100.5)
                        .price(Currency::JPY, 10000_f64)
                        .image_vec_key(1)
                        .spec_key(0)
                        .build()
                )
                .attr(
                    AttrKeys::new(2, 2, 0, 0),
                    AttrSpecificData::builder()
                        .stock(0)
                        .price(Currency::USD, 96.5)
                        .price(Currency::JPY, 10000_f64)
                        .image_vec_key(2)
                        .spec_key(0)
                        .build()
                )
                .attr(
                    AttrKeys::new(0, 3, 0, 0),
                    AttrSpecificData::builder()
                        .stock(10)
                        .price(Currency::USD, 98.3)
                        .price(Currency::JPY, 10000_f64)
                        .image_vec_key(0)
                        .spec_key(0)
                        .build()
                )
                .indexes(
                    AttrIndexes::builder()
                        .attr(
                            0,
                            AttrIndex::builder("Color")
                                .label(0, AttrType::Color( AttrColor { name: "Black".to_string(), color: (0, 13, 15)} ))
                                .label(1, AttrType::Color( AttrColor { name: "Mustard Yellow".to_string(), color: (246, 188, 3)} ))
                                .label(2, AttrType::Color( AttrColor { name: "Clement Red".to_string(), color: (255, 61, 104)} ))
                                .build()
                        )
                        .attr(
                            1,
                            AttrIndex::builder("Size")
                                .label(0, AttrType::Text("S".to_string()))
                                .label(1, AttrType::Text("M".to_string()))
                                .label(2, AttrType::Text("L".to_string()))
                                .label(3, AttrType::Text("XL".to_string()))
                                .build()
                        )
                        .build()
                )
                .build(),
        }
    }
}
