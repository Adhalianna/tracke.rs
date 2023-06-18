# Ask for access token using password as a user
POST http://0.0.0.0:4000/api/session/token
[FormParams]
grant_type: password
password: password$123
username: test@tracke.rs

HTTP 200
[Captures]
token: jsonpath "$.access_token"

# Once authorised create new client credentials
POST http://0.0.0.0:4000/api/user/test@tracke.rs/authorised_clients
Authorization: bearer {{token}}
{
    "user_id": "0",
    "name": "test client",
    "website": "dummy" 
}

HTTP 201
[Captures]
client_id: jsonpath "$.data.client_id"
client_secret: jsonpath "$.data.client_secret"

# Check the list of all authorised clients
GET http://0.0.0.0:4000/api/user/test@tracke.rs/authorised_clients
Authorization: bearer {{token}}

HTTP 200

# Use POST to get full details about one of the authorised clients.
# POST is used to prevent caching accidentially the query results by a browser or
# other tool.
POST http://0.0.0.0:4000/api/user/test@tracke.rs/authorised_client/{{client_id}}
Authorization: bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$.data.client_id" == {{client_id}}
jsonpath "$.data.client_secret" == {{client_secret}}

# Use the credentials to get a token
POST http://0.0.0.0:4000/api/session/token
[FormParams]
grant_type: client_credentials
client_id: {{client_id}}
client_secret: {{client_secret}}

HTTP 200
[Captures]
client_token: jsonpath "$.access_token"

# Use the token issued to a client to query something
GET http://0.0.0.0:4000/api/task/1
Authorization: bearer {{client_token}}

HTTP 200

# Remove as a user an authorised client
DELETE http://0.0.0.0:4000/api/user/test@tracke.rs/authorised_client/{{client_id}}
Authorization: bearer {{token}}

HTTP 200

# The client can no longer get an access token
POST http://0.0.0.0:4000/api/session/token
[FormParams]
grant_type: client_credentials
client_id: {{client_id}}
client_secret: {{client_secret}}

HTTP 400