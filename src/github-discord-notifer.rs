use github_flows::{listen_to_event, octocrab::models::IssueEvent, EventPayload};
use serde_json::Value;
use slack_flows::send_message_to_channel;
use tokio::*;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let owner = "jaykchen";
    let repo = "vitesse-lite";
    let label_watch_list = vec![
        "good first issue".to_string(),
        "help wanted".to_string(),
        "LFX mentorship".to_string(),
    ];

    listen_to_event(owner, repo, vec!["issues", "issue_comment"], |payload| {
        handler(payload, &label_watch_list)
    })
    .await;

    Ok(())
}

async fn handler(payload: EventPayload, label_watch_list: &Vec<String>) {
    let lowercase_list = label_watch_list
        .into_iter()
        .map(|word| word.to_ascii_lowercase())
        .collect::<Vec<String>>();

    match payload {
        EventPayload::IssuesEvent(e) => {
            let issue = e.issue;
            let issue_title = issue.title;
            let issue_url = issue.url;
            let user = issue.user.login;
            let labels = issue.labels;

            for label in labels {
                let label_name = label.name.to_lowercase();
                if lowercase_list.contains(&label_name) {
                    let body = format!(
                        r#"Issue: {issue_title} by {user} 
                    {issue_url}"#
                    );
                    send_message_to_channel("ik8", "general", body);
                    return;
                }
            }
        }

        EventPayload::UnknownEvent(e) => {
            let payload = e.to_string();
            let val: Value = serde_json::from_str(&payload).unwrap();

            match val.get("issue") {
                None => (),

                Some(issue) => {
                    let issue_title = &issue["title"].as_str().unwrap();
                    let issue_url = &issue["url"].as_str().unwrap();
                    let user = &issue["user"]["login"].as_str().unwrap();

                    match issue["labels"].as_array() {
                        None => (),

                        Some(labels) => {
                            for label in labels {
                                let label_name = label["name"]
                                    .as_str()
                                    .expect("no label found")
                                    .to_lowercase();

                                if lowercase_list.contains(&label_name) {
                                    let body = format!(
                                        r#"Issue: {issue_title} by {user} 
                            {issue_url}"#
                                    );
                                    send_message_to_channel("ik8", "general", body);
                                    return;
                                }
                            }
                        }
                    }
                }
            };
        }

        _ => (),
    }
}
