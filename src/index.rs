use dirs;
use reqwest;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub struct Comic {
    id: u64,
    title: String,
    alt: String,
    img: String,
    transcript: String,
}

impl Comic {
    fn new(id: u64, title: String, alt: String, img: String, transcript: String) -> Comic {
        Comic {
            id,
            title,
            alt,
            img,
            transcript,
        }
    }

    /// Downloads the comic image saving it to the proper directory, and returning the path
    pub async fn download_img(&self) -> String {
        // setup paths
        let home = get_home();
        let path_str = format!("{}/.local/share/searxkcd/images/{}.png", home, self.id.to_string());
        let img_path = Path::new(&path_str);

        // get the image
        let img = reqwest::get(&self.img)
            .await.unwrap()
            .bytes()
            .await.unwrap();
        fs::write(img_path, img)
            .expect("Failed to write image");

        path_str
    }
}

/// Returns a String representation of the home directory
pub fn get_home() -> String {
    let home_path = dirs::home_dir()
        .expect("Failed to get home directory");
    let home = home_path.to_str()
        .expect("Failed to convert home directory to string");
    home.to_string()
}

/// Initializes the index by creating the necessary directories
pub fn init_index() {
    let home = get_home();
    fs::create_dir_all(format!("{}/.local/share/searxkcd/images", home))
        .expect("Failed to create index directory");
}

// async fn update_index(comics: &mut Vec<Comic>) {
pub async fn update_index(comics: &mut Vec<Comic>) {
    // get id of latest comic
    let body = reqwest::get("https://xkcd.com/info.0.json")
    .await.unwrap()
    .text()
    .await.unwrap();

    // deserialize JSON into a Comic
    let latest: Value = serde_json::from_str(&body).unwrap();
    let latest_id = latest["num"].as_u64().unwrap();
    let latest_url = latest["img"].as_str().unwrap().to_string();
    let latest_title = latest["title"].as_str().unwrap().to_string();
    let latest_alt = latest["alt"].as_str().unwrap().to_string();
    let latest_comic: Comic = Comic::new(latest_id, latest_title, latest_alt, latest_url, "".to_string());

    comics.push(latest_comic);
}
