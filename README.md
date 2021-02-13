# Invaders

Invaders is an open source terminal arcade game with audio, based off of the "Space Invaders" classic arcade game.

I made this while lerning rust from this [course](https://www.udemy.com/course/ultimate-rust-crash-course/)

### Dependencies on Linux

Audio should work out-of-the-box on macOS, Windows, and iOS.  For Linux, the
downstream package for actually _playing_ sound ([CPAL]) requires
the *Alsa* development libraries to be installed.

**CentOS**

```bash
sudo yum install -y alsa-lib-devel
```

**Debian/Ubuntu**

```bash
sudo apt install libasound2-dev pkg-config
```
