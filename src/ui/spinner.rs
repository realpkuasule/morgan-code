use indicatif::{ProgressBar, ProgressStyle};

pub struct Spinner {
    pb: ProgressBar,
}

impl Spinner {
    pub fn new(message: impl Into<String>) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap()
        );
        pb.set_message(message.into());
        pb.enable_steady_tick(std::time::Duration::from_millis(100));

        Self { pb }
    }

    pub fn set_message(&self, message: impl Into<String>) {
        self.pb.set_message(message.into());
    }

    pub fn finish(&self, message: impl Into<String>) {
        self.pb.finish_with_message(message.into());
    }

    pub fn finish_and_clear(&self) {
        self.pb.finish_and_clear();
    }
}
