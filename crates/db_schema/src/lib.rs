#[cfg(feature = "full")]
#[macro_use]
extern crate diesel;

#[cfg(feature = "full")]
pub mod ptt_comment;
#[cfg(feature = "full")]
pub mod ptt_commuity;
#[cfg(feature = "full")]
pub mod ptt_post;
#[cfg(feature = "full")]
pub mod ptt_post_comment;

#[cfg(feature = "full")]
pub use crate::ptt_comment::PttComment;
#[cfg(feature = "full")]
pub use crate::ptt_community::PttCommunity;
#[cfg(feature = "full")]
pub use crate::ptt_post::PttPost;
#[cfg(feature = "full")]
pub use crate::ptt_post_comment::PttPostComment;
