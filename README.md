# Oxide
*A decentralized, monorepo & polyglot friendly devtool from
the future*

The idea of oxide is to provide one centralized devtool for as many languages
and  ecosystems as possible, while focusing on being monorepo friendly.

## Project Structure
Oxide projects are intended to be laid out like this:
```
// An example project with a web server and a client interface
[example]
|-oxide.toml
|-[server]
| |-oxide.toml
| |-build.gradle.kts
| |-settings.gradle.kts
|
|-[web]
  |-oxide.toml
  |-package.json
  |-requirements.txt
```

## Commands

### Sync

When you run `oxide sync` in any of the [project]s the client will look up the
tree to the top level of the workspace, in this case: `[example]`, and it will
send a signal to the daemon process to resynchronize its project graph to match
the configurations stored in the `oxide.toml` files.

### TO-DO

- [ ] Installation packages
    - [ ] NPM
    - [ ] Cargo
      - [ ] Cargo extension?
    - [ ] RubyGems
    - [ ] MacOS
        - [ ] .pkg
        - [ ] Homebrew
    - [ ] Windows (exe, msi)
    - [ ] Linux
        - [ ] AppImage
        - [ ] Snap
        - [ ] Flatpak
        - [ ] Distro packages

#### Homeserver
- [ ] Decentralize with OSP
- [ ] Package Management
  - [ ] Package registries
- [ ] Web dashboard
  - [ ] Audit log
  - [ ] Hosted admin UI

#### CLI & System Daemon
- [ ] Project init
- [ ] Build
- [ ] Local package registries
- [ ] Audit log
  - [ ] Sqlite storage
  - [ ] Hosted admin UI
- [ ] Configuration
  - [ ] Daemon
    - [x] Local repo enable/disable
        

#### Language & Integration Features
- [ ] Node / JS
- [ ] Python
  - [ ] Venv management
- [ ] JVM
  - [ ] Gradle
  - [ ] Maven
  - [ ] Eclipse
  - [ ] Ant

*This list isn't exhaustive!*

## Licensing
This project is currently licensed under the [Apache 2.0 License](LICENSE).