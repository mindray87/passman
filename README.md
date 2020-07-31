<h1 align="center">
    PASSMAN
</h1>

<p align="center">
A simple password manager for your terminal.
</p>

[![Build Status](https://travis-ci.com/mindray87/pwd-man.svg?token=ziwkzZeesRqGqDpdiqQf&branch=master)](https://travis-ci.com/mindray87/pwd-man)

## Features:

* Remembers your passwords
* Saves your passwords in an aes encrypted file in your home directory
* Print your passwords in the console
* Copy your password into the clipboard
* Clears your password from the clipboard after 30 seconds

## Build and Install (on Ubuntu)
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
```shell script
cargo doc --open --no-deps --bin passman-cli --bin passman-daemon 
```