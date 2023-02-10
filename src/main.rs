use reqwest::blocking::Client;
use reqwest::blocking::Response;
use std::io::Read;

const URL_CHECK_INTERNET: &str = "http://www.google.com";
const REPO_KEY_WORDS_FILE: &str = "https://raw.githubusercontent.com/sergiosouzalima/human-resource-words/master/hr-specific-words.txt";

fn check_internet() -> bool {
    let client = Client::new();
    let response = client.get(URL_CHECK_INTERNET).send();
    match response {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}
fn get_words_from_repo() -> (bool, Response) {
    let client = Client::new();
    let response = client
        .get(REPO_KEY_WORDS_FILE)
        .send()
        .unwrap();
    (response.status().is_success(), response)
}

fn main() {
    
    println!("Checking internet connection....");
    // Check if internet connection is available.
    if !check_internet() {
        println!("No internet connection available.");
        return;
    }

    println!("Checking if repository is available at {}....", REPO_KEY_WORDS_FILE);
    // Retrieve key words to search, from the Github repo.
    let (is_success, mut response) = get_words_from_repo();
    if !is_success {
        println!("Failed to retrieve file from Github repository.");
        return;
    }

    // Push key words into an array.
    let mut contents = String::new();
    response.read_to_string(&mut contents).unwrap();
    let mut words = Vec::new();
    for line in contents.lines() {
        let upper_word = line.to_uppercase();
        words.push(upper_word);
    } 

    println!("Showing content from {}:", REPO_KEY_WORDS_FILE);
    for word in words.iter() {
        println!("{}", word);
    }
}
