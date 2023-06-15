POST http://0.0.0.0:4000/api/session/token
[FormParams]
grant_type: password
password: password$123
username: test@tracke.rs

HTTP 200
[Captures]
token: jsonpath "$.access_token"

GET http://0.0.0.0:4000/api/task/0
Authorization: bearer {{token}}

HTTP 200

GET http://0.0.0.0:4000/api/task/1
Authorization: bearer {{token}}

HTTP 200

GET http://0.0.0.0:4000/api/task/2
Authorization: bearer {{token}}

HTTP 200

DELETE http://0.0.0.0:4000/api/task/4/checkmark
Authorization: bearer {{token}}

HTTP 200

PUT http://0.0.0.0:4000/api/task/4/checkmark
Authorization: bearer {{token}}

HTTP 200

