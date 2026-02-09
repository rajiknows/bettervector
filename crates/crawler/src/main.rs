struct CrawlConfig {
    pub max_depth: usize,
    pub concurrent_reqwests: usize,
    pub user_agent: bool,
    pub timeout: usize,
}

struct CrawlJob {}

struct CrawledDocument {
    pub hash: f64,
    pub url: String,
    pub content: Vec<u8>,
    pub headers: Vec<Header>,
    pub status_code: u16,
    pub fetch_timestamp: f64,
}

struct Header {}

fn main() {
    println!("Hello, world!");
}
