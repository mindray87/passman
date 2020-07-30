<h1 align="center">
    PASSMAN
</h1>

<p align="center">
A simple password manager for your terminal.
</p>

[![Build Status](https://travis-ci.com/mindray87/pwd-man.svg?token=ziwkzZeesRqGqDpdiqQf&branch=master)](https://travis-ci.com/mindray87/pwd-man)

## TODO:

### CLI
- [x] parse arguments
- [x] open connection to daemon
- [x] send messages to daemon
- [x] clipboard action
- [x] password generator
- [ ] encrypt socket connection

### Daemon  
- [x] open and save password file
- [x] encrypt and decrypt password file
- [x] create password file
- [x] write, read, and delete passwords
- [x] receive messages
- [x] send messages
- [x] accept only messages form localhost
- [ ] auto close password file

## Build and Install
```shell script
# install dependencies
sudo apt-get install -y libx11-xcb-dev libxcb-render-util0-dev libxcb-shape0-dev libxcb-xfixes0-dev

# build the project
cargo build

# install the daemon
sudo cp ~/passman/src/daemon/passman.service /etc/systemd/system/
sudo sed -i "s/<<USER>>/$USER/g" /etc/systemd/system/passman.service
sudo systemctl start passman

# run the cli
TODO: Add Passman to PATH
```

## Build Documentation
#### build: 
```shell script
cargo doc --open --no-deps --bin passman-cli --bin passman-daemon 
```