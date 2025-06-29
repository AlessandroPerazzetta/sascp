use std::process::Command;
// use std::collections::HashMap;
use indexmap::IndexMap;
use serde::Deserialize;
use tokio::sync::OnceCell;

use axum::{
    routing::{get, post},
    extract::Form,
    response::Html,
    Router,
};

static COMMANDS: OnceCell<IndexMap<&'static str, (&'static str, &'static str)>> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    let mut cmds = IndexMap::new();

    cmds.insert("date", ("Show Date", "🕒"));
    cmds.insert("whoami", ("Who Am I", "👤"));
    cmds.insert("uptime", ("System Uptime", "⏳"));
    cmds.insert("find /media/ -type f -iname *.mp4", ("List all MP4 Files under media", "🎬"));    
    cmds.insert("df -h", ("Disk Usage", "💾"));
    cmds.insert("free -h", ("Memory Usage", "🧠"));
    cmds.insert("top -b -n 1", ("System Processes", "📊"));
    cmds.insert("ifconfig", ("Show Network Info", "🖧"));
    cmds.insert("ifstat -p", ("Show Network Usage", "🌐"));
    cmds.insert("shutdown -r now", ("Reboot System", "🔄"));
    cmds.insert("shutdown -h now", ("Shutdown System Now", "⏻"));
    COMMANDS.set(cmds).unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/run", post(run_command));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:16661").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    let cmds = COMMANDS.get().unwrap();
    let mut html = String::from("<h1>Command Center</h1><form action=\"/run\" method=\"post\">");
    for (cmd, (desc, icon)) in cmds {
        html.push_str(&format!(
            "<button type=\"submit\" name=\"cmd\" value=\"{}\" style=\"font-size:2em;margin:10px\">{}<br><span style=\"font-size:0.5em\">{}</span></button>",
            cmd, icon, desc
        ));
    }
    html.push_str("</form>");
    Html(html)
}

#[derive(Deserialize)]
struct CmdForm {
    cmd: String,
}

async fn run_command(Form(input): Form<CmdForm>) -> Html<String> {
    let cmds = COMMANDS.get().unwrap();
    if !cmds.contains_key(input.cmd.as_str()) {
        return Html(format!("<p>Unknown command</p><a href=\"/\">Back</a>"));
    }
    let mut parts = input.cmd.split_whitespace();
    let cmd = parts.next().unwrap();
    let args: Vec<&str> = parts.collect();
    let output = Command::new(cmd)
        .args(&args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_else(|_| "Failed to execute".to_string());
    Html(format!(
        "<pre>{}</pre><a href=\"/\">Back</a>",
        html_escape::encode_text(&output)
    ))
}