//! Inbound adapter: MCP (JSON-RPC over HTTP POST) for coding agents.
// shortcut: hand-rolled MCP (tools only, no SSE, no sessions); switch to the rmcp crate when resources/prompts/streaming are needed

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::application::ReviewService;
use crate::domain::{Block, Origin};

#[derive(Deserialize)]
struct PushArgs {
    session: String,
    goal: String,
    blocks: Vec<Block>,
    #[serde(flatten)]
    origin: Origin,
}

fn tool() -> Value {
    json!({
        "name": "push_review",
        "description": "Show the human reviewer what this coding session is doing, live in the Sidecar Review desktop app. \
Each session is one document; call this after every meaningful unit of work with the same `session` and the new blocks are appended in order. \
Send `agent`, `cwd` and `agent_session_id` with every call so the reviewer can tell where the review comes from. \
Send only the core: the few lines that carry the logic, trimmed of imports, boilerplate, formatting and unrelated context. \
No prose explanations — `goal` and `title` are one short line each, written in the user's language. \
Give every `diff` and `code` block a `mermaid` diagram that best explains that code (its flow, sequence, state or structure); leave it out only when the code is obvious without one. \
Block types: `diff` (file, before, after, mermaid), `code` (file, code, mermaid), `mermaid` (source), `html` (html: a self-contained chart or diagram, rendered in a sandboxed iframe).",
        "inputSchema": {
            "type": "object",
            "required": ["session", "goal", "blocks"],
            "properties": {
                "session": { "type": "string", "description": "Stable id of this coding session, e.g. the branch or task name." },
                "goal": { "type": "string", "description": "One line: what the session is trying to achieve." },
                "agent": { "type": "string", "description": "Name of the coding agent sending this, e.g. claude, opencode, codex." },
                "cwd": { "type": "string", "description": "Absolute path of the agent's working directory." },
                "agent_session_id": { "type": "string", "description": "The agent's own id for this conversation, when it knows one." },
                "blocks": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["type"],
                        "properties": {
                            "type": { "enum": ["diff", "code", "mermaid", "html"] },
                            "title": { "type": "string", "description": "One line: the function or behavior this block is about." },
                            "file": { "type": "string", "description": "diff, code: path of the file (picks syntax highlighting)." },
                            "before": { "type": "string", "description": "diff: core code before the change. Omit for new code." },
                            "after": { "type": "string", "description": "diff: core code after the change. Omit for deleted code." },
                            "code": { "type": "string", "description": "code: the snippet." },
                            "mermaid": { "type": "string", "description": "diff, code: Mermaid source of the diagram that best explains this code, shown above it. Omit only when the code is obvious without one." },
                            "source": { "type": "string", "description": "mermaid: diagram source." },
                            "html": { "type": "string", "description": "html: full HTML document or fragment." }
                        }
                    }
                }
            }
        }
    })
}

fn call(service: &ReviewService, params: &Value) -> Result<String, String> {
    if params["name"] != "push_review" {
        return Err(format!("unknown tool: {}", params["name"]));
    }
    let args: PushArgs = serde_json::from_value(params["arguments"].clone()).map_err(|e| e.to_string())?;
    let session = args.session.clone();
    let count = service.push(args.session, args.goal, args.blocks, args.origin);
    Ok(format!("ok: session '{session}' now shows {count} blocks"))
}

/// One JSON-RPC message in, one out. `None` for notifications.
fn rpc(service: &ReviewService, req: &Value) -> Option<Value> {
    let id = req.get("id")?;
    let result = match req["method"].as_str().unwrap_or("") {
        "initialize" => json!({
            "protocolVersion": req["params"]["protocolVersion"].as_str().unwrap_or("2025-06-18"),
            "capabilities": { "tools": {} },
            "serverInfo": { "name": "sidecar", "version": env!("CARGO_PKG_VERSION") }
        }),
        "ping" => json!({}),
        "tools/list" => json!({ "tools": [tool()] }),
        "tools/call" => {
            let (text, is_error) = match call(service, &req["params"]) {
                Ok(t) => (t, false),
                Err(t) => (t, true),
            };
            json!({ "content": [{ "type": "text", "text": text }], "isError": is_error })
        }
        m => {
            return Some(json!({ "jsonrpc": "2.0", "id": id, "error": { "code": -32601, "message": format!("method not found: {m}") } }));
        }
    };
    Some(json!({ "jsonrpc": "2.0", "id": id, "result": result }))
}

