const { invoke } = window.__TAURI__.tauri;

async function preload()
{
    let userDir = await invoke("get_user_dir");
    let setup_done = await invoke("dir_exists");
    console.log(userDir);
    console.log(setup_done);

}

preload();