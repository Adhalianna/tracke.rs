//! Definitions of authorization scopes and extractors checking them

use axum::async_trait;
    
    #[derive(Debug)]
    pub struct UserIdScope(pub trackers_models::types::Uuid);
    impl std::str::FromStr for UserIdScope {
        type Err = base62::DecodeError;
    
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let num = base62::decode(s)?;
            let uuid = uuid::Uuid::from_u128(num);
            Ok(Self(trackers_models::types::Uuid::from(uuid)))
        }
    }
    
    impl std::fmt::Display for UserIdScope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    
    impl ScopeVariable<super::UserClaims> for UserIdScope {
        type Variable = Self;
        fn scope_name() -> &'static str {
            "user_resources"
        }
    }
    

    /// Allows creating extractors which knowing the path of the accessed resource can check
    /// for scopes applying to that specific resource.
    ///
    /// Use the trait in combination with the [`PathAndScope`] extractor.
    ///
    /// `P` is used as the generic `T` type within [`axum::extract::Path`].
    pub trait PathBasedPolicy<C: aliri::jwt::CoreClaims + aliri_oauth2::oauth2::HasScope> {
        type Path: Send + serde::de::DeserializeOwned;
        /// Select which part of the path is used as a variable component of the scope
        fn path_variable(path: &Self::Path) -> &str;
        /// Set the part of the scope name which is not variable
        fn scope_name() -> &'static str;

        /// Knowing the variable component value produce a scope that can be
        /// put within the claims
        ///
        /// # Panics
        /// __The default implementation panics.__ The panic happens when for some
        /// reason the string composed from the provided variable and
        /// [`scope_name`](Self::scope_name) cannot be used as a scope token.
        fn produce_token_for(var: &str) -> aliri_oauth2::oauth2::ScopeToken {
            aliri_oauth2::oauth2::ScopeToken::from_string(Self::scope_name().to_owned() + ":" + var)
                .unwrap()
        }
    }

    /// An extractor checking the [`PathBasedPolicy`] and extracting the path parameter.
    pub struct PathAndScope<
        T: PathBasedPolicy<C>,
        C: aliri::jwt::CoreClaims + aliri_oauth2::oauth2::HasScope,
    >(pub T::Path);

    #[derive(Debug)]
    pub enum PathAndScopeRejection {
        Scope(aliri_axum::AuthFailed),
        Path(axum::extract::rejection::PathRejection),
    }

    impl axum::response::IntoResponse for PathAndScopeRejection {
        fn into_response(self) -> axum::response::Response {
            match self {
                PathAndScopeRejection::Scope(s) => s.into_response(),
                PathAndScopeRejection::Path(p) => p.into_response(),
            }
        }
    }

    #[async_trait]
    impl<T, S, C> axum::extract::FromRequestParts<S> for PathAndScope<T, C>
    where
        T: PathBasedPolicy<C>,
        S: Send + Sync,
        C: aliri::jwt::CoreClaims
            + aliri_oauth2::oauth2::HasScope
            + axum::extract::FromRequestParts<S, Rejection = aliri_axum::AuthFailed>
            + Send
            + Sync,
    {
        type Rejection = PathAndScopeRejection;

        async fn from_request_parts(
            parts: &mut axum::http::request::Parts,
            state: &S,
        ) -> Result<Self, Self::Rejection> {
            let axum::extract::Path(path) =
                axum::extract::Path::<T::Path>::from_request_parts(parts, state)
                    .await
                    .map_err(|err| PathAndScopeRejection::Path(err))?;

            let claims = match C::from_request_parts(parts, state).await {
                Ok(claims) => claims,
                Err(err) => return Err(PathAndScopeRejection::Scope(err)),
            };

            if claims
                .scope()
                .iter()
                .any(|s| s == &T::produce_token_for(T::path_variable(&path)))
            {
                Ok(PathAndScope(path))
            } else {
                Err(PathAndScopeRejection::Scope(
                    aliri_axum::AuthFailed::InsufficientScopes { policy: None },
                ))
            }
        }
    }

    pub trait ScopeVariable<C: aliri::jwt::CoreClaims + aliri_oauth2::oauth2::HasScope> {
        type Variable: Send + std::str::FromStr;
        fn scope_name() -> &'static str;
    }

    pub struct VariableScope<
        V: ScopeVariable<C>,
        C: aliri::jwt::CoreClaims + aliri_oauth2::oauth2::HasScope,
    >(pub V::Variable);
    
    impl<V,C> aide::OperationInput for VariableScope<V,C> where V: ScopeVariable<C>, C: aliri::jwt::CoreClaims + aliri_oauth2::oauth2::HasScope {
    }
    
    #[async_trait]
    impl<V, S, C> axum::extract::FromRequestParts<S> for VariableScope<V, C>
    where
        V: ScopeVariable<C>,
        S: Send + Sync,
        C: aliri::jwt::CoreClaims
            + aliri_oauth2::oauth2::HasScope
            + axum::extract::FromRequestParts<S, Rejection = aliri_axum::AuthFailed>
            + Send
            + Sync + Clone,
    {
        type Rejection = aliri_axum::AuthFailed;

        async fn from_request_parts(
            parts: &mut axum::http::request::Parts,
            state: &S,
        ) -> Result<Self, Self::Rejection> {
            use std::str::FromStr;

            let claims = match C::from_request_parts(parts, state).await {
                Ok(claims) => claims,
                Err(err) => return Err(err),
            };

            if let Some(scope) = claims
                .clone()
                .scope()
                .iter()
                .find(|s| s.as_str().starts_with(V::scope_name()))
            {
                let Ok(var) = 
                    scope
                        .as_str()
                        .to_owned()
                        .strip_prefix(&(V::scope_name().to_owned() + ":"))
                        .ok_or(aliri_axum::AuthFailed::MissingClaims)
                        .and_then(|v| {
                            V::Variable::from_str(v).map_err(|_| aliri_axum::AuthFailed::MissingClaims)
                        }) else {
                    return Err(aliri_axum::AuthFailed::MissingClaims);
                };
                Ok(Self(var))
            } else {
                Err(
                    aliri_axum::AuthFailed::InsufficientScopes { policy: None },
                )
            }
        }
    }