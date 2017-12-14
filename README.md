# bind_spawn - It binds and it spawns. Surprise!

*STATUS: not functional*

Microservices all the way. Now you have plenty of services to run and you don't want to think about port mapping. Or tests are running in parallel and you want to avoid port collisions. This is where `bind_spawn` comes in.

## Usage

```
Usage: bind_spawn [options] -- <command> [args...]

Options:
--host ADDR - Select on which address to bind (default: 0.0.0.0)
--help      - Shows this help
```

Example:

```
$ port=$(bind_spawn -- bundle exec puma)
# at that point, bind_spawns has printed the dynamically associated port on stdout and forked the process into the background.
$ my_other_service --puma-addr=127.0.0.1:$port
```

## Requirements

To work, all the services need to implement the [systemd listen_fds protocol](https://www.freedesktop.org/software/systemd/man/sd_listen_fds.html#) to receice the bound port.

And your service graph musn't have any cycles.

