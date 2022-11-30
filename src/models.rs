use super::schema::images;
use super::schema::image_tags;
use super::schema::tags;

#[derive(Identifiable, Queryable)]
pub struct Image {
    pub id: i32,
    pub url: String,
}

#[derive(Insertable)]
#[diesel(table_name = images)]
pub struct NewImage<'a> {
    pub url: &'a str,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(Image))]
#[diesel(belongs_to(Tag))]
pub struct ImageTag {
    pub id: i32,
    pub image_id: i32,
    pub tag_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = image_tags)]
pub struct NewImageTag {
    pub image_id: i32,
    pub tag_id: i32,
}

#[derive(Identifiable, Queryable)]
pub struct Tag {
    pub id: i32,
    pub label: String,
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag<'a> {
    pub label: &'a str,
}
