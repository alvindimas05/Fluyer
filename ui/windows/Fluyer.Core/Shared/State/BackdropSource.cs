namespace Fluyer.Core.State;

/// <summary>
/// What the animated backdrop renders: the real cover, or Rust's palette-block square.
/// Port of <c>BackdropSource</c> in ui/macos/Sources/Shared/State/SettingsState.swift.
/// </summary>
public enum BackdropSource
{
    Artwork,
    Blocks,
}
