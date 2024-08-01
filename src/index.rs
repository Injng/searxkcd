use dirs;
use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
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
pub async fn get_comic(id: u64) -> Comic {
    // check if id is valid
    let latest_id: u64 = get_latest_id().await;
    if id == 0 || id > latest_id {
        panic!("Invalid comic id");
    }

    // otherwise, get basic comic info from XKCD api
    let api_url: String = format!("https://xkcd.com/{}/info.0.json", id);
    let body: String = reqwest::get(&api_url).await.unwrap().text().await.unwrap();

    // deserialize JSON into a Comic
    let comic: Value = serde_json::from_str(&body).unwrap();
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

    // find index of transcript heading
    let transcript_idx: usize = body.find("Transcript").expect("Transcript not found");
    let after_transcript: String = body[transcript_idx..].to_string();
    let after_idx: usize = after_transcript.find("<dl>").expect("dl not found");
    let before_idx: usize = after_transcript
        .find("<span id=\"Discussion\">")
        .expect("Span not found");
    let transcript_snippet: String = after_transcript[after_idx..before_idx].to_string();

    // parse transcript snippet
    let document = Html::parse_fragment(&transcript_snippet);
    let dl_selector = Selector::parse("dl").unwrap();
    let dd_selector = Selector::parse("dd").unwrap();

    // get transcript items and write to String
    let transcript_html = document.select(&dl_selector);
    let mut transcript: String = "".to_string();
    for items in transcript_html {
        for item in items.select(&dd_selector) {
            transcript += &item.text().collect::<String>();
            transcript += " ";
        }
    }

    // make and return the Comic
    Comic::new(comic_id, comic_title, comic_alt, comic_url, transcript)
}

/// Update index
pub async fn update_index(comics: &mut Vec<Comic>) {
    let latest_id: u64 = get_latest_id().await;
    let latest_comic: Comic = get_comic(latest_id).await;
    println!("Latest comic: {}", latest_comic.transcript);
    comics.push(latest_comic);

    // update index file
    let home: String = get_home();
    let index_path: String = format!("{}/.local/share/searxkcd/index.json", home);
    let index_file = Path::new(&index_path);
    let index_json: String = serde_json::to_string(&comics).unwrap();
    fs::write(index_file, index_json).expect("Failed to create index file");
}
