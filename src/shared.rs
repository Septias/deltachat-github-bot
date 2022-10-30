use serde::{Deserialize, Serialize};

use self::{issue::IssueEvent, pr::PREvent};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub login: String,
}

#[derive(Debug)]
pub enum WebhookEvent {
    Issue(IssueEvent),
    PR(PREvent),
}

impl WebhookEvent {
    pub fn event_type(&self) -> &str {
        match self {
            WebhookEvent::Issue(_) => "issue",
            WebhookEvent::PR(_) => "PR",
        }
    }
}
pub mod issue {
    use serde::{Deserialize, Serialize};

    use super::User;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "lowercase")]
    pub enum IssueAction {
        Opened,
        Edited,
        Deleted,
        Pinned,
        Unpinned,
        Closed,
        Reopened,
        Assigned,
        Unassigned,
        Labeled,
        Unlabeled,
        Locked,
        Unlocked,
        Transferred,
        Milestoned,
        Demilestoned,
    }
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct IssueEvent {
        pub action: IssueAction,
        pub sender: User,
    }
}

pub mod pr {
    use serde::{Deserialize, Serialize};

    use super::User;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "lowercase")]
    pub enum PRAction {
        Opened,
        Edited,
        Closed,
        Reopened,
        Assigned,
        Unassigned,
        ReviewRequested,
        ReviewRequestRemoved,
        Labeled,
        Unlabeled,
        Synchronized,
    }
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct PREvent {
        pub action: PRAction,
        pub sender: User,
    }
}

pub struct Repository {
    id: String,
    name: String,
}