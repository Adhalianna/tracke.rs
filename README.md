# Project Goal

## Project Topic

__GitHab__ is a habit tracker web application. With GitHab users can store their habit data
on their accounts and access it either through the application or Discord webhooks.

# Developement Guide

## Required Tools 

### Docker

The project uses docker to make it simpler to work on it while using different OSes as a developer.
To install docker one should follow the guide applicable to a given operating system and
distribution.
 - [Windows installation guide](https://docs.docker.com/desktop/install/windows-install/)
 - [RPM or Debian based Linux distributions](https://docs.docker.com/desktop/install/linux-install/)
 - [ArchLinux based distributions](https://docs.docker.com/desktop/install/archlinux/)

Besides docker we will be using an extension to it called _docker compose_. It's a tool which can
parse a special configuration file usually called `docker-compose.yml` and run multiple containers
using the file as a recipe for their parameters and origin. [To use _docker compose_ on Windows or
Mac the desktop application is required](https://docs.docker.com/compose/install/) or the WSL.

Having all of that installed neither front-end or back-end teams should worry about getting any
other teams tools of trade - all the build (compilation, launching, etc) processes can happen in
docker containers.

## Using the Tools

### Docker Compose in Terminal

_TODO, maybe not neccessary_