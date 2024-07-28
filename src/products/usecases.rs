use std::collections::HashMap;

use super::entities;

pub fn find_one_product(product_id: i32) -> entities::Result<entities::Product, String> {
    let mut products_map: HashMap<i32, entities::Product> = HashMap::new();

    products_map.insert(
        1,
        entities::Product {
            id: 1,
            title: String::from("Brown"),
            description: String::from("Brown is line friends family !"),
        },
    );

    products_map.insert(
        2,
        entities::Product {
            id: 2,
            title: String::from("Cony"),
            description: String::from("Cony is line friends family !"),
        },
    );

    let result = products_map.get(&product_id);
    match result {
        Some(p) => {
            return entities::Result::Ok(entities::Product {
                id: p.id,
                title: p.title.clone(),
                description: p.description.clone(),
            });
        }
        None => entities::Result::Err(format!("product_id {} not found", product_id)),
    }
}
