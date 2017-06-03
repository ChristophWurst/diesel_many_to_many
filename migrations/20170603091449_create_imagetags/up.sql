CREATE TABLE image_tags (
  id SERIAL PRIMARY KEY,
  image_id SERIAL references images(id),
  tag_id SERIAL references tags(id)
)
