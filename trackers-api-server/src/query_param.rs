use diesel::NullableExpressionMethods;

use crate::prelude::*;
extern crate serde_regex;

#[derive(Deserialize, JsonSchema, Debug, Clone)]
pub struct TasksQuery {
    /// Limits results to either completed (checkmarked) tasks if true or 
    /// uncompleted if false.
    pub checkmarked: Option<bool>,
    /// Array subscript (`[]`) must be used within the query string
    /// for the parameter to be parsed correctly. The subscript notation
    /// makes it easy to add new elements to the query string.
    ///
    /// ### Example
    ///
    /// The following displays the permissiveness of the query string parser:
    ///
    /// ```
    /// tasks?tag[0]=assignments&checkmarked=true&tag[2]=studies
    /// ```
    ///
    /// On the other hand, the following will cause the query string to be 
    /// ignored (because of missing `[]`):
    ///
    /// ```
    /// tasks?tag=assignments
    /// ```
    pub tags: Option<trackers_models::types::Tags>,
    /// Looks for a title which uses the given phrase as a substring
    pub title: Option<String>,
    /// Use regex to search for a pattern in title or description. Supports only very limited
    /// regex variant implemented for the [`SIMILAR TO`](https://www.postgresql.org/docs/current/functions-matching.html#FUNCTIONS-SIMILARTO-REGEXP)
    /// operation in PostgreSQL.
    pub regex: Option<RegexStr>,
}

impl TasksQuery {
    pub fn is_empty(&self) -> bool {
        self.checkmarked.is_none() && self.tags.is_none() && self.title.is_none() && self.regex.is_none()
    }
}

#[derive(Deserialize, Debug, Clone, diesel::expression::AsExpression,diesel::deserialize::FromSqlRow)]
#[serde(transparent)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub struct RegexStr(#[serde(deserialize_with = "serde_regex::deserialize")] pub regex::Regex);

impl JsonSchema for RegexStr {
    fn schema_name() -> String {
        "regex pattern".to_owned()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<String>()
    }
}

impl diesel::serialize::ToSql<diesel::sql_types::Text, diesel::pg::Pg> for RegexStr {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        use std::io::Write;
        out.write_fmt(format_args!("{}", self.0.to_string()))?;
        Ok(diesel::serialize::IsNull::No)
    }
}

type BoxedFilters = Box<
    dyn diesel::BoxableExpression<
        db_schema::tasks::table,
        diesel::pg::Pg,
        SqlType = diesel::sql_types::Bool,
    >,
>;

type BoxedFiltersJoin<OTHER> = Box<
    dyn diesel::BoxableExpression<
        diesel::dsl::InnerJoinQuerySource<OTHER, db_schema::tasks::table>,
        diesel::pg::Pg,
        SqlType = diesel::sql_types::Bool,
    >,
>;

