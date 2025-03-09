use iso8601_duration::Duration;
use leptos::leptos_dom::logging::console_log;

const API_KEY: &str = dotenvy_macro::dotenv!("YOUTUBE_API_KEY");

fn get_video_id(link: &str) -> Result<String, String> {
    let regex = regex::Regex::new(
        r"^.*((youtu.be\/)|(v\/)|(\/u\/\w\/)|(embed\/)|(watch\?))\??(v=)?([^#&?]*).*",
    )
    .unwrap();
    let videoid = regex.captures(link);
    match videoid {
        Some(videoid) => {
            if videoid.get(8).is_none() || videoid.get(8).unwrap().as_str().len() != 11 {
                return Err("Invalid video id field".to_string());
            }
            Ok(videoid.get(8).unwrap().as_str().to_string())
        }
        None => Err("Invalid youtube link no match regex".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_share() {
        let link = "https://youtu.be/vVfU08SocHc?si=pgpjbUuaQL-eGiCH";
        let result = get_video_id(link).unwrap();
        assert_eq!(result, "vVfU08SocHc");
    }

    #[test]
    fn video_url_bar() {
        let link = "https://www.youtube.com/watch?v=vVfU08SocHc&ab_channel=Theo-t3․gg";
        let result = get_video_id(link).unwrap();
        assert_eq!(result, "vVfU08SocHc");
    }
}

pub fn get_thumbnail(link: &str) -> Result<String, String> {
    let mut thumbnail = String::from("https://img.youtube.com/vi/");
    thumbnail.push_str(&get_video_id(link)?);
    thumbnail.push_str("/hqdefault.jpg");
    Ok(thumbnail)
}

/*
yoputube api response example
{
 "kind": "youtube#videoListResponse",
 "etag": "\"XlbeM5oNbUofJuiuGi6IkumnZR8/ny1S4th-ku477VARrY_U4tIqcTw\"",
 "items": [
  {

   "id": "9bZkp7q19f0",
   "kind": "youtube#video",
   "etag": "\"XlbeM5oNbUofJuiuGi6IkumnZR8/HN8ILnw-DBXyCcTsc7JG0z51BGg\"",
   "contentDetails": {
    "duration": "PT4M13S",
    "dimension": "2d",
    "definition": "hd",
    "caption": "false",
    "licensedContent": true,
    "regionRestriction": {
     "blocked": [
      "DE"
     ]
    }
   }
  }
 ]
}
*/

#[derive(serde::Deserialize)]
struct ApiResponse {
    items: Vec<Items>,
}

#[derive(serde::Deserialize)]
struct Items {
    #[serde(rename = "contentDetails")]
    content_details: ContentDetails,
    snippet: Snippet,
}

#[derive(serde::Deserialize)]
struct ContentDetails {
    duration: String,
}

#[derive(serde::Deserialize)]
struct Snippet {
    title: String,
}

pub struct VideoMetadata {
    pub title: String,
    pub duration: u32,
}

pub async fn get_metadata(link: &str) -> Result<VideoMetadata, String> {
    let video_id = get_video_id(link)?;
    // let url = format!("https://www.googleapis.com/youtube/v3/videos?part=contentDetails,&id={}&key={}", video_id, API_KEY);
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?part=contentDetails,snippet&id={}&key={}",
        video_id, API_KEY
    );
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
    console_log(format!("response: {:?}", response).as_str());
    let json: ApiResponse = response.json().await.map_err(|e| e.to_string())?;
    let duration = json.items[0].content_details.duration.as_str();
    let duration = Duration::parse(duration).map_err(|e| format!("{:?}", e))?;
    let metadata = VideoMetadata {
        title: json.items[0].snippet.title.clone(),
        duration: duration.num_seconds().unwrap() as u32,
    };
    Ok(metadata)
}
