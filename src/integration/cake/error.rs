use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use crate::engine::{Decision, DecisionKind};

#[derive(Debug)]
pub struct CakeError {
    pub status: StatusCode,
    pub action: String,
    pub decision: Decision,
}

impl CakeError {
    pub fn forbidden(action: &str, decision: Decision) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            action: action.to_string(),
            decision,
        }
    }
}

#[derive(Serialize)]
struct ErrorBody {
    authorization: AuthorizationBody,
}

#[derive(Serialize)]
struct AuthorizationBody {
    action: String,
    decision: String,
    selected_rule: Option<String>,
    trace: Vec<String>,
}

impl IntoResponse for CakeError {
    fn into_response(self) -> Response {
        // English comment: Move Decision out entirely to avoid partial-move borrow issues.
        let Decision {
            kind,
            selected_rule,
            trace,
        } = self.decision;

        let decision = match kind {
            DecisionKind::Permit => "permit",
            DecisionKind::Deny => "deny",
        }
        .to_string();

        let trace = trace.into_iter().map(|e| e.to_string()).collect::<Vec<_>>();

        let body = ErrorBody {
            authorization: AuthorizationBody {
                action: self.action,
                decision,
                selected_rule,
                trace,
            },
        };

        (self.status, axum::Json(body)).into_response()
    }
}
