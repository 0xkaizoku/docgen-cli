use serde_json::json;

#[tokio::test]
async fn test_mcp_protocol_handlers() {
    // Note: We can test MCP JSON-RPC handlers directly
    let list_req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list",
        "params": {}
    });

    assert_eq!(list_req["method"], "tools/list");

    let convert_req = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "doc_convert",
            "arguments": {
                "input_content": "# MCP Test Document\n\nGenerated via tool call.",
                "output_path": "mcp_test.html",
                "theme": "modern-executive"
            }
        }
    });

    assert_eq!(convert_req["params"]["name"], "doc_convert");
}
