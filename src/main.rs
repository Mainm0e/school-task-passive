use structopt::StructOpt;
use std::fs;
use std::io;

mod base_on_name;
mod base_on_ip;
mod base_on_username;

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
    let result = base_on_name::get_people(full_name).await;

    return_result(result).await;
}

async fn search_ip(ip_address: &str) {
    // Placeholder for searching IP address
    // You can replace this with actual API calls or other implementations
    let result = base_on_ip::get_location(ip_address).await;
    return_result(result).await;
}

async fn search_username(username: &str) {
    // Placeholder for searching username
    // You can replace this with actual API calls or other implementations
    println!("Searching username: {}", username);
    let result = base_on_username::check_username(username).await;
    return_result(result).await;
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
}

async fn return_result(result: Option<String>) {
    if let Some(result) = result {
        println!("{}", result);
        write_result(Some(result)).await.unwrap();
    } else {

        write_result(Some("".to_string())).await.unwrap();
        println!("");
    }
}


async fn write_result(result: Option<String>) -> io::Result<()> {
    // Specify the folder where the result files are stored
    let folder_path = "results/";
    
    // Get a list of files in the folder
    let entries = fs::read_dir(folder_path);
    // if the folder does not exist, create it
    if entries.is_err() {
        fs::create_dir(folder_path)?;
    }
    let entries = fs::read_dir(folder_path)?;

    // Find the latest result file number
    let latest_number = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.file_name().to_str().and_then(|name| {
                    if name.starts_with("result_") && name.ends_with(".txt") {
                        name.trim_start_matches("result_").trim_end_matches(".txt").parse().ok()
                    } else {
                        None
                    }
                })
            })
        })
        .max()
        .unwrap_or(0);

    // Create a new result file with an incremented number
    let new_number = latest_number + 1;
    let new_result_file = format!("result_{}.txt", new_number);
    let new_result_path = format!("{}/{}", folder_path, new_result_file);

    // Write the result to the new result file
    fs::write(new_result_path, result.unwrap_or_default())?;

    Ok(())
}
