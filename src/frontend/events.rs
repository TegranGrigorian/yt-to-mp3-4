pub enum AppEvent {
    DownloadStarted(String), // URL of the download
    DownloadCompleted(String), // Path to the downloaded file
    ErrorOccurred(String), // Error message
}