async fn mcp(State(service): State<Arc<ReviewService>>, headers: HeaderMap, Json(req): Json<Value>) -> Response {
    // browsers always send Origin on cross-site POSTs and agents never do: keeps web pages from writing here
    if headers.contains_key("origin") {
        return StatusCode::FORBIDDEN.into_response();
    }
    match rpc(&service, &req) {
        Some(res) => Json(res).into_response(),
        None => StatusCode::ACCEPTED.into_response(),
    }
}

pub fn router(service: Arc<ReviewService>) -> Router {
    Router::new().route("/mcp", post(mcp)).with_state(service)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;
    use crate::adapters::file_repo;
    use crate::application::DocumentNotifier;
    use crate::domain::Document;

    static NOTIFIED: AtomicUsize = AtomicUsize::new(0);
    struct Count;
    impl DocumentNotifier for Count {
        fn changed(&self, _: &[Document]) {
            NOTIFIED.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn push(service: &ReviewService, args: Value) -> Value {
        let req = json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "push_review", "arguments": args } });
        rpc(service, &req).unwrap()["result"].clone()
    }

    #[test]
    fn mcp_flow() {
        let service = ReviewService::new(Box::new(file_repo::temp("mcp")), Box::new(Count));
        let init = rpc(&service, &json!({ "id": 0, "method": "initialize", "params": { "protocolVersion": "2025-03-26" } })).unwrap();
        assert_eq!(init["result"]["protocolVersion"], "2025-03-26");
        assert!(rpc(&service, &json!({ "method": "notifications/initialized" })).is_none());
        assert_eq!(rpc(&service, &json!({ "id": 1, "method": "tools/list" })).unwrap()["result"]["tools"][0]["name"], "push_review");
        assert_eq!(rpc(&service, &json!({ "id": 2, "method": "nope" })).unwrap()["error"]["code"], -32601);

        let blocks = json!([
            { "type": "diff", "title": "t", "file": "a.rs", "after": "fn a() {}", "mermaid": "graph LR; a-->b" },
            { "type": "code", "file": "a.rs", "code": "fn a() {}" },
            { "type": "mermaid", "source": "graph LR; a-->b" },
            { "type": "html", "html": "<b>hi</b>" }
        ]);
        assert_eq!(push(&service, json!({ "session": "s1", "goal": "g1", "blocks": blocks, "agent": "claude", "cwd": "/repo" }))["isError"], false);
        push(&service, json!({ "session": "s2", "goal": "g", "blocks": [] }));
        push(&service, json!({ "session": "s1", "goal": "g2", "blocks": blocks }));
        // rejected input changes nothing and notifies nobody
        assert_eq!(push(&service, json!({ "session": "s1", "goal": "g", "blocks": [{ "type": "video" }] }))["isError"], true);
        assert_eq!(NOTIFIED.load(Ordering::SeqCst), 3);

        let docs = service.documents();
        assert_eq!(docs.len(), 2);
        let last = docs.last().unwrap();
        assert_eq!((last.session.as_str(), last.goal.as_str(), last.blocks.len()), ("s1", "g2", 8));
        // the second s1 push named no agent: the first one's answer stays
        assert_eq!((last.origin.agent.as_str(), last.origin.cwd.as_str(), last.origin.agent_session_id.as_str()), ("claude", "/repo", ""));
        assert!(last.created_at > 0 && last.updated_at >= last.created_at);
        // the diagram stays with its code block; blocks sent without one get an empty string
        let saved = serde_json::to_value(&last.blocks).unwrap();
        assert_eq!((&saved[0]["mermaid"], &saved[1]["mermaid"]), (&json!("graph LR; a-->b"), &json!("")));
    }
}
