use serde::Deserialize;
use serde::Serialize;

/// Error types for this crate
pub mod error;

/// Data structures for ser/de of account-related resources
pub mod account;
/// Data structures for ser/de of attachment-related resources
pub mod attachment;
/// Data structures for ser/de of card-related resources
pub mod card;
/// Data structures for ser/de of context-related resources
pub mod context;
/// Data structures for ser/de of streaming events
pub mod event;
/// Data structures for ser/de of filter-related resources
pub mod filter;
/// Type-safe ID values
pub mod ids;
pub use ids::*;
/// Data structures for ser/de of instance-related resources
pub mod instance;
/// Data structures for ser/de of list-related resources
pub mod list;
/// Data structures for ser/de of mention-related resources
pub mod mention;
/// Data structures for ser/de of notification-related resources
pub mod notification;
/// Data structures for ser/de of push-subscription-related resources
pub mod push;
/// Data structures for ser/de of relationship-related resources
pub mod relationship;
/// Data structures for ser/de of report-related resources
pub mod report;
/// Data structures for ser/de of search-related resources
pub mod search_result;
/// Data structures for ser/de of status-related resources
pub mod status;
/// Data structure for ser/de visibility
pub mod visibility;

/// Admin API data structures, some of which have the same names as non-admin data structures
pub mod admin;

/// An empty JSON object.
#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Empty {}

/// The purpose of this module is to alleviate imports of many common
/// structs by adding a glob import to the top of mastodon heavy
/// modules:
pub mod prelude {
    pub use super::{
        account::{Account, Source},
        admin::{Ip, IpBlock, IpBlockSeverity},
        attachment::{Attachment, MediaType},
        card::Card,
        context::Context,
        event::Event,
        filter::{Filter, FilterContext},
        ids::*,
        instance::*,
        list::List,
        mention::Mention,
        notification::Notification,
        push::Subscription,
        relationship::Relationship,
        report::Report,
        search_result::SearchResult,
        status::{Application, Emoji, Status},
        visibility::Visibility,
        Empty,
    };
}
