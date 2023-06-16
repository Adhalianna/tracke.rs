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
[Asserts]
jsonpath "$.data.checkmarked" == false

PUT http://0.0.0.0:4000/api/task/4/checkmark
Authorization: bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$.data.checkmarked" == true

POST http://0.0.0.0:4000/api/user/test@tracke.rs/tasks
Authorization: bearer {{token}}
{
    "tracker_id": "0",
    "checkmarked": true,
    "checkmarked_at": "2023-06-16T12:15:09.809941Z",
    "title": "Review test material",
    "time_estimate": "8h",
    "tags":
    [
      "chore"
    ],
    "list":
    [
      {
        "idx": 3,
        "item_content": "encryption algorithms",
        "checkmarked": false
      },
      {
        "idx": 2,
        "item_content": "protecting applications",
        "checkmarked": false
      },
      {
        "idx": 1,
        "item_content": "sql injections",
        "checkmarked": true
      }
    ]
}

HTTP 201
[Asserts]
jsonpath "$.data.list[0].idx" == 1
jsonpath "$.data.list[0].item_content" == "sql injections"

DELETE http://0.0.0.0:4000/api/task/4/checkmark
Authorization: bearer {{token}}

HTTP 200
