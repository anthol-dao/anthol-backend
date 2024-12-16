mod camera;
mod cap;
mod pen;
mod stool;

use anthol_store::item::Item;

pub fn get() -> Vec<Item> {
    vec![camera::get(), cap::get(), pen::get(), stool::get()]
}
