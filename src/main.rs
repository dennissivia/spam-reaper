use octokit;
use std::env;


fn delete_review_comments(token: &String, username: &str){
    let comments = octokit::get_all_review_comments(&token, String::from("integrations/slack")).expect("API Failed");
    println!("found total of {} comments", comments.len());
    let suspicious = comments.iter().filter(|c| c.user.login == username);

    for comment in suspicious {
        println!("found comment from {}", username);
        println!("comment: {}", comment.body )
    }
}

fn delete_issue_comments(token: &String, username: &str){
    let comments = octokit::get_all_review_comments(&token, String::from("integrations/slack")).expect("API Failed");
    println!("found total of {} comments", comments.len());
    let suspicious = comments.iter().filter(|c| c.user.login == username);

    for comment in suspicious {
        println!("found comment from {}", username);
        println!("comment: {}", comment.body )
    }
}

fn main() {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is required");
    let username = "alkanalkn63";
    delete_review_comments(&token, username);
    delete_issue_comments(&token, username);
}
