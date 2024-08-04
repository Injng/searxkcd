use dirs;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize)]
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
        let path_str = format!(
            "{}/.local/share/searxkcd/images/{}.png",
            home,
            self.id.to_string()
        );
        let img_path = Path::new(&path_str);

        // get the image
        let img = reqwest::get(&self.img)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        fs::write(img_path, img).expect("Failed to write image");

        path_str
    }
}

/// Returns a String representation of the home directory
pub fn get_home() -> String {
    let home_path = dirs::home_dir().expect("Failed to get home directory");
    let home = home_path
        .to_str()
        .expect("Failed to convert home directory to string");
    home.to_string()
}

/// Initializes the index by creating the necessary directories
pub fn init_index() {
    // get home directory of system
    let home = get_home();

    // create index directory
    fs::create_dir_all(format!("{}/.local/share/searxkcd/images", home))
        .expect("Failed to create index directory");

    // check if index file exists; if not, create one
    let index_path = format!("{}/.local/share/searxkcd/index.json", home);
    let index_file = Path::new(&index_path);
    if !index_file.exists() {
        let index: Vec<Comic> = Vec::new();
        let index_json = serde_json::to_string(&index).unwrap();
        fs::write(index_file, index_json).expect("Failed to create index file");
    }

    // check if error file exists; if not, create one
    let error_path = format!("{}/.local/share/searxkcd/error.json", home);
    let error_file = Path::new(&error_path);
    if !error_file.exists() {
        let error: Vec<u64> = Vec::new();
        let error_json = serde_json::to_string(&error).unwrap();
        fs::write(error_file, error_json).expect("Failed to create error file");
    }
}

/// Gets the id of the latest comic
async fn get_latest_id() -> u64 {
    let body = reqwest::get("https://xkcd.com/info.0.json")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let latest: Value = serde_json::from_str(&body).unwrap();
    let latest_id = latest["num"].as_u64().unwrap();
    latest_id
}

/// Gets the comic specified by the id
pub async fn get_comic(id: u64) -> Result<Comic, &'static str> {
    // create reqwest client and target URL
    let client = reqwest::Client::new();
    let api_url: String = format!("https://xkcd.com/{}/info.0.json", id);

    // get comic info from XKCD api, retrying when necessary
    let mut body: String = "".to_string();
    let mut is_error = true;
    for _ in 0..3 {
        body = match client.get(&api_url).send().await {
            Ok(response) => response.text().await.unwrap(),
            Err(_) => continue,
        };
        is_error = false;
        break;
    }

    // check if failed
    if is_error {
        println!("Error: failed retrieving info for #{}", id);
        return Err("Error: failed retrieving comic");
    }

    // deserialize JSON into a Comic
    let comic: Value = match serde_json::from_str(&body) {
        Ok(comic) => comic,
        Err(_) => {
            println!("Error: failed deserializing info for #{}", id);
            return Err("Error: failed deserializing comic");
        }
    };
    let comic_id: u64 = comic["num"].as_u64().unwrap();
    let comic_url: String = comic["img"].as_str().unwrap().to_string();
    let comic_title: String = comic["title"].as_str().unwrap().to_string();
    let comic_alt: String = comic["alt"].as_str().unwrap().to_string();

    // get transcript from explainxkcd
    let explain_url: String = format!("https://www.explainxkcd.com/wiki/index.php/{}", comic_id);
    let body: String = reqwest::get(&explain_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // get transcript snippet from html
    let mut transcript: String = "".to_string();
    let mut failed = false;
    let after_idx: usize = match body.find("id=\"Transcript\"") {
        Some(idx) => idx,
        None => {
            failed = true;
            0
        }
    };
    let before_idx: usize = match body.find("id=\"Discussion\"") {
        Some(idx) => idx,
        None => {
            failed = true;
            0
        }
    };
    if !failed {
        transcript = body[after_idx..before_idx].to_string();
    }

    // make and return the Comic
    Ok(Comic::new(
        comic_id,
        comic_title,
        comic_alt,
        comic_url,
        transcript,
    ))
}

/// Update index
pub async fn update_index() -> Vec<Comic> {
    // get path to index and error files
    let home: String = get_home();
    let index_path: String = format!("{}/.local/share/searxkcd/index.json", home);
    let index_file = Path::new(&index_path);
    let error_path: String = format!("{}/.local/share/searxkcd/error.json", home);
    let error_file = Path::new(&error_path);

    // read json into vector
    let index_json: String = fs::read_to_string(index_file).expect("Failed to read index file");
    let mut index: Vec<Comic> = serde_json::from_str(&index_json).unwrap();
    let error_json: String = fs::read_to_string(error_file).expect("Failed to read error file");
    let errors: Vec<u64> = serde_json::from_str(&error_json).unwrap();

    // retry failed comics
    let mut new_errors: Vec<u64> = Vec::new();
    for id in errors {
        let comic: Comic = match get_comic(id).await {
            Ok(comic) => comic,
            Err(_) => {
                new_errors.push(id);
                continue;
            }
        };
        index.push(comic);
    }

    // get last id in index file; if no comics in index file, set to zero
    let last_id: u64 = (index.len() + new_errors.len()) as u64;
    let next_id: u64 = last_id + 1;

    // get comics that are not in index file
    for id in next_id..(get_latest_id().await + 1) {
        let comic: Comic = match get_comic(id).await {
            Ok(comic) => comic,
            Err(_) => {
                new_errors.push(id);
                continue;
            }
        };
        index.push(comic);
    }

    // update index and error files
    let index_json: String = serde_json::to_string(&index).unwrap();
    fs::write(index_file, index_json).expect("Failed to create index file");
    let error_json: String = serde_json::to_string(&new_errors).unwrap();
    fs::write(error_path, error_json).expect("Failed to create error file");

    index
}
