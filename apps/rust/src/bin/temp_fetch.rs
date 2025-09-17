use rust_lib::fetch_with_proxy::fetch_with_proxy;
use tokio; // Add tokio for async main

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Test HTML scraping approach
  let html_url = "
https://komiku.org/?post_type=manga&s=naruto";
  println!("Fetching HTML URL: {}", html_url);
  let html_response = fetch_with_proxy(html_url).await?;
  println!("HTML Response length: {}", html_response.data.len());
  println!("First 500 chars: {}", &html_response.data[..(500).min(html_response.data.len())]);

  // Test API approach for manga listing
  let api_manga_url = "
https://api.komiku.org/manga/page/1/?tipe=manga
";
  println!("\nFetching API Manga URL: {}", api_manga_url);
  match fetch_with_proxy(api_manga_url).await {
    Ok(api_response) => {
      println!("API Manga Response length: {}", api_response.data.len());
      println!("First 500 chars: {}", &api_response.data[..(500).min(api_response.data.len())]);
    }
    Err(e) => {
      println!("API Manga fetch failed: {:?}", e);
    }
  }

  // Test API approach for manhua listing
  let api_manhua_url = "
https://api.komiku.org/manga/page/1/?tipe=manhua
";
  println!("\nFetching API Manhua URL: {}", api_manhua_url);
  match fetch_with_proxy(api_manhua_url).await {
    Ok(api_response) => {
      println!("API Manhua Response length: {}", api_response.data.len());
      println!("First 500 chars: {}", &api_response.data[..(500).min(api_response.data.len())]);
    }
    Err(e) => {
      println!("API Manhua fetch failed: {:?}", e);
    }
  }

  // Test API approach for manhwa listing
  let api_manhwa_url = "
https://api.komiku.org/manga/page/1/?tipe=manhwa
";
  println!("\nFetching API Manhwa URL: {}", api_manhwa_url);
  match fetch_with_proxy(api_manhwa_url).await {
    Ok(api_response) => {
      println!("API Manhwa Response length: {}", api_response.data.len());
      println!("First 500 chars: {}", &api_response.data[..(500).min(api_response.data.len())]);
    }
    Err(e) => {
      println!("API Manhwa fetch failed: {:?}", e);
    }
  }

  Ok(())
}
