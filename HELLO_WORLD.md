# Working dummy version of back-end

The back-end can already respond to an extremly simple request and it even
fetches some data from the database to do so. It also displays documentation
for its own endpoints using Swagger.

To run it:
```sh
sudo docker compose up
```

Then access `http://0.0.0.0:4000/api/hello` from your browser of choice.
The documentation is available under `http://0.0.0.0:4000/api/doc`.