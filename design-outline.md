# A rough outline of the design
There will be a "backend" component that manages the music library and syncing playback between clients, and a "frontend" component that drives the GUI for desktop and mobile. The Qt framework will be used for the desktop frontend.

The primary use case is this: configure a library of music tracks, customizing organization/layout, metadata, cover art, playback, etc, and sync it across multiple devices.



# Are there similar projects out there?
Some quick searching led me to a project called "funkwhale". It's more a federated streaming platform, than a personal music library organizer; it requires setting up a web server and domain, and isn't exactly what we're looking for. A neat project though.



# Ideas for the Android App
Picture this: the app would support multiple "libraries", so you could use the app with a local library (just a folder with audio files), or connect to a "network library", which could be your other devices, or a public library exposed on the internet. This way, the app could be useful as a stand-alone music app, and also support all the syncing/streaming features if configured.



# Other Features
I've had a number of ideas for this; I'll try to quickly list a bunch now, in no particular order.
- playback of audio from multiple sources (Youtube, FLAC/WAV/MP3, video MP4, soundcloud, etc)
- karaoke-like lyric support (potentially using machine learning? at the very least, an interface for adding them in by hand)
- syncing playlists, playback history, playback queue, etc between devices
- listening parties?
- ability to customize playback, per track, per playlist/queue (volume adjustment, equalizing, gap-less playback, etc.)
- probably more to come, but I wanted to write these down now while I was thinking of them.

# Quick note on syncing
I know I want to support syncing data between devices, but I think spoopify will have a semi-manual syncing process. I plan on eventually making a larger "syncing" app for things like notes, todo-lists, passwords, and timers/alarms/calendar events, and that will probably be its own app, but it would be cool if there was a way for the two projects and apps to "integrate" so truly automatic and secure syncing is supported.

# Mix Tapes
I've read about how some people are nostalgic for the days of making "mix tapes" for their friends; I would like the provide a similar ability within this software project, without stepping on the RIAA's toes and getting shut down like napster or something. I like the idea of being able to create "mix tapes" that you can share to friends, or burn to a physical CD, for personal use, but not turning into a lime-wire like music piracy like haven.
