# Contributing Guide

<!--toc:start-->
- [Contributing Guide](#contributing-guide)
  - [Running the Dockers](#running-the-dockers)
    - [Services](#services)
    - [`.env` file](#env-file)
  - [Project structure](#project-structure)
<!--toc:end-->

## Running the Dockers

The project can be set up and used with docker compose. The compose file is
constructed in such a way that no other than the 
`docker compose build; docker compose` actions are needed to run the API. What
is however required is to fill a `.env` file which should be placed at the root
of the repository.

### Services

The compose file runs multiple services. Two of them are provided only to build
correctly other images, those are the `trackers-chef` and `trackers-planner` 
services. Along with the `migrator` service they are expected to successfully 
exit with __0__ code. The migrator prepares the database if neccessary. Left
running should be the __database__ and the __api__ server.
 
### `.env` file

The provided compose configuration expects a _.env_ file to be located at the
root of the repository. To fill in that file correctly check the environments
defined for the GitHub repository of the project. Access to those may require
the collaborator role within the GitHub repository.


## Project structure

The code is split into two crates:
- `trackers-models` - defines the schema used by the Diesel ORM as well as any
other models used by the API. The goal is to make it useful for anyone who
would want to provide a client for the API server written in Rust or any other
language that can use Rust source code.
- `trackers-api-server` - the actuall API server and the place defining the
business logic.