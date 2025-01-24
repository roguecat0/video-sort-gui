use iced_video_player::{Video, VideoPlayer};

fn main() -> iced::Result {
    iced::run("Video Player", (), App::view)
}

struct App {
    video: Video,
}

impl Default for App {
    fn default() -> Self {
        let path = std::path::PathBuf::from(
            r#"C:\Users\tevon\Programming\rust\video_sort_gui\media\a\3b2770df0c408136382ef33df36eb822.ogv"#,
        );
        //.parent()
        //.unwrap()
        //.join("../media/test3.mp4")
        //.canonicalize()
        //.unwrap();

        App {
            video: Video::new(&url::Url::from_file_path(path).unwrap()).unwrap(),
        }
    }
}

impl App {
    fn view(&self) -> iced::Element<()> {
        VideoPlayer::new(&self.video).into()
    }
}
