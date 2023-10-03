const { invoke } = window.__TAURI__.tauri;

/*
let b_launch;
let l_launch_tx;

async function call_rust_backend() 
{
    l_launch_tx.textContent = await invoke("randommessage");
}
*/


window.addEventListener("DOMContentLoaded", () => {
    console.log('DOMContent was successfully loaded!');
    /*
    b_launch = document.querySelector("#b_open_in");
    l_launch_tx = document.querySelector("#l_launch_tx")
    b_launch.addEventListener('click', () =>
    {  
        console.log('Button was pressed!');
        call_rust_backend()
    });
    */
});