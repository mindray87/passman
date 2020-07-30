<h1 align="center">
    PASSMAN
</h1>

<p align="center">
A simple password manager for your terminal.
<br>
[![Build Status](https://travis-ci.com/mindray87/pwd-man.svg?token=ziwkzZeesRqGqDpdiqQf&branch=master)](https://travis-ci.com/mindray87/pwd-man)
</p>

## TODO:

### CLI
- [ ] parse arguments
- [ ] open connection to daemon
- [ ] send messages to daemon
- [ ] clipboard action
- [ ] password generator
- [ ] encrypt socket connection

### Daemon  
- [ ] open and save password file
- [ ] encrypt and decrypt password file
- [ ] create password file
- [ ] write, read, and delete passwords
- [ ] receive messages
- [ ] send messages
- [ ] accept only messages form localhost
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
TODO
```