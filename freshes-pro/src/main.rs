use rand::Rng;
use regex::Regex;
use reqwest::Client;
use serde_json::json;
use tokio::task;

fn unescape_unicode(s: &str) -> String {
    let re = Regex::new(r"\\u([0-9a-fA-F]{4})").unwrap();
    re.replace_all(s, |caps: &regex::Captures| {
        let code = u32::from_str_radix(&caps[1], 16).unwrap();
        std::char::from_u32(code).unwrap().to_string()
    })
    .into_owned()
}

fn get_random_name() -> String {
    let random_first_name: String = (0..6).map(|_| rand::thread_rng().gen_range('a'..='z')).collect();
    let random_last_name: String = (0..8).map(|_| rand::thread_rng().gen_range('a'..='z')).collect();
    format!("{} {}", random_first_name, random_last_name)
}

fn get_random_phone() -> String {
    let phone_start: String = "056".to_string();
    let random_phone: String = (0..7).map(|_| rand::thread_rng().gen_range(0..10).to_string()).collect();
    format!("{}{}", phone_start, random_phone)
}

async fn submit(id: usize) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let post_id = "10";
    let form_id = "64b800d";
    let random_name = get_random_name();
    let phone_number = get_random_phone();

    let website_domain = "freshes.pro";
    let website_url = format!("https://{}/", website_domain);

    let form_data = json!({
      "action": "elementor_pro_forms_send_form",
      "referrer": website_url,
      "post_id": post_id,
      "form_id": form_id,
      "form_fields[name]": random_name,
      "form_fields[field_8a90fae]": phone_number,
    });

    let form_submit_url = format!("{}/wp-admin/admin-ajax.php", website_url);
    let submit_response = client
        .post(form_submit_url)
        .form(&form_data)
        .send()
        .await?
        .text()
        .await?;

    let response_text = unescape_unicode(&submit_response);
    println!("Task {} - Response: {}", id, response_text);
    Ok(format!("Success - Response: {}", response_text))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = vec![];

    for task_num in 0..10000 {
        tasks.push(task::spawn(async move {
            if let Err(e) = submit(task_num).await {
                eprintln!("Task {} failed: {}", task_num, e);
            }
        }));
    }

    for task in tasks {
        task.await?;
    }

    Ok(())
}
