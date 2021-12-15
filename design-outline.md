# A rough outline of the design
There will be a "backend" component that manages the music library and syncing playback between clients, and a "frontend" component that drives the GUI for desktop and mobile. The Qt framework will be used for the desktop frontend.

The primary use case is this: configure a library of music tracks, customizing organization/layout, metadata, cover art, playback, etc, and sync it across multiple devices.



# Are there similar projects out there?
Some quick searching led me to a project called "funkwhale". It's more a federated streaming platform, than a personal music library organizer; it requires setting up a web server and domain, and isn't exactly what we're looking for. A neat project though.



# Ideas for the Android App
Picture this: the app would support multiple "libraries", so you could use the app with a local library (just a folder with audio files), or connect to a "network library", which could be your other devices, or a public library exposed on the internet. This way, the app could be useful as a stand-alone music app, and also support all the syncing/streaming features if configured.
