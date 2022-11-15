const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let resEl;

async function postData(){
const response = await fetch('https://reqres.in/api/users', {method: 'POST', body: {name: "fSFD", job:"ADaDs"}});
const data = await response.json().then((value)=>{
  invoke("do_sql")
  console.log(value);
  resEl.textContent = value.createdAt;
}) ;


}

async function greet() {
  postData();
  invoke("serve");

  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  resEl = document.querySelector("#msg");

  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#greet-button")
    .addEventListener("click", () => greet());
});
