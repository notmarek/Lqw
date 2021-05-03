use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: i64,
    #[serde(rename = "type")]
    pub tag_type: String,
    pub name: String,
    pub url: String,
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Title {
    pub english: String,
    pub japanese: String,
    pub pretty: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub t: String,
    pub w: i32,
    pub h: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Images {
    pub pages: Vec<Image>,
    pub cover: Image,
    pub thumbnail: Image,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gallery {
    pub id: i64,
    pub media_id: String,
    pub title: Title,
    pub images: Images,
    pub scanlator: String,
    pub upload_date: i64,
    pub tags: Vec<Tag>,
    pub num_pages: i64,
    pub num_favorites: i64,
}

impl Gallery {
    pub async fn get(id: i64) -> Result<Self, reqwest::Error> {
        let gallery = reqwest::get(&format!("https://nhentai.net/api/gallery/{}", id))
            .await?
            .json::<Self>()
            .await?;
        Ok(gallery)
    }
    pub fn get_tags(self) -> String {
        let mut f: String = String::new();
        for tag in self.tags {
            if tag.tag_type == "tag".to_string() {
                if f == String::new() {
                    f = tag.name;
                } else {
                    f = f + ", " + tag.name.as_str();
                }
            }
        }
        f
    }
}
