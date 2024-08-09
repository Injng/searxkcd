# SearXKCD

An SDL2 application to search XKCD comics with not only the name, but also the content of the comic.

## Dependencies
Make sure that libsdl2 is installed, along with sdl2_image and sdl2_ttf.

Note: main repository is developed using Mercurial, at [https://hg.sr.ht/~lnjng/searxkcd](https://hg.sr.ht/~lnjng/searxkcd).

## Development
Currently the application is very slow, especially when it comes to rendering text. Future work should be done to
optimize the rendering process and possibly cache previously rendered textures to speed up the process.
