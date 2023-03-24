use github_flows::{listen_to_event, octocrab::models::IssueEvent, EventPayload};
use slack_flows::send_message_to_channel;
use tokio::*;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let login = "jaykchen";
    let owner = "jaykchen";
    let repo = "vitesse-lite";
    let label_watch_list = vec![
        "good first issue".to_string(),
        "help wanted".to_string(),
        "LFX mentorship".to_string(),
    ];

    listen_to_event(
        login,
        owner,
        repo,
        vec!["issues", "issue_comment"],
        |payload| handler(payload, &label_watch_list),
    )
    .await;

    Ok(())
}

async fn handler(payload: EventPayload, label_watch_list: &Vec<String>) {
    let lowercase_list = label_watch_list
        .into_iter()
        .map(|word| word.to_ascii_lowercase())
        .collect::<Vec<String>>();

    let mut issue = None;

    match payload {
        EventPayload::IssuesEvent(e) => {
            issue = Some(e.issue);
        }

        EventPayload::IssueCommentEvent(e) => {
            issue = Some(e.issue);
        }

        _ => (),
    }

    if let Some(issue) = issue {
        let issue_title = issue.title;
        let issue_url = issue.html_url;
        let user = issue.user.login;
        let labels = issue.labels;

        for label in labels {
            let label_name = label.name.to_lowercase();
            if lowercase_list.contains(&label_name) {
                let body = format!(
                    r#"A new issue that needs your help: {issue_title} by {user} 
                    {issue_url}"#
                );
                send_message_to_channel("ik8", "general", body);
                return;
            }
        }
    }
}
