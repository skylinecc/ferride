*Ferride is not even near working*


<p align="center"><img width="200" src="./src/resources/images/ferris_ferride.svg"></img></p>
<h2 align="center">Ferride</h2>
<p align="center">A Simple, Cross Platform IDE for Rust.</p>

<p align="center">
<a href="https://discord.gg/krPcwnu3Yv"><img src="https://img.shields.io/discord/621844549710381068?logo=discord"></a>
<a href="./LICENSE"><img src="https://img.shields.io/badge/license-GPL-blue.svg"></a>
<img src="https://img.shields.io/github/languages/code-size/skylinecc/ferride">
</p>

<p align="center"><img src="./docs/greeter.png"></p>
<p align="center"><i>The Welcome Window</i></p>


## Contributions
If you find this project interesting and want to contribute, pull requests and issues are greatly appreciated, I'm trying to make this as fast and efficient as possible.

## Planned Core Features (Work in Progress To-Do List)
- [ ] Customizable GTK Interface (Want it to look more legacy, menu bar, etc, no GNOME headerbar type thing so we can use it more natively on Windows and Macos (ew)).
- [ ] Sections for a terminal, text editor, file manager, and a file overview (structs, functions, traits, enums, etc).
- [ ] Integrated terminal based on Alacritty (Create our own [alacritty-gobject](https://github.com/DefunctLizard/alacritty-gobject) then have easy settings in our applicaiton preferences window).
- [ ] Integrated cargo project preferences into our window so things like name and description can be set from the GUI menu.
- [ ] Connect to crates.io API so developers can search crates.io from within ferride to search and add new dependencies easier.
- [ ] Integrated Rust Language completion in GtkSourceView from the rust-analyzer Rust API.
- [ ] Configuration files in users $HOME for remembering settings as well as project directories.
- [ ] Integration with Cargo and Cargo subcommands so ferride has buttons for running, building, and testing examples, targets, and tests.
- [ ] Allow cloning git repositories and then super simple "add, commit, push" system.
- [ ] Preview HTML & MD files directly in ferride with live updating using `libwebkitgtk`.
- [ ] Preview SVG & PNG/JPG (etc etc of course) images directly in ferride using `gdk_pixbuf`
- [ ] Custom GTK stylesheets for themes like solarized and monokai like VSCode. (contrary to https://stopthemingmy.app/?)
- [X] Our own little ferris icon, make it super cute with a hard hat and a hammer or something :)
- [ ] Use github and gitlab API to allow user to search up a repository before cloning it.
- [ ] Maybe link to gitoxide instead of git because gitoxide is written in pure rust meaning that the user won't have to have libgit (gitoxide is still under development, it can't clone or push yet :/)
- [ ] Have ability to build and view documentation for all our dependencies directly in ferride using `libwebkitgtk`.

### Future References
A couple of crates we might want to use in the future.
- https://crates.io/crates/github
- https://crates.io/crates/gitlab
- https://crates.io/crates/git2 (https://github.com/Byron/gitoxide)
- https://crates.io/crates/sourceview4 (https://gitlab.gnome.org/World/Rust/sourceview5-rs)
- https://crates.io/crates/webkit2gtk
- https://crates.io/crates/crates-io (https://crates.io/crates/libcratesio)
- https://github.com/rust-analyzer/rust-analyzer/tree/master/crates/ide
- https://crates.io/crates/markdown
- https://crates.io/crates/dirs
- https://crates.io/crates/serde_yaml



