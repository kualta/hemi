HemiTyper is a typing trainer that provides you with only half the keyboard per session, to help you improve coordination of each hand separately.

Written in pure Rust using [Dioxus](https://github.com/dioxuslabs/dioxus). 

Supported layouts:
 - Qwerty
 - Colemak
   
Available at <b> [hemi.kualta.dev](https://hemi.kualta.dev/) </b> 

<img src="https://github.com/kualta/hemi/assets/72769566/f554af58-2386-4243-9f07-cac3c82fd014" width="600" align="center">

### Building
use [Dioxus CLI](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli) to build and serve locally:
```sh
cargo install dioxus-cli
```
to build run
```sh
dx build
```
to serve on localhost run
```sh
dx serve --hot-reload
```
to compile tailwind classes run
```sh
npx tailwindcss -i ./assets/style.css -o ./assets/tailwind.css
```

### Contributing 
The tool is considered complete, but PRs are still welcome.
 
If you wish to add another layout or change dictionary for an existing one, check `assets/words.json` and `src/words.rs`.

### License
Hemi is licensed under **GNU General Public License v3.0**, check [license](LICENSE) for more details.
