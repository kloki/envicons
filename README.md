![Example](./example.png)

# Envicons

I use this to extend the prompt provided by [startship](https://starship.rs/).
It checks if some envars and/or docker containers are running and returns it in a succinct way using colored icons.

This probably only makes sense to me.

## Install and configure

git clone this repo and run:

```bash
cargo install --path <path-of-repo>
```

Add this to your `startship.toml`.

```toml
[custom.envicons]
when = true
command = "envicons"

```

For the icons install [nerdfonts](https://www.nerdfonts.com/).
