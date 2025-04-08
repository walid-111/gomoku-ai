use crate::*;
use ai_player::thread_handle;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskRequest {
	pub moves: Vec<PackedLocation>,
	pub depth: usize,
	pub beta: f32,
	pub alpha: f32,
}

pub type TaskResult = f32;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_name = threadSend)]
	async fn _thread_send(task: &str) -> JsValue;
}

#[wasm_bindgen(js_name = threadHandle)]
pub fn _thread_handle(request: &str) -> TaskResult {
	let task = serde_json::from_str(&request).unwrap();

	thread_handle(task)
}

pub async fn thread_send(task: TaskRequest) -> TaskResult {
	let request = serde_json::to_string(&task).unwrap();

	_thread_send(&request).await.as_f64().unwrap() as TaskResult
}
