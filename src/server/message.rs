use chrono::Utc;
use rss::Item;

#[derive(Clone, Debug)]
pub struct UpdateMessage {
    pub item: Item,
    pub targets: Vec<usize>,
}

#[derive(Clone, Debug)]
pub enum StatMessage {
    TargetSuccess {
        id: usize,
    },
    TargetFailure {
        id: usize,
    },
    SubscriptionSuccess {
        id: usize,
        time: chrono::DateTime<Utc>,
    },
    SubscriptionFailure {
        id: usize,
    },
}
