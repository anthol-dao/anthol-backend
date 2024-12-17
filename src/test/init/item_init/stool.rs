use anthol_store::item::{
    attr::{AttrIndexes, AttrSpecificData, ItemAttrsV1},
    image::ItemImagesV1,
    spec::{ItemSpecsV1, SpecCategory, SpecKey, SpecLabel},
    Item, ItemVersion,
};
use common::{
    item::{attr::AttrKeys, MediaDataWithCaption, Tag},
    media::{
        mime::{Mime, MimeImage},
        MediaData,
    },
    unit::Currency,
};

pub fn get() -> Item {
    Item {
        id: "wiggle-stool".try_into().unwrap(),
        name: "Wiggle Stool".to_string(),
        version: ItemVersion::V1 {
            descriptions: vec![
                "The Wiggle Stool is part of Frank Gehry's 1972 furniture series 'Easy Edges', which successfully introduced a new aesthetic dimension to such an everyday material as cardboard. The iconic stool is robust and lends a striking note to any interior.".to_string(),
            ],
            tags: vec![Tag::new("furniture").unwrap()],
            images: ItemImagesV1::builder()
                .image(
                    0,
                    MediaDataWithCaption::builder()
                        .data(
                            MediaData::builder()
                                .cid("bafybeifwfqsftwqgqee6e5o44ylydjki6vvbd3gzur3kw3couez7tqjihe/front.jpg")
                                .mime(Mime::Image(MimeImage::Jpeg))
                                .alt("The front image of the stool.")
                                .build(),
                        )
                        .caption("The architect Frank Gehry is known for his use of unusual materials. With his furniture series 'Easy Edges', he succeeded in bringing a new aesthetic dimension to such an everyday material as cardboard.")
                        .build(),
                )
                .image(
                    1,
                    MediaDataWithCaption::builder()
                        .data(
                            MediaData::builder()
                                .cid("bafybeifwfqsftwqgqee6e5o44ylydjki6vvbd3gzur3kw3couez7tqjihe/side.jpg")
                                .mime(Mime::Image(MimeImage::Jpeg))
                                .alt("The side image of the stool.")
                                .build(),
                        )
                        .caption("The sculptural form of the Wiggle Stool makes it stand out. Although surprisingly simple in appearance, it is constructed with the consummate skill of an architect, making it not only very comfortable but also durable and robust.")
                        .build(),
                )
                .index_vec(0, vec![0, 1])
                .build(),
            specs: ItemSpecsV1::builder()
                .spec(
                    0,
                    SpecCategory::builder("Size")
                        .label(0,
                            SpecLabel::builder("Height")
                                .value(0, vec!["406mm"]).build())
                        .label(1,
                            SpecLabel::builder("Width")
                                .value(0, vec!["430mm"]).build())
                        .label(2,
                            SpecLabel::builder("Depth")
                                .value(0, vec!["400mm"]).build())
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
                        .price(Currency::USD, 845.00)
                        .price(Currency::JPY, 78100_f64)
                        .image_vec_key(0)
                        .spec_key(0)
                        .build()
                )
                .indexes(AttrIndexes::default())
                .build(),
        }
    }
}
