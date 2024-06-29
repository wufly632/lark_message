#[macro_use] extern crate rocket;

use std::collections::HashMap;
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::serde::json::serde_json::json;
use rocket::yansi::Paint;

#[derive(Deserialize, Debug)]
struct Task {
    pipelineId: String,
    pipelineName: String,
    stageName: String,
    taskName: String,
    buildNumber: String,
    statusCode: String,
    statusName: String,
    pipelineUrl: String,
    message: String,
    executorId: String,
    executorName: String,
    pipelineTags: Option<String>,
    pipelineEnvironment: Option<String>,
    flowInstId: String,
    pipelineInstId: String,
    pipelineMark: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SourceData {
    repo: String,
    branch: String,
    commitId: String,
    privousCommitId: Option<String>,
    commitMsg: String,
    args: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Source {
    name: String,
    sign: String,
    #[serde(rename = "type")]
    source_type: String,
    data: SourceData,
}

#[derive(Deserialize, Debug)]
struct GlobalParam {
    id: Option<String>,
    key: String,
    value: String,
}

#[derive(Deserialize, Debug)]
struct RequestBody {
    event: String,
    action: String,
    task: Task,
    pipeline: Option<String>,
    artifacts: Vec<String>,
    sources: Vec<Source>,
    globalParams: Vec<GlobalParam>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Response {
    complete: bool
}

#[post("/<env>", data = "<req>")]
async fn msg_lark(env: String, req: Json<RequestBody>) -> Json<Response> {
    let mut map = HashMap::new();
    map.insert(String::from("dev"), String::from("f541a977-6909-4bba-b7ee-caf9cc9661e9"));
    map.insert(String::from("daily"), String::from(""));
    map.insert(String::from("staging"), String::from(""));
    map.insert(String::from("prod"), String::from(""));
    let env_url = map.get(&env).expect("环境不存在");
    // 处理接收到的任务
    println!("Received req: {:?}", req);
    let task = req.into_inner().task;

    // 提取所需信息
    let pipeline_name = task.pipelineName;
    let status_name = task.statusName;
    let executor_name = task.executorName;

    // 转发到Lark自定义机器人
    let lark_webhook_url = "https://open.larksuite.com/open-apis/bot/v2/hook/".to_owned() + env_url.as_str();
    let message = json!({
                "msg_type": "text",
                "content": {
                    "text": format!("项目: {}, 状态: {}，操作人: {}", pipeline_name, status_name, executor_name)
                }
            });

    let client = reqwest::Client::new();
    let res = client.post(lark_webhook_url)
        .json(&message)
        .send()
        .await;

    match res {
        Ok(_) => // 返回处理后的任务
            Json(Response {
                complete: true,
            }),
        Err(e) => {
            println!("{}", e);
            Json(Response {
                complete: false,
            })
        },
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![msg_lark])
}