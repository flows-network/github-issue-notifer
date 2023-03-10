use github_flows::{listen_to_event, EventPayload};
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

    listen_to_event(owner, repo, vec!["issue"], |payload| {
        handler(owner, repo, payload, &label_watch_list)
    })
    .await;

    Ok(())
}

async fn handler(owner: &str, repo: &str, payload: EventPayload, label_watch_list: &Vec<String>) {
    let lowercase_list = label_watch_list
        .into_iter()
        .map(|word| word.to_ascii_lowercase())
        .collect::<Vec<String>>();

    match payload {
        EventPayload::IssuesEvent(e) => {
            let issue = e.issue;
            let user = issue.user.login;
            let labels = issue.labels;

            for label in labels {
                let label_name = label.name.to_lowercase();
                if lowercase_list.contains(&label_name) {
                    let body = format!("new contributor {user} submitted an issue");
                    send_message_to_channel("ik8", "general", body);
                    return;
                }
            }
        }

        _ => (),
    }
}
