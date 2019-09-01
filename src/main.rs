use octokit;
use std::env;
use std::thread;
use std::time::Duration;

fn delete_review_comments(token: &String, nwo: &String, username: &str) {
    let comments = octokit::get_all_review_comments(&token, &nwo).expect("API Failed");
    println!("found total of {} comments", comments.len());
    let suspicious = comments.iter().filter(|c| c.user.login == username);

    for comment in suspicious {
        println!("found comment from {}", username);
        println!("comment: {}", comment.body)
    }
}

fn delete_issue_comments(token: &String, nwo: &String, username: &str) {
    let comments = octokit::get_all_review_comments(&token, &nwo).expect("API Failed");
    println!("found total of {} comments", comments.len());
    let suspicious = comments.iter().filter(|c| c.user.login == username);

    for comment in suspicious {
        println!("found comment from {}", username);
        println!("comment: {}", comment.body)
    }
}

fn jwt_me() {
    //    let key_path = "./octorust-dev.2019-07-24.private-key.pem";
    let key_path = env::var("KEY_FILE").expect("KEY_FILE is not set");
    let app_id = env::var("APP_ID").expect("APP_ID is required");
    let result = octokit::generate_jwt(&key_path, &app_id);
    match result {
        Ok(data) => {
            println!("Generated JWT\n{}", data);
            let maybe_app = octokit::get_app(&data);
            match maybe_app {
                Some(app) => println!("Current app is: {:?}", app),
                None => println!("failed to load app"),
            }
        }
        Err(err) => {
            println!("All failed:\n{}", err);
        }
    }
}

fn spam_ping_pong(token: &String, nwo: &String, body: &str) {
    let issue_number = 2;
    let result = octokit::create_issue_comment(token, issue_number, &nwo, String::from(body));
    println!("create result: {:?}", result);
    let comment = result.expect("Failed to create comment");

    println!(
        "Comment created with id: {}, deleting in 4 seconds",
        comment.id
    );
    thread::sleep(Duration::from_millis(4000));

    octokit::delete_issue_comment(token, &nwo, comment.id);
}

fn main() {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is required");
    let nwo = String::from("octorust/playground");
    let spammy_user = "alkanalkn63";
    let spammy_body = "💜";
    // let filterKnownUsers = |comment: octokit::IssueComment| comment.user.login == spammy_user;
    // let filterSpammyContent = |comment: octokit::IssueComment| comment.body == spammy_body;

    spam_ping_pong(&token, &nwo, spammy_body);
    delete_review_comments(&token, &nwo, spammy_user);
    delete_issue_comments(&token, &nwo, spammy_user);
    jwt_me();
}
