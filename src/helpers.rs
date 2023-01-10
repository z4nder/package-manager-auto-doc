pub async fn extract_copyright_from_github(git_url: &String) -> String {
    let mut git_file = git_url.to_string();
    git_file = git_file.replace("github.com/", "raw.githubusercontent.com/");
    git_file = format!("{}/master/LICENSE", &git_file);

    let mut res = reqwest::get(&git_file)
        .await
        .expect("[ERROR] -> Failed to get current package");

    if res.status() == 404 {
        git_file = git_file.replace("LICENSE", "LICENSE.md");
        res = reqwest::get(&git_file)
            .await
            .expect("[ERROR] -> Failed to get current package");
    }

    if res.status() == 404 {
        git_file = git_file.replace("LICENSE.md", "LICENSE.txt");
        res = reqwest::get(&git_file)
            .await
            .expect("[ERROR] -> Failed to get current package");
    }

    let text_response = res
        .text()
        .await
        .expect("[ERROR] -> Failed to parse to json");

    // TODO Case find on Copyright occurence and net is not Copyright stop search to optimize
    //  Look need lowercase reponse string
    let formated: Vec<&str> = text_response.split("\n").collect();
    let formated: Vec<&str> = formated
        .iter()
        .filter(|&element| element.contains("Copyright"))
        .cloned()
        .collect();

    formated.join(", ")
}
