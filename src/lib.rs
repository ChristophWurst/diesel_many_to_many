#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use models::*;
use schema::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn reset_db(conn: &mut PgConnection) {
    use self::schema::images::dsl::*;
    use self::schema::tags::dsl::*;
    use self::schema::image_tags::dsl::*;

    diesel::delete(image_tags)
        .execute(conn)
        .expect("could not delete image_tags associations");
    diesel::delete(tags)
        .execute(conn)
        .expect("could not delete tags");
    diesel::delete(images)
        .execute(conn)
        .expect("could not delete images");
}

fn insert_test_data(conn: &mut PgConnection) -> (models::Image, models::Image) {
    let new_img1 = NewImage { url: "img1.jpg" };
    let new_img2 = NewImage { url: "img1.jpg" };

    let img1: Image = diesel::insert_into(images::table)
        .values(&new_img1)
        .get_result(conn)
        .expect("Error savig img1");
    let img2: Image = diesel::insert_into(images::table)
        .values(&new_img2)
        .get_result(conn)
        .expect("Error savig img2");

    let cat_tag = NewTag { label: "cat" };
    let cute_tag = NewTag { label: "cute" };
    let tag1: Tag = diesel::insert_into(tags::table)
        .values(&cat_tag)
        .get_result(conn)
        .expect("Error saving cat tag");
    let tag2: Tag = diesel::insert_into(tags::table)
        .values(&cute_tag)
        .get_result(conn)
        .expect("Error saving cute tag");

    // Associate images with tag(s)
    let img1_tag1 = NewImageTag {
        image_id: img1.id,
        tag_id: tag1.id,
    };
    let img1_tag2 = NewImageTag {
        image_id: img1.id,
        tag_id: tag2.id,
    };
    diesel::insert_into(image_tags::table)
        .values(&img1_tag1)
        .execute(conn)
        .expect("Error associationg img1 with tag1");
    diesel::insert_into(image_tags::table)
        .values(&img1_tag2)
        .execute(conn)
        .expect("Error associationg img1 with tag2");

    (img1, img2)
}

fn get_tags_for_image(img: &models::Image, conn: &mut PgConnection) -> Vec<Tag> {
    let image_tag_ids = ImageTag::belonging_to(img).select(image_tags::tag_id);
    tags::table
        .filter(tags::id.eq_any(image_tag_ids))
        .load::<Tag>(conn)
        .expect("could not load tags")
}

pub fn list_tags() {
    let conn = &mut establish_connection();
    reset_db(conn);
    let (img1, img2) = insert_test_data(conn);

    let result = get_tags_for_image(&img1, conn);
    println!("Image 1 has {} tags.", result.len());
    for t in result {
        println!("{}: {}", t.id, t.label);
    }

    let result = get_tags_for_image(&img2, conn);
    println!("Image 2 has {} tags.", result.len());
    for t in result {
        println!("{}: {}", t.id, t.label);
    }
}
