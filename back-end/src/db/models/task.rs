use crate::prelude::*;

#[derive(Queryable, Insertable, Debug, Serialize, JsonSchema)]
#[diesel(table_name = crate::db::schema::tasks)]
pub struct Task {
    pub task_id: Uuid,
    pub user_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub time_estimate: Option<Duration>,
    pub soft_deadline: Option<chrono::NaiveDateTime>,
    pub hard_deadline: Option<chrono::NaiveDateTime>,
    pub tags: Option<Vec<Option<String>>>,
}