// NOTE: The generics and all are super tangled in diesel. The following impl is very
// repetetive, it could be probably simplified with a macro but for sure not with a
// function.
impl TasksQuery {
    pub fn into_filters(&self) -> BoxedFilters {
        let mut boxed_filters: Option<BoxedFilters> = None;
        if let Some(checkmarked) = self.checkmarked {
            if checkmarked {
                boxed_filters = boxed_filters
                    .and_then(|fls| {
                        let part: BoxedFilters = Box::new(
                            fls.and(db_schema::tasks::columns::completed_at.is_not_null()),
                        );
                        Some(part)
                    })
                    .or_else(|| {
                        let part: BoxedFilters =
                            Box::new(db_schema::tasks::columns::completed_at.is_not_null());
                        Some(part)
                    });
            } else {
                boxed_filters = boxed_filters
                    .and_then(|fls| {
                        let part: BoxedFilters = Box::new(
                            fls.and(db_schema::tasks::columns::completed_at.is_not_null()),
                        );
                        Some(part)
                    })
                    .or_else(|| {
                        let part: BoxedFilters =
                            Box::new(db_schema::tasks::columns::completed_at.is_not_null());
                        Some(part)
                    });
            }
        }
        if let Some(tags) = self.tags.clone() {
            boxed_filters = boxed_filters
                    .and_then(|fls| {
                        let part: BoxedFilters = Box::new(
                            fls.and(db_schema::tasks::columns::tags.contains(tags.clone()).assume_not_null()),
                        );
                        Some(part)
                    })
                    .or_else(||  {
                        let part: BoxedFilters =
                            Box::new(db_schema::tasks::columns::tags.contains(tags).assume_not_null());
                        Some(part)
                    });
        }
        if let Some(title) = self.title.clone() {
            boxed_filters = boxed_filters
                    .and_then(|fls| {
                        let part: BoxedFilters = Box::new(
                            fls.and(db_schema::tasks::columns::title.ilike(format!("%{title}%"))),
                        );
                        Some(part)
                    })
                    .or_else(||  {
                        let part: BoxedFilters =
                            Box::new(db_schema::tasks::columns::title.ilike(format!("%{title}%")));
                        Some(part)
                    });
        }
        
        boxed_filters.unwrap_or_else(||
            // We shouldn't ever get here. If we got here it means either the exterior business logic
            // was implemented poorly or the implementation of this function did not consider all the
            // cases. Either way, it is simpler to return anything and avoid difficult to track panicks.
            {
                #[cfg(debug_assertions)]
                dbg!("We shouldn't be executing this part.");
                  
                Box::new(db_schema::tasks::task_id.is_not_null()
            )})
    }
    pub fn into_join_filters<OTHER>(&self) -> BoxedFiltersJoin<OTHER>
    where
        OTHER: diesel::QuerySource
            + diesel::query_source::TableNotEqual<db_schema::tasks::table>
            + diesel::JoinTo<db_schema::tasks::table>
            + 'static,
        db_schema::tasks::table: diesel::JoinTo<OTHER>,
    {
        let mut boxed_filters: Option<BoxedFiltersJoin<OTHER>> = None;
        if let Some(checkmarked) = self.checkmarked {
            if checkmarked {
                boxed_filters = boxed_filters
                    .and_then(|fls| {
                        let part: BoxedFiltersJoin<OTHER> = Box::new(
                            fls.and(db_schema::tasks::columns::completed_at.is_not_null()),
                        );
                        Some(part)
                    })
                    .or_else(|| {
                        let part: BoxedFiltersJoin<OTHER> =
                            Box::new(db_schema::tasks::columns::completed_at.is_not_null());
                        Some(part)
                    });
            } else {
                boxed_filters = boxed_filters
                    .and_then(|fls| {
                        let part: BoxedFiltersJoin<OTHER> =
                            Box::new(fls.and(db_schema::tasks::columns::completed_at.is_null()));
                        Some(part)
                    })
                    .or_else(|| {
                        let part: BoxedFiltersJoin<OTHER> =
                            Box::new(db_schema::tasks::columns::completed_at.is_null());
                        Some(part)
                    });
            }
        }
        if let Some(tags) = self.tags.clone() {
            boxed_filters = boxed_filters
                    .and_then(|fls| {
                        let part: BoxedFiltersJoin<OTHER> = Box::new(
                            fls.and(db_schema::tasks::columns::tags.contains(tags.clone()).assume_not_null()),
                        );
                        Some(part)
                    })
                    .or_else(||  {
                        let part: BoxedFiltersJoin<OTHER> =
                            Box::new(db_schema::tasks::columns::tags.contains(tags).assume_not_null());
                        Some(part)
                    });
        }
        if let Some(title) = self.title.clone() {
            boxed_filters = boxed_filters
                    .and_then(|fls| {
                        let part: BoxedFiltersJoin<OTHER> = Box::new(
                            fls.and(db_schema::tasks::columns::title.ilike(format!("%{title}%"))),
                        );
                        Some(part)
                    })
                    .or_else(||  {
                        let part: BoxedFiltersJoin<OTHER> =
                            Box::new(db_schema::tasks::columns::title.ilike(format!("%{title}%")));
                        Some(part)
                    });
        }
        if let Some(regex) = self.regex.clone() {
            boxed_filters = boxed_filters
                    .and_then(|fls| {
                        let part: BoxedFiltersJoin<OTHER> = Box::new(
                            fls.and(db_schema::tasks::columns::title.similar_to(regex.clone())).or(db_schema::tasks::columns::description.similar_to(regex.clone())).assume_not_null(),
                        );
                        Some(part)
                    })
                    .or_else(||  {
                        let part: BoxedFiltersJoin<OTHER> =
                            Box::new(db_schema::tasks::columns::title.similar_to(regex.clone()).or(db_schema::tasks::columns::description.similar_to(regex.clone())).assume_not_null());
                        Some(part)
                    });
        }
        boxed_filters.unwrap_or_else(|| 
            // We shouldn't ever get here. If we got here it means either the exterior business logic
            // was implemented poorly or the implementation of this function did not consider all the
            // cases. Either way, it is simpler to return anything and avoid difficult to track panicks.
            {
                #[cfg(debug_assertions)]
                dbg!("We shouldn't be executing this part.");
                  
                Box::new(db_schema::tasks::task_id.is_not_null()
            )})
    }
}
