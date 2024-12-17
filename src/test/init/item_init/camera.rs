use anthol_store::item::{
    attr::{AttrIndexes, AttrSpecificData, ItemAttrsV1},
    image::ItemImagesV1,
    spec::{ItemSpecsV1, SpecCategory, SpecKey, SpecLabel},
    Item, ItemVersion,
};
use common::{
    item::{attr::AttrKeys, ItemId, MediaDataWithCaption, Tag},
    media::{
        mime::{Mime, MimeImage},
        MediaData,
    },
    unit::Currency,
};

pub fn get() -> Item {
    Item {
        id: ItemId::new("polaroid-onestep-2").unwrap(),
        name: "Polaroid OneStep 2".to_string(),
        version: ItemVersion::V1 {
            descriptions: vec![
                "ポラロイドカメラ。".to_string(),
                "古くて新鮮なカメラで日常の一瞬を切り取る。".to_string(),
            ],
            tags: vec![Tag::new("camera").unwrap(), Tag::new("polaroid").unwrap()],
            images: ItemImagesV1::builder()
                .image(
                    0,
                    MediaDataWithCaption::builder()
                        .data(
                            MediaData::builder()
                                .url("https://images.unsplash.com/photo-1526170375885-4d8ecf77b99f")
                                .mime(Mime::Image(MimeImage::Jpeg))
                                .alt("Polaroid OneStep 2")
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
                                .alt("Polaroid OneStep 2")
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
                                .alt("Polaroid OneStep 2")
                                .build(),
                        )
                        .caption("White and black Polaroid instant camera beside books.")
                        .build(),
                )
                .image(
                    3,
                    MediaDataWithCaption::builder()
                        .data(
                            MediaData::builder()
                                .url("https://images.unsplash.com/photo-1526170286768-b3c80b34b036")
                                .mime(Mime::Image(MimeImage::Jpeg))
                                .alt("Polaroid OneStep 2")
                                .build(),
                        )
                        .caption("With Impossible color instant films.")
                        .build(),
                )
                .image(
                    4,
                    MediaDataWithCaption::builder()
                        .data(
                            MediaData::builder()
                                .url("https://images.unsplash.com/photo-1545155085-967cb231d65f")
                                .mime(Mime::Image(MimeImage::Jpeg))
                                .alt("Polaroid OneStep 2")
                                .build(),
                        )
                        .caption("Polaroid OneStep 2")
                        .build(),
                )
                .index_vec(0, vec![0, 1, 2, 3, 4])
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
                    AttrKeys::default(),
                    AttrSpecificData::builder()
                        .stock(10)
                        .price(Currency::USD, 99.5)
                        .price(Currency::JPY, 10000_f64)
                        .image_vec_key(0)
                        .spec_key(0)
                        .build()
                )
                .indexes(AttrIndexes::default())
                .build(),
        }
    }
}
