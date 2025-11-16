# Backtune
Backtune is a work in progress app that adds a background soundtrack to your daily life. Ever imagined what it would feel like if random themes would start playing while you're doing something in real life? Well, you'd need to walk in
headphones 24/7 to experience that, but hey, at least this app plays a random theme every now and then while you're on your computer, which is already where many people spend most of their life! (which, just saying, isn't actually good for you)

# This is a Rust learning project,
so expect the code to *not* be of top quality. I tried to not use any unsafe code blocks, though, so it should be memory safe.

The project also used some AI.

# Usage
Currently the app has no UI, so you will have to configure it yourself.
Run the app, then go to your operating system's base directory (e.g. `AppData` on Windows, `.local/share` on Linux), and there you will find the Backtune folder. Inside it will be the sounds folder (where you have to put your sounds) and a
settings.json file containing the minimum and maximum wait times between songs. After that, restart the app and wait for your background soundtrack to start playing.
### All of the sounds have to be in the `.wav` format.

# Planned features:
- UI for in-app configuration,
- Configurable volume,
- Configurable fade in/out for the sounds,
- Run audio playing as a separate background process that doesn't require the app window,
- Spotify Desktop Client integration for playing Spotify songs instead of local files **(maybe in the far future)**.
