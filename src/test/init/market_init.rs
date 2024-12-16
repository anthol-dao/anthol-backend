use shared::{
    item::{
        attr::AttrKeys, tag::Tag, ItemAttrSpecificDataInMarket, ItemAttrSpecificDataInMarketV1,
        ItemDataInMarket, ItemDataInMarketV1, ItemId,
    },
    market::{MarketId, MarketName},
    media::{
        mime::{Mime, MimeImage},
        MediaData,
    },
    store::StoreId,
    unit::{Currency, Price},
};

type MarketInitVec = Vec<(
    MarketId,
    MarketName,
    Vec<((StoreId, ItemId), ItemDataInMarket)>,
)>;

pub fn markets_init() -> MarketInitVec {
    vec![
        (
            "delighted-cameras".try_into().unwrap(),
            "Delighted Cameras".to_string(),
            vec![(
                (
                    "specimens".try_into().unwrap(),
                    "polaroid-onestep-2".try_into().unwrap(),
                ),
                ItemDataInMarket::V1(ItemDataInMarketV1 {
                    item_name: "Polaroid OneStep 2".to_string(),
                    store_name: "Specimens".to_string(),
                    tags: vec![Tag::new("camera").unwrap(), Tag::new("polaroid").unwrap()],
                    attrs: vec![(
                        AttrKeys::default(),
                        ItemAttrSpecificDataInMarket::V1(ItemAttrSpecificDataInMarketV1 {
                            is_in_stock: true,
                            price: vec![(Currency::USD, Price::new(99.5))]
                                .into_iter()
                                .collect(),
                            image: MediaData::builder()
                                .url("https://images.unsplash.com/photo-1526170375885-4d8ecf77b99f")
                                .mime(Mime::Image(MimeImage::Jpeg))
                                .alt("Polaroid OneStep 2")
                                .build(),
                        }),
                    )],
                }),
            )],
        ),
        (
            "specimens-authorized".try_into().unwrap(),
            "Specimens Authorized".to_string(),
            vec![
                (
                    (
                        "specimens".try_into().unwrap(),
                        "pen".try_into().unwrap()
                    ),
                    ItemDataInMarket::V1(ItemDataInMarketV1 {
                        item_name: "Pen".to_string(),
                        store_name: "Specimens".to_string(),
                        tags: vec![Tag::new("pen").unwrap()],
                        attrs: vec![(
                            AttrKeys::default(),
                            ItemAttrSpecificDataInMarket::V1(ItemAttrSpecificDataInMarketV1 {
                                is_in_stock: true,
                                price: vec![(Currency::USD, Price::new(99.5))]
                                    .into_iter()
                                    .collect(),
                                image: MediaData::builder()
                                    .url("https://images.unsplash.com/photo-1519079754742-f83afaef6d35")
                                    .mime(Mime::Image(MimeImage::Jpeg))
                                    .alt("A black pen with a white logo on the front.")
                                    .build(),
                            })
                        )],
                    }),
                ),
                (
                    (
                        "specimens".try_into().unwrap(),
                        "pen".try_into().unwrap()
                    ),
                    ItemDataInMarket::V1(ItemDataInMarketV1 {
                        item_name: "Pen".to_string(),
                        store_name: "Specimens".to_string(),
                        tags: vec![Tag::new("pen").unwrap()],
                        attrs: vec![(
                            AttrKeys::default(),
                            ItemAttrSpecificDataInMarket::V1(ItemAttrSpecificDataInMarketV1 {
                                is_in_stock: true,
                                price: vec![(Currency::USD, Price::new(99.5))]
                                    .into_iter()
                                    .collect(),
                                image: MediaData::builder()
                                    .url("https://images.unsplash.com/photo-1519079754742-f83afaef6d35")
                                    .mime(Mime::Image(MimeImage::Jpeg))
                                    .alt("A black pen with a white logo on the front.")
                                    .build(),
                            })
                        )],
                    }),
                ),
                (
                    (
                        "specimens".try_into().unwrap(),
                        "polaroid-onestep-2".try_into().unwrap(),
                    ),
                    ItemDataInMarket::V1(ItemDataInMarketV1 {
                        item_name: "Polaroid OneStep 2".to_string(),
                        store_name: "Specimens".to_string(),
                        tags: vec![Tag::new("camera").unwrap(), Tag::new("polaroid").unwrap()],
                        attrs: vec![(
                            AttrKeys::default(),
                            ItemAttrSpecificDataInMarket::V1(ItemAttrSpecificDataInMarketV1 {
                                is_in_stock: true,
                                price: vec![(Currency::USD, Price::new(99.5))]
                                    .into_iter()
                                    .collect(),
                                image: MediaData::builder()
                                    .url("https://images.unsplash.com/photo-1526170375885-4d8ecf77b99f")
                                    .mime(Mime::Image(MimeImage::Jpeg))
                                    .alt("Polaroid OneStep 2")
                                    .build(),
                            }),
                        )],
                    }),
                ),
                (
                    (
                        "specimens".try_into().unwrap(),
                        "wiggle-stool".try_into().unwrap(),
                    ),
                    ItemDataInMarket::V1(ItemDataInMarketV1 {
                        item_name: "Wiggle Stool".to_string(),
                        store_name: "Specimens".to_string(),
                        tags: vec![Tag::new("furniture").unwrap()],
                        attrs: vec![(
                            AttrKeys::default(),
                            ItemAttrSpecificDataInMarket::V1(ItemAttrSpecificDataInMarketV1 {
                                is_in_stock: true,
                                price: vec![(Currency::USD, Price::new(845.00))]
                                    .into_iter()
                                    .collect(),
                                image: MediaData::builder()
                                    .cid("bafybeifwfqsftwqgqee6e5o44ylydjki6vvbd3gzur3kw3couez7tqjihe/front.jpg")
                                    .mime(Mime::Image(MimeImage::Jpeg))
                                    .alt("The front image of the stool.")
                                    .build(),
                            }),
                        )],
                    }),
                ),
            ],
        ),
    ]
}
