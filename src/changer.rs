use rand::Rng;
use serde_json::Value;

pub(crate) fn run() -> Result<(), String> {
    Ok(set_wallpaper_image(get_random_picture_url(parse_data(request_data()?)?)?)?)
}


fn request_data() -> Result<Value, String> {
    let request_resp = attohttpc::get("https://www.reddit.com/r/wallpaper/top/.json?t=day").send();


    match request_resp {
        Ok(res) => {
            if !res.is_success() {
                return Err("Reddit request not successful".to_string())
            }

            match res.json::<Value>() {
                Ok(val) => Ok(val),
                _ => Err("Could not parse reddit response json".to_string())
            }
        }
        _ => Err("Could not fetch reddit posts".to_string()),
    }
}

fn parse_data(data: Value) -> Result<Vec<Value>, String> {
    data
        .get("data")
        .ok_or("Could not parse data".to_string())?
        .get("children").expect("Could not get children").as_array()
        .ok_or("Could not parse posts".to_string()).cloned()
}

fn get_random_picture_url(posts: Vec<Value>) -> Result<String, String> {
    let mut img_url: &str = "";

    for i in 0..100 {
        let post_index = rand::thread_rng().gen_range(0..10);
        let url = posts.get(post_index).ok_or("Post on index does not exist".to_string())?
            .get("data").ok_or("Could not get data from post".to_string())?
            .get("url").ok_or("Could not get url from post".to_string())?
            .as_str().ok_or("Could not convert to string".to_string());

        if url.is_ok() && (url.as_ref().unwrap().ends_with(".png") || url.as_ref().unwrap().ends_with(".jpg")) {
            img_url = url.unwrap();
            break;
        }

        if i == 99 {
            return Err("Could not get image after 100 tries.".to_string());
        }
    }

    Ok(img_url.to_string())
}

fn set_wallpaper_image(url: String) -> Result<(), String> {
    wallpape_rs::set_from_url(url.as_ref()).expect("Could not set wallpaper");

    Ok(())
}