use reqwest;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "passive", about = "Passive Recognition Tool")]
struct Opt {
    #[structopt(short = "fn", long = "fn")]
    full_name: Option<String>,

    #[structopt(short = "ip", long = "ip")]
    ip_address: Option<String>,

    #[structopt(short = "u", long = "u")]
    username: Option<String>,

    #[structopt(short = "hp", long = "help")]
    help: bool,

}


#[tokio::main]
async fn main() {
    // Parse command-line arguments
    let opt = Opt::from_args();

    // Choose the appropriate function based on the provided options
    if let Some(full_name) = opt.full_name {
        search_full_name(&full_name).await;
    } else if let Some(ip_address) = opt.ip_address {
        search_ip(&ip_address).await;
    } else if let Some(username) = opt.username {
        search_username(&username).await;
    } else if opt.help {
        search_help().await;
    } else {
        println!("  ");
    }
}

async fn search_full_name(full_name: &str) {
    // Placeholder for searching full name
    // You can replace this with actual API calls or other implementations
    println!("Searching full name: {}", full_name);
}

async fn search_ip(ip_address: &str) {
    // Placeholder for searching IP address
    // You can replace this with actual API calls or other implementations
    println!("Searching IP address: {}", ip_address);
}

async fn search_username(username: &str) {
    // Placeholder for searching username
    // You can replace this with actual API calls or other implementations
    println!("Searching username: {}", username);
}

async fn search_help() {
    // Placeholder for searching help
    // You can replace this with actual API calls or other implementations
    println!("
    Welcome to passive v1.0.0

    OPTIONS:
        -fn         Search with full-name
        -ip         Search with ip address
        -u          Search with username
        
        ");
    // return  help
    
}