POST http://0.0.0.0:4000/api/session/token
[FormParams]
grant_type: password
password: password$123
username: test@tracke.rs

HTTP 200
[Captures]
token: jsonpath "$.access_token"

POST http://0.0.0.0:4000/api/user/test@tracke.rs/tasks
Authorization: bearer {{token}}
{
    "checkmarked": true,
    "title": "Review test material",
    "time_estimate": "8h",
    "list":
    [
      {
        "idx": 3,
        "item_content": "encryption algorithms",
        "checkmarked": false
      },
      {
        "idx": 1,
        "item_content": "protecting applications",
        "checkmarked": false
      },
      {
        "idx": 2,
        "item_content": "sql injections",
        "checkmarked": true
      }
    ]
}

HTTP 201
[Captures]
task_id: jsonpath "$.data.task_id"

GET http://0.0.0.0:4000/api/task/{{task_id}}/list
Authorization: bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$.data[0].idx" == 1
jsonpath "$.data[0].item_content" == "protecting applications"

GET http://0.0.0.0:4000/api/task/{{task_id}}/list/item/3
Authorization: bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$.data.item_content" == "encryption algorithms"


DELETE http://0.0.0.0:4000/api/task/{{task_id}}/list/item/3
Authorization: bearer {{token}}

HTTP 200

GET http://0.0.0.0:4000/api/task/{{task_id}}/list/item/3
Authorization: bearer {{token}}

HTTP 404

POST http://0.0.0.0:4000/api/task/{{task_id}}/list/items
Authorization: bearer {{token}}
{
    "item_content": "hello",
    "checkmarked": false
}

HTTP 200

GET http://0.0.0.0:4000/api/task/{{task_id}}/list/item/3
Authorization: bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$.data.item_content" == "hello"

PUT http://0.0.0.0:4000/api/task/{{task_id}}/list/item/3/checkmark
Authorization: bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$.data.checkmarked" == true

DELETE http://0.0.0.0:4000/api/task/{{task_id}}/list/item/3/checkmark
Authorization: bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$.data.checkmarked" == false