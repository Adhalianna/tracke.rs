# Project Goal

## Project Topic

__GitHab__ is a habit tracker web application. With GitHab users can store their habit data
on their accounts and access it either through the application or Discord webhooks.

# Developement Guide

Each part, back-end and front-end, may use different tools but the following instructions
apply to both.

## Required Tools 

### Docker

The project uses docker to make it simpler to work on it while using different
OSes as a developer. To install docker one should follow the guide applicable
to a given operating system and distribution.
 - [Windows installation guide](https://docs.docker.com/desktop/install/windows-install/)
 - [RPM or Debian based Linux distributions](https://docs.docker.com/desktop/install/linux-install/)
 - [ArchLinux based distributions](https://docs.docker.com/desktop/install/archlinux/)

Besides docker we will be using an extension to it called _docker compose_.
It's a tool which can parse a special configuration file usually called 
`docker-compose.yml` and run multiple containers using the file as a recipe for
their parameters. [To use _docker compose_ on Windows or
Mac a desktop application is required](https://docs.docker.com/compose/install/)
or the WSL.

## Using the Tools

### Launching the database

To turn the database up execute:
```sh
sudo docker compose up database
```
The database has it's ports exposed so it can be reached from the host using:
```sh
psql -d githab -h 0.0.0.0 -p 5432 -U githab
```
The database may be empty when launched which can cause errors in the back-end
application. In such a case it is recommended to run the migrator docker:
```sh
sudo docker compose up migrator
```
If the migrator docker turns out to be too slow (it should take a while to
build only at the first run) the original cli application for managing
migrations can be installed. Check the [back-end's README](./back-end/README.md)
for more details.

### Launching back-end server

To run back-end server execute:
```sh
sudo docker compose up back-end
```
If you have the Rust toolchain available locally and want to iterate quickly
using just `cargo` to compile the project is recommended.
```sh
cd back-end
cargo dev
```
More information can be found in [the back-end's README](./back-end/README.md).

### Launching all the components
It is a good idea to check occasionally if all the dockers can communicate with
each other as expected. The website they are hosting should be available from
the host. To turn on all the services execute:
```sh
sudo docker compose up
```