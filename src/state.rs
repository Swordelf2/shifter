/// Bevy State of the app
///
/// # Transitions
/// Loading ->
/// * Game: upon completion of loading of all assets
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    /// The app is loading assets, loading screen is shown
    Loading,
    /// Actual gameplay
    Game,
}
