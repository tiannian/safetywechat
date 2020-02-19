pub mod text;
pub mod image;
pub mod voice;
pub mod video;
pub mod location;
pub mod link;

pub enum MessageType {
    Text(String),
    Image(image::Image),
    Voice(voice::Voice),
    Video(video::Video),
    ShortVideo(video::Video),
    Location(location::Location),
    Link(link::Link),
    Event,
}

