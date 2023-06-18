POST http://0.0.0.0:4000/api/session/token
[FormParams]
grant_type: password
password: password$123
username: test@tracke.rs

HTTP 200
[Captures]
token: jsonpath "$.access_token"

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

GET http://0.0.0.0:4000/api/user/test@tracke.rs/authorised_clients
Authorization: bearer {{token}}

HTTP 200

POST http://0.0.0.0:4000/api/user/test@tracke.rs/authorised_client/{{client_id}}
Authorization: bearer {{token}}

HTTP 200

POST http://0.0.0.0:4000/api/session/token
[FormParams]
grant_type: client_credentials
client_id: {{client_id}}
client_secret: {{client_secret}}

HTTP 200
[Captures]
client_token: jsonpath "$.access_token"

GET http://0.0.0.0:4000/api/task/1
Authorization: bearer {{client_token}}

HTTP 200

DELETE http://0.0.0.0:4000/api/user/test@tracke.rs/authorised_client/{{client_id}}
Authorization: bearer {{token}}

HTTP 200

POST http://0.0.0.0:4000/api/session/token
[FormParams]
grant_type: client_credentials
client_id: {{client_id}}
client_secret: {{client_secret}}

HTTP 400