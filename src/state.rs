/// Bevy State of the app
///
/// # Transitions
/// Loading ->
/// * set(Menu): upon completion of loading of all assets
///
/// Menu ->
/// * push(Game): upon clicking on a menu item
///
/// Game ->
/// * pop() -> Menu: upon game over
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    /// The app is loading assets, loading screen is shown
    Loading,
    /// Main menu
    Menu,
    /// Actual gameplay
    Game,
}
