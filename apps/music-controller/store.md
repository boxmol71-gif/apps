Control music playback and the phone's music-stream volume from the watch.

The standalone Android **Music Companion** app shows the current track and its notification artwork on the phone. To protect privacy, only generic playback state and volume are sent to the watch; the title, artist and artwork never cross the watch's Bluetooth link. Anyone close enough to read that link can therefore see only whether media is playing and the volume level.

## Requirements

- The [patched firmware](/firmware) with the channel key and the [App Loader](/firmware).
- The standalone **Android Music Companion** in the `music-companion/` project delivered with this app, with its watch bridge enabled and notification-listener access granted.
